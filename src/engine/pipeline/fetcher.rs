use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::collections::HashMap;

use iced::widget::image::Handle;

use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::Stylesheet;
use crate::engine::net;
use crate::engine::parser::Parser;
use crate::engine::js::{JsBridge, JSEngine};
use crate::plog;

use super::extractor::{extract_elements, StyledElement};
use super::layout::apply_caelum_layout;

static CSS_CACHE: OnceLock<RwLock<HashMap<String, Stylesheet>>> = OnceLock::new();
fn css_cache() -> &'static RwLock<HashMap<String, Stylesheet>> {
    CSS_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn extract_styles(node: &Node, styles: &mut Vec<String>) {
    if let NodeType::Element(elem) = &node.node_type {
        if elem.tag_name.to_lowercase() == "style" {
            for child in &node.children {
                if let NodeType::Text(text) = &child.node_type {
                    styles.push(text.clone());
                }
            }
        }
        for child in &node.children {
            extract_styles(child, styles);
        }
    }
}

fn extract_links(node: &Node, links: &mut Vec<String>) {
    if let NodeType::Element(elem) = &node.node_type {
        if elem.tag_name.to_lowercase() == "link" {
            if let Some(rel) = elem.attributes.get("rel") {
                if rel.contains("stylesheet") {
                    if let Some(href) = elem.attributes.get("href") {
                        links.push(href.clone());
                    }
                }
            }
        }
        for child in &node.children {
            extract_links(child, links);
        }
    }
}

enum ScriptSource {
    Inline(String),
    External(String),
}

fn extract_scripts(node: &Node, scripts: &mut Vec<ScriptSource>) {
    if let NodeType::Element(elem) = &node.node_type {
        let tag = elem.tag_name.to_lowercase();
        if tag == "script" {
            let src = elem.attributes.get("src").cloned();
            if let Some(url) = src {
                if !url.is_empty() {
                    scripts.push(ScriptSource::External(url));
                    return;
                }
            }
            let text: String = node.children.iter()
                .filter_map(|c| {
                    if let NodeType::Text(t) = &c.node_type {
                        let s = t.trim().to_string();
                        if !s.is_empty() { Some(s) } else { None }
                    } else { None }
                })
                .collect::<Vec<_>>()
                .join("\n");
            if !text.is_empty() {
                scripts.push(ScriptSource::Inline(text));
            }
            return;
        }
    }
    for child in &node.children {
        extract_scripts(child, scripts);
    }
}

fn inject_js_output(dom: &mut Node, text: &str) {
    if text.is_empty() { return; }
    fn is_body(node: &Node) -> bool {
        matches!(&node.node_type, NodeType::Element(data) if data.tag_name.to_lowercase() == "body")
    }
    fn find_body_mut<'a>(node: &'a mut Node) -> Option<&'a mut Node> {
        if is_body(node) { return Some(node); }
        for child in &mut node.children {
            if let Some(found) = find_body_mut(child) {
                return Some(found);
            }
        }
        None
    }
    let target = if let Some(body) = find_body_mut(dom) { body } else { dom };
    // ponytail: parse document.write() output as HTML fragment
    let mut parser = crate::engine::parser::Parser::new(text.to_string());
    let fragment = parser.parse_node();
    target.children.extend(fragment.children);
}

fn run_blocking<T, F>(f: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    std::thread::spawn(f)
        .join()
        .map_err(|_| "blocking task panicked".to_string())
}

