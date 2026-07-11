use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use lru::LruCache;
use std::num::NonZeroUsize;

use iced::Color;
use iced::widget::image::Handle;

use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::Stylesheet;
use crate::engine::net;
use crate::engine::parser::Parser;
use crate::engine::js::{JsBridge, JSEngine};
use crate::plog;

use super::extractor::{extract_elements_flat, StyledElement};
use super::layout::apply_caelum_layout;

static CSS_CACHE: OnceLock<Mutex<LruCache<String, Stylesheet>>> = OnceLock::new();
fn css_cache() -> &'static Mutex<LruCache<String, Stylesheet>> {
    CSS_CACHE.get_or_init(|| Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())))
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

// ponytail: injects document.write() output into FlatNode vec
fn inject_js_output_flat(bridge: &mut crate::engine::js::JsBridge, text: &str) {
    if text.is_empty() { return; }
    if let Some(body_id) = bridge.body_id {
        let child_ids = bridge.parse_html_fragment(text);
        for &child_id in &child_ids {
            if let Some(child) = bridge.nodes.get_mut(child_id as usize) {
                child.parent = Some(body_id);
            }
        }
        if let Some(body) = bridge.nodes.get_mut(body_id as usize) {
            body.children.extend(child_ids);
        }
    }
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
        input_type: String::new(), input_value: String::new(), input_placeholder: String::new(), checked: false,
        position: "static".into(), inset_top: 0.0, inset_right: 0.0, inset_bottom: 0.0, inset_left: 0.0, svg_stroke: None, svg_stroke_width: 1.0, z_index: 0, is_svg: false, svg_path_data: String::new(), colspan: 1, rowspan: 1, grid_row: None, grid_col: None, table_col_count: 0, table_row_count: 0, attrs: std::collections::HashMap::new(),
    };
    vec![
        se("div", "", 0.0, 0.0, content_width, viewport_h, fg, 16.0, "normal", Some(bg)),
        se("h1", &format!("⚠  {}", reason), pad, 60.0, content_width - pad * 2.0, 36.0, red, 22.0, "bold", None),
        se("p", &format!("Could not load: {}", url), pad, 110.0, content_width - pad * 2.0, 24.0, muted, 14.0, "normal", None),
        se("p", "Check the URL and try again.", pad, 145.0, content_width - pad * 2.0, 20.0, fg, 14.0, "normal", None),
    ]
}

pub async fn fetch_page_content(url: String, content_width: f32, viewport_h: f32) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    tokio::task::spawn_blocking(move || do_fetch_page_content_sync(url, content_width, viewport_h))
        .await
        .unwrap_or_else(|e| {
            plog!("FETCH", "spawn_blocking join error: {}", e);
            (String::new(), vec![], None)
        })
}

fn do_fetch_page_content_sync(url: String, content_width: f32, viewport_h: f32) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    plog!("FETCH", "URL={}", url);
    let response = match net::fetch_with_redirects(&url, 5, None) {
        Ok(resp) => {
            plog!("FETCH", "Status=OK len={}", resp.body.len());
            resp
        }
        Err(e) => {
            plog!("FETCH", "Fetch error: {}", e);
            let reason = format!("{}", e);
            return (url.clone(), error_page(&url, &reason, content_width, viewport_h), None);
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
    let dom_node = parser.parse_node();
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
        if let Ok(mut cache) = css_cache().lock() {
            if let Some(cached) = cache.get(&resolved) {
                plog!("CSS", "Cache HIT: {}", resolved);
                stylesheet.rules.extend(cached.rules.clone());
                continue;
            }
        }
        plog!("CSS", "Fetching external CSS from {}", resolved);
        match net::fetch(&resolved) {
            Ok((css_content, css_status)) => {
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
                    if let Ok(mut cache) = css_cache().lock() {
                        // ponytail: LruCache::put auto-evicts LRU entry when over capacity
                        cache.put(resolved.clone(), parsed.clone());
                    }
                    let rules = parsed.rules;
                    let count = rules.len();
                    stylesheet.rules.extend(rules);
                    plog!("CSS", "Parsed {} rules from external CSS", count);
                }
            }
            Err(e) => { plog!("CSS", "Failed to fetch external CSS: {}", e); }
        }
    }
    plog!("CSS", "Total stylesheet rules: {}", stylesheet.rules.len());

    let js_enabled = super::is_js_enabled();
    let mut scripts = Vec::new();
    extract_scripts(&dom_node, &mut scripts);
    plog!("JS", "Found {} script blocks (js_enabled={})", scripts.len(), js_enabled);
    let bridge = Arc::new(Mutex::new(JsBridge::load_dom(&dom_node, &url)));
    let mut js_engine = JSEngine::new();
    if js_enabled {
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
                    match net::fetch(&resolved) {
                        Ok((fetched, _status)) => fetched,
                        Err(e) => {
                            plog!("JS", "Failed to fetch external script: {}", e);
                            continue;
                        }
                    }
                }
            };
            if let Err(e) = js_engine.execute_with_bridge(&code, &bridge) {
                plog!("JS", "Script execution failed: {}", e);
            }
        }
    } else {
        plog!("JS", "JavaScript disabled by user setting");
    }
    let flat_nodes = {
        let mut guard = bridge.lock().unwrap_or_else(|e| e.into_inner());
        let output = guard.take_output();
        if !output.is_empty() {
            plog!("JS", "Injecting JS output ({} chars)", output.len());
            inject_js_output_flat(&mut guard, &output);
        }
        // ponytail: clone FlatNode vec for extraction; avoids to_dom() serialization
        guard.nodes.clone()
    };

    let mut elements = Vec::with_capacity(flat_nodes.len().min(2000));
    extract_elements_flat(&flat_nodes, &mut elements, &stylesheet, content_width, viewport_h);
    plog!("EXTRACT", "Extracted {} elements", elements.len());
    elements.truncate(2000);

    // ponytail: per-page decoded image LRU, max 50 entries, evicted by clearing
    let mut img_cache: HashMap<String, (f32, f32, Handle)> = HashMap::new();
    let mut img_count = 0;
    for el in elements.iter_mut() {
        if let Some(ref img_src) = el.image_url.clone() {
            let resolved = net::resolve_url(img_src, &url);
            if !net::csp_allows_image_url(&resolved, &page_url, &csp_policy) {
                plog!("CSP", "Blocked image: {}", resolved);
                continue;
            }
            img_count += 1;
            if let Some((w, h, handle)) = img_cache.get(&resolved) {
                el.width = *w;
                el.height = *h;
                el.image_handle = Some(handle.clone());
                continue;
            }
            let bytes = match net::fetch_bytes(&resolved) {
                Ok(b) => b,
                Err(e) => {
                    plog!("IMAGES", "Failed to fetch image: {}", e);
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
                    let (fw, fh, handle) = if scale < 1.0 {
                        let resized = image::imageops::resize(&rgba, (w as f32 * scale) as u32, (h as f32 * scale) as u32, image::imageops::FilterType::Lanczos3);
                        let (rw, rh) = resized.dimensions();
                        (rw as f32, rh as f32, Handle::from_rgba(rw, rh, resized.into_raw()))
                    } else {
                        (w as f32, h as f32, Handle::from_rgba(w, h, rgba.into_raw()))
                    };
                    el.width = fw;
                    el.height = fh;
                    el.image_handle = Some(handle.clone());
                    if img_cache.len() >= 50 {
                        img_cache.clear();
                    }
                    img_cache.insert(resolved, (fw, fh, handle));
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

