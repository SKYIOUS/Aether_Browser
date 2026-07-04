use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::collections::HashMap;

use iced::Color;
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

fn error_page(url: &str, reason: &str, content_width: f32, viewport_h: f32) -> Vec<StyledElement> {
    let pad = 24.0;
    let red = Color::from_rgb(0.88, 0.18, 0.18); // ponytail: simple accent, no palette import
    let bg = Color::from_rgb(0.13, 0.13, 0.13);
    let fg = Color::from_rgb(0.95, 0.95, 0.95);
    let muted = Color::from_rgb(0.65, 0.65, 0.65);
    // ponytail: no Default derive, inline closure to keep it short
    let se = |tag: &str, text: &str, x: f32, y: f32, w: f32, h: f32, color: Color, size: f32, weight: &str, bg: Option<Color>| StyledElement {
        tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
        is_link: false, href: None, indent_level: 0, color, font_size: size, font_weight: weight.into(),
        background_color: bg, border_widths: [0.0; 4], border_color: None, image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None, padding: [0.0; 4], display: "block".into(),
        flex_direction: "row".into(), flex_wrap: "nowrap".into(), justify_content: "flex-start".into(),
        align_items: "stretch".into(), flex_grow: 0.0, flex_shrink: 0.0, flex_basis: None,
        css_width: None, css_height: None, parent_index: None, min_width: None, max_width: None, min_height: None, max_height: None,
        x, y, width: w, height: h, line_height: 1.4, text_decoration: "none".into(), text_transform: "none".into(), border_radius: [0.0; 4],
    };
    vec![
        se("div", "", 0.0, 0.0, content_width, viewport_h, fg, 16.0, "normal", Some(bg)),
        se("h1", &format!("⚠  {}", reason), pad, 60.0, content_width - pad * 2.0, 36.0, red, 22.0, "bold", None),
        se("p", &format!("Could not load: {}", url), pad, 110.0, content_width - pad * 2.0, 24.0, muted, 14.0, "normal", None),
        se("p", "Check the URL and try again.", pad, 145.0, content_width - pad * 2.0, 20.0, fg, 14.0, "normal", None),
    ]
}

pub async fn fetch_page_content(url: String, content_width: f32, viewport_h: f32) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    plog!("FETCH", "URL={}", url);
    let response = match run_blocking({
        let url = url.clone();
        move || net::fetch_with_redirects(&url, 5, None)
    }) {
        Ok(Ok(resp)) => {
            plog!("FETCH", "Status=OK len={}", resp.body.len());
            resp
        }
        Ok(Err(e)) => {
            plog!("FETCH", "Fetch error: {}", e);
            return (url.clone(), error_page(&url, &e.to_string(), content_width, viewport_h), None);
        }
        Err(e) => {
            plog!("FETCH", "Fetch task join error: {}", e);
            return (url.clone(), error_page(&url, &e, content_width, viewport_h), None);
        }
    };
    let net::Response { body: html, headers, final_url: page_url, .. } = response;
    let csp_policy = net::parse_csp_from_headers(&headers);

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

    let inline_styles_ok = net::csp_allows_inline_style(&csp_policy);
    let mut stylesheet = Stylesheet { rules: Vec::new() };
    let style_limit = styles.len().min(50);
    plog!("STYLE", "Processing up to {} style blocks", style_limit);
    for (si, style_content) in styles.iter().take(style_limit).enumerate() {
        if !inline_styles_ok {
            plog!("CSP", "Blocked inline style block {} (no 'unsafe-inline')", si);
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
        let resolved = net::resolve_url(link_url, &url);
        if !net::csp_allows_style_url(&resolved, &page_url, &csp_policy) {
            plog!("CSP", "Blocked external CSS: {}", resolved);
            continue;
        }
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
    let inline_scripts_ok = net::csp_allows_inline_script(&csp_policy);
    for (si, script) in scripts.iter().enumerate() {
        let code = match script {
            ScriptSource::Inline(s) => {
                if !inline_scripts_ok {
                    plog!("CSP", "Blocked inline script {} (no 'unsafe-inline')", si);
                    continue;
                }
                plog!("JS", "Executing inline script {}", si);
                s.clone()
            }
            ScriptSource::External(src) => {
                let resolved = net::resolve_url(src, &url);
                if !net::csp_allows_script_url(&resolved, &page_url, &csp_policy) {
                    plog!("CSP", "Blocked external script: {}", resolved);
                    continue;
                }
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
        if let Err(e) = js_engine.execute_with_bridge(&code, &bridge) {
            plog!("JS", "Script execution failed: {}", e);
        }
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
    extract_elements(&dom_node, &mut elements, 0, &stylesheet, None, None, vec![], content_width, viewport_h);
    plog!("EXTRACT", "Extracted {} elements", elements.len());
    elements.truncate(2000);

    let mut img_count = 0;
    for el in elements.iter_mut() {
        if let Some(ref img_src) = el.image_url.clone() {
            let resolved = net::resolve_url(img_src, &url);
            if !net::csp_allows_image_url(&resolved, &page_url, &csp_policy) {
                plog!("CSP", "Blocked image: {}", resolved);
                continue;
            }
            img_count += 1;
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