pub async fn fetch_page_content(url: String, content_width: f32, viewport_h: f32) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    plog!("FETCH", "URL={}", url);
    let response = match run_blocking({
        let url = url.clone();
        move || net::fetch_with_redirects(&url, 5)
    }) {
        Ok(Ok(resp)) => {
            plog!("FETCH", "Status=OK len={}", resp.body.len());
            resp
        }
        Ok(Err(e)) => {
            plog!("FETCH", "Fetch error: {}", e);
            return (url, vec![], None);
        }
        Err(e) => {
            plog!("FETCH", "Fetch task join error: {}", e);
            return (url, vec![], None);
        }
    };
    let net::Response { body: html, headers, .. } = response;
    let csp_blocks_scripts = net::csp_blocks_scripts(&headers);
    let csp_blocks_styles = net::csp_blocks_styles(&headers);

    let max_html = 1_000_000;
    let html = if html.len() > max_html {
        plog!("FETCH", "Truncated from {} to {}", html.len(), max_html);
        html[..max_html].to_string()
    } else {
        html
    };

    let mut parser = Parser::new(html);
    let mut dom_node = parser.parse_node();
    plog!("PARSE", "DOM root has {} children", dom_node.children.len());

    let mut styles = Vec::new();
    extract_styles(&dom_node, &mut styles);
    plog!("STYLE", "Found {} style blocks", styles.len());

    let mut stylesheet = Stylesheet { rules: Vec::new() };
    let style_limit = styles.len().min(50);
    plog!("STYLE", "Processing up to {} style blocks", style_limit);
    if csp_blocks_styles {
        plog!("STYLE", "Skipping inline styles due to CSP");
    }
    for (si, style_content) in styles.iter().take(style_limit).enumerate() {
        if csp_blocks_styles {
            break;
        }
        let max_css_len = 500_000;
        let trimmed = if style_content.len() > max_css_len {
            plog!("CSS", "Truncated inline style {} from {} to {}", si, style_content.len(), max_css_len);
            &style_content[..max_css_len]
        } else {
            style_content.as_str()
        };
        stylesheet.rules.extend(crate::engine::stratus::parse(trimmed).rules);
    }
    plog!("CSS", "Parsed {} rules from inline styles", stylesheet.rules.len());

    let mut link_urls = Vec::new();
    extract_links(&dom_node, &mut link_urls);
    let link_limit = link_urls.len().min(50);
    plog!("CSS", "Found {} external CSS links, processing {}", link_urls.len(), link_limit);
    for link_url in link_urls.iter().take(link_limit) {
        if csp_blocks_styles {
            break;
        }
        let resolved = net::resolve_url(link_url, &url);
        if let Ok(cache) = css_cache().read() {
            if let Some(cached) = cache.get(&resolved) {
                plog!("CSS", "Cache HIT: {}", resolved);
                stylesheet.rules.extend(cached.rules.clone());
                continue;
            }
        }
        plog!("CSS", "Fetching external CSS from {}", resolved);
        match run_blocking({
            let resolved = resolved.clone();
            move || net::fetch(&resolved)
        }) {
            Ok(Ok((css_content, css_status))) => {
                if css_status >= 400 {
                    plog!("CSS", "External CSS HTTP error {} for {}", css_status, resolved);
                } else {
                    let max_css_len = 500_000;
                    let trimmed = if css_content.len() > max_css_len {
                        plog!("CSS", "Truncated external CSS from {} to {}", css_content.len(), max_css_len);
                        css_content[..max_css_len].to_string()
                    } else {
                        css_content
                    };
                    let parsed = crate::engine::stratus::parse(&trimmed);
                    if let Ok(mut cache) = css_cache().write() {
                        if cache.len() > 100 {
                            cache.clear();
                            plog!("CSS", "Cache evicted (size > 100)");
                        }
                        cache.insert(resolved.clone(), parsed.clone());
                    }
                    let rules = parsed.rules;
                    let count = rules.len();
                    stylesheet.rules.extend(rules);
                    plog!("CSS", "Parsed {} rules from external CSS", count);
                }
            }
            Ok(Err(e)) => { plog!("CSS", "Failed to fetch external CSS: {}", e); }
            Err(e) => { plog!("CSS", "Fetch task join error: {}", e); }
        }
    }
    plog!("CSS", "Total stylesheet rules: {}", stylesheet.rules.len());

    let mut scripts = Vec::new();
    extract_scripts(&dom_node, &mut scripts);
    plog!("JS", "Found {} script blocks", scripts.len());
    let bridge = Arc::new(Mutex::new(JsBridge::load_dom(&dom_node, &url)));
    let mut js_engine = JSEngine::new();
    if csp_blocks_scripts {
        plog!("JS", "Skipping script execution due to CSP");
    }
    for (si, script) in scripts.iter().enumerate() {
        if csp_blocks_scripts {
            break;
        }
        let code = match script {
            ScriptSource::Inline(s) => {
                plog!("JS", "Executing inline script {}", si);
                s.clone()
            }
            ScriptSource::External(src) => {
                let resolved = net::resolve_url(src, &url);
                plog!("JS", "Fetching external script from {}", resolved);
                match run_blocking({
                    let resolved = resolved.clone();
                    move || net::fetch(&resolved)
                }) {
                    Ok(Ok((fetched, _status))) => fetched,
                    Ok(Err(e)) => {
                        plog!("JS", "Failed to fetch external script: {}", e);
                        continue;
                    }
                    Err(e) => {
                        plog!("JS", "Fetch task join error: {}", e);
                        continue;
                    }
                }
            }
        };
        let _ = js_engine.execute_with_bridge(&code, &bridge);
    }
    let (modified_dom, js_output) = {
        let mut guard = bridge.lock().unwrap_or_else(|e| e.into_inner());
        let dom = guard.to_dom();
        let output = guard.take_output();
        (dom, output)
    };
    dom_node = modified_dom;
    if !js_output.is_empty() {
        plog!("JS", "Injecting JS output ({} chars)", js_output.len());
        inject_js_output(&mut dom_node, &js_output);
    }

    let mut elements = Vec::new();
    extract_elements(&dom_node, &mut elements, 0, &stylesheet, None, None, vec![]);
    plog!("EXTRACT", "Extracted {} elements", elements.len());
    elements.truncate(2000);

    let mut img_count = 0;
    for el in elements.iter_mut() {
        if let Some(ref img_src) = el.image_url.clone() {
            img_count += 1;
            let resolved = net::resolve_url(img_src, &url);
            let bytes = match run_blocking({
                let resolved = resolved.clone();
                move || net::fetch_bytes(&resolved)
            }) {
                Ok(Ok(b)) => b,
                Ok(Err(e)) => {
                    plog!("IMAGES", "Failed to fetch image: {}", e);
                    continue;
                }
                Err(e) => {
                    plog!("IMAGES", "Fetch task join error: {}", e);
                    continue;
                }
            };
            if bytes.len() < 5_000_000 {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    let rgba = img.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    let max_dim = 800.0;
                    let scale = if (w as f32).max(h as f32) > max_dim {
                        max_dim / (w as f32).max(h as f32)
                    } else {
                        1.0
                    };
                    if scale < 1.0 {
                        let resized = image::imageops::resize(&rgba, (w as f32 * scale) as u32, (h as f32 * scale) as u32, image::imageops::FilterType::Lanczos3);
                        let (rw, rh) = resized.dimensions();
                        el.width = rw as f32;
                        el.height = rh as f32;
                        el.image_handle = Some(Handle::from_rgba(rw, rh, resized.into_raw()));
                    } else {
                        el.width = w as f32;
                        el.height = h as f32;
                        el.image_handle = Some(Handle::from_rgba(w, h, rgba.into_raw()));
                    }
                } else {
                    plog!("IMAGES", "Failed to decode image bytes ({} bytes)", bytes.len());
                }
            }
        }
    }
    plog!("IMAGES", "Loaded {} images", img_count);

    apply_caelum_layout(&mut elements, content_width, viewport_h);
    plog!("CAELUM", "Layout computed for {} elements", elements.len());

    plog!("FINAL", "Done. URL={} elements={}", url, elements.len());

    // ponytail: one engine per page-load script batch; dropped here, timer/event engine created on main thread in PageLoaded
    (url, elements, Some(bridge))
}
