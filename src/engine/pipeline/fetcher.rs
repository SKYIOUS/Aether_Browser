use std::sync::{Arc, Mutex, OnceLock};
use std::num::NonZeroUsize;
use iced::Color;
use iced::widget::image::Handle;

use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::Stylesheet;
use crate::engine::net;
use crate::engine::js::{JsBridge, JSEngine};
use crate::plog;

use lru::LruCache;
use resvg::usvg;
use resvg::tiny_skia;

use super::extractor::{extract_elements_flat, BoxSizing, FontWeight, MAX_ELEMENTS, StyledElement, TextDecor};
use crate::engine::stratus::{AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent, Position};
use super::layout::apply_taffy_layout;

static CSS_CACHE: OnceLock<Mutex<LruCache<String, (Stylesheet, usize)>>> = OnceLock::new();
fn css_cache() -> &'static Mutex<LruCache<String, (Stylesheet, usize)>> {
    CSS_CACHE.get_or_init(|| Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())))
}

// ponytail: byte ceilings, not fidelity targets (PLAN A1) — generous enough
// that real documents and stylesheets process whole; they only bound
// pathological inputs.
const MAX_HTML_BYTES: usize = 5_000_000;
const MAX_CSS_SOURCE_BYTES: usize = 500_000;
const CSS_TOTAL_BUDGET_BYTES: usize = 8_000_000;

fn trim_to_budget(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes { return s; }
    let mut end = max_bytes;
    while !s.is_char_boundary(end) { end -= 1; }
    &s[..end]
}

fn apply_html_budget(html: String, max_bytes: usize) -> String {
    let end = trim_to_budget(&html, max_bytes).len();
    if end == html.len() { return html; }
    plog!("FETCH", "Truncated HTML from {} to {} bytes", html.len(), end);
    let mut out = html;
    out.truncate(end);
    out
}

// Greedy in document order: a source is kept while cumulative retained bytes
// fit the budget; the first non-fitting source — and everything after it — is
// skipped whole. Counts exactly the bytes handed in, i.e. post per-source-trim,
// cache hits included. The `used` carry makes inline + external phases share
// one cumulative budget. Called once per external sheet, so under pressure a
// later small sheet can outlive an earlier large skip — ceiling behavior,
// not cascade-order fidelity.
fn css_sources_within_total_budget<'a>(
    sources: &[&'a str],
    used_bytes: usize,
    budget: usize,
) -> (Vec<&'a str>, usize) {
    let mut kept = Vec::new();
    let mut total = used_bytes;
    for source in sources {
        if total + source.len() > budget { break; }
        total += source.len();
        kept.push(*source);
    }
    (kept, total)
}

fn extract_styles(node: &Node, styles: &mut Vec<String>, depth: usize) {
    if depth > 100 { return; }
    if let NodeType::Element(elem) = &node.node_type {
        if elem.tag_name.to_lowercase() == "style" {
            for child in &node.children {
                if let NodeType::Text(text) = &child.node_type {
                    styles.push(text.clone());
                }
            }
        }
        for child in &node.children {
            extract_styles(child, styles, depth + 1);
        }
    }
}

fn extract_links(node: &Node, links: &mut Vec<String>, depth: usize) {
    if depth > 100 { return; }
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
            extract_links(child, links, depth + 1);
        }
    }
}

enum ScriptSource {
    Inline(String),
    External(String),
}

fn extract_scripts(node: &Node, scripts: &mut Vec<ScriptSource>, depth: usize) {
    if depth > 100 { return; }
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
        extract_scripts(child, scripts, depth + 1);
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


fn render_vayu_page(url: &str, content_width: f32, viewport_h: f32, session_history: &[String]) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    let page = match url {
        "vayu://newtab" => newtab_page(content_width, viewport_h),
        "vayu://history" => history_page(content_width, viewport_h, session_history.to_vec()),
        "vayu://bookmarks" => bookmarks_page(content_width, viewport_h),
        "vayu://settings" => settings_page(content_width, viewport_h),
        _ => return (url.to_string(), error_page(url, "Unknown internal page", content_width, viewport_h, 0), None),
    };
    (url.to_string(), page, None)
}

fn newtab_page(content_width: f32, viewport_h: f32) -> Vec<StyledElement> {
    let se = |tag: &str, text: &str, x: f32, y: f32, w: f32, h: f32, color: iced::Color, size: f32, weight: &str, bg: Option<iced::Color>| StyledElement {
        tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
        is_link: false, href: None, indent_level: 0, color, font_size: size, font_weight: if weight == "bold" { FontWeight::Bold } else { FontWeight::Normal },
        background_color: bg, border_widths: [0.0; 4], border_color: None, image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None, padding: [0.0; 4], display: Display::Block,
        flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap, justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch, align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox, flex_grow: 0.0, flex_shrink: 0.0, flex_basis: None,
        css_width: None, css_height: None, parent_index: None, min_width: None, max_width: None, min_height: None, max_height: None,
        x, y, width: w, height: h, line_height: 1.4, text_decoration: TextDecor::default(), border_radius: [0.0; 4],
        input_type: String::new(), input_value: String::new(), input_placeholder: String::new(), checked: false,
        position: Position::Static, inset_top: 0.0, inset_right: 0.0, inset_bottom: 0.0, inset_left: 0.0,
    };
    vec![
        se("div", "", 0.0, 0.0, content_width, viewport_h, iced::Color::from_rgb(0.98, 0.98, 0.98), 16.0, "normal", Some(iced::Color::WHITE)),
        se("h1", "New Tab", 40.0, 40.0, content_width - 80.0, 48.0, iced::Color::from_rgb(0.1, 0.1, 0.1), 28.0, "bold", None),
        se("p", "Welcome to Vayu Browser", 40.0, 100.0, content_width - 80.0, 24.0, iced::Color::from_rgb(0.4, 0.4, 0.4), 14.0, "normal", None),
    ]
}

fn history_page(content_width: f32, _viewport_h: f32, session_history: Vec<String>) -> Vec<StyledElement> {
    let pad = 24.0;
    let fg = Color::from_rgb(0.13, 0.13, 0.13);
    let muted = Color::from_rgb(0.45, 0.45, 0.45);
    let link_color = Color::from_rgb(0.0, 0.0, 0.93);
    let se = |text: &str, y: f32, color: Color, size: f32, weight: &str, is_link: bool, href: Option<String>| StyledElement {
        tag: if is_link { "a".into() } else { "p".into() },
        text: text.into(), wrapped_lines: vec![], dom_path: vec![],
        is_link, href, indent_level: 0, color, font_size: size,
        font_weight: if weight == "bold" { FontWeight::Bold } else { FontWeight::Normal },
        background_color: None, border_widths: [0.0; 4], border_color: None,
        image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None, padding: [0.0; 4],
        display: Display::Block,
        flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap, justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch, align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox,
        flex_grow: 0.0, flex_shrink: 0.0, flex_basis: None,
        css_width: Some(content_width - pad * 2.0), css_height: None, parent_index: Some(0),
        min_width: None, max_width: None, min_height: None, max_height: None,
        x: 0.0, y, width: 0.0, height: 0.0, line_height: 1.4,
        text_decoration: TextDecor::default(), border_radius: [0.0; 4],
        input_type: String::new(), input_value: String::new(), input_placeholder: String::new(),
        checked: false, position: Position::Static, inset_top: 0.0, inset_right: 0.0,
        inset_bottom: 0.0, inset_left: 0.0,
    };
    let mut out = vec![
        se("History", 60.0, fg, 22.0, "bold", false, None),
    ];
    if session_history.is_empty() {
        out.push(se("No pages visited yet.", 110.0, muted, 14.0, "normal", false, None));
        return out;
    }
    // Most recent first; collapse CONSECUTIVE repeats only - revisiting a page
    // after other pages keeps both entries.
    let mut last_emitted: Option<&String> = None;
    for (i, url) in session_history.iter().rev().enumerate() {
        if last_emitted == Some(url) { continue; }
        last_emitted = Some(url);
        let display = if url.chars().count() > 60 {
            let trimmed: String = url.chars().take(57).collect();
            format!("{}...", trimmed)
        } else {
            url.clone()
        };
        out.push(se(&display, 110.0 + i as f32 * 28.0, link_color, 14.0, "normal", true, Some(url.clone())));
    }
    out
}

fn bookmarks_page(content_width: f32, viewport_h: f32) -> Vec<StyledElement> {
    let se = |tag: &str, text: &str, x: f32, y: f32, w: f32, h: f32, color: iced::Color, size: f32, weight: &str, bg: Option<iced::Color>| StyledElement {
        tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
        is_link: false, href: None, indent_level: 0, color, font_size: size, font_weight: if weight == "bold" { FontWeight::Bold } else { FontWeight::Normal },
        background_color: bg, border_widths: [0.0; 4], border_color: None, image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None, padding: [0.0; 4], display: Display::Block,
        flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap, justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch, align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox, flex_grow: 0.0, flex_shrink: 0.0, flex_basis: None,
        css_width: None, css_height: None, parent_index: None, min_width: None, max_width: None, min_height: None, max_height: None,
        x, y, width: w, height: h, line_height: 1.4, text_decoration: TextDecor::default(), border_radius: [0.0; 4],
        input_type: String::new(), input_value: String::new(), input_placeholder: String::new(), checked: false,
        position: Position::Static, inset_top: 0.0, inset_right: 0.0, inset_bottom: 0.0, inset_left: 0.0,
    };
    vec![
        se("div", "", 0.0, 0.0, content_width, viewport_h, iced::Color::from_rgb(0.98, 0.98, 0.98), 16.0, "normal", Some(iced::Color::WHITE)),
        se("h1", "Bookmarks", 40.0, 40.0, content_width - 80.0, 48.0, iced::Color::from_rgb(0.1, 0.1, 0.1), 28.0, "bold", None),
    ]
}

fn settings_page(content_width: f32, viewport_h: f32) -> Vec<StyledElement> {
    let se = |tag: &str, text: &str, x: f32, y: f32, w: f32, h: f32, color: iced::Color, size: f32, weight: &str, bg: Option<iced::Color>| StyledElement {
        tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
        is_link: false, href: None, indent_level: 0, color, font_size: size, font_weight: if weight == "bold" { FontWeight::Bold } else { FontWeight::Normal },
        background_color: bg, border_widths: [0.0; 4], border_color: None, image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None, padding: [0.0; 4], display: Display::Block,
        flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap, justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch, align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox, flex_grow: 0.0, flex_shrink: 0.0, flex_basis: None,
        css_width: None, css_height: None, parent_index: None, min_width: None, max_width: None, min_height: None, max_height: None,
        x, y, width: w, height: h, line_height: 1.4, text_decoration: TextDecor::default(), border_radius: [0.0; 4],
        input_type: String::new(), input_value: String::new(), input_placeholder: String::new(), checked: false,
        position: Position::Static, inset_top: 0.0, inset_right: 0.0, inset_bottom: 0.0, inset_left: 0.0,
    };
    vec![
        se("div", "", 0.0, 0.0, content_width, viewport_h, iced::Color::from_rgb(0.98, 0.98, 0.98), 16.0, "normal", Some(iced::Color::WHITE)),
        se("h1", "Settings", 40.0, 40.0, content_width - 80.0, 48.0, iced::Color::from_rgb(0.1, 0.1, 0.1), 28.0, "bold", None),
    ]
}

fn error_page(url: &str, reason: &str, content_width: f32, viewport_h: f32, status: u16) -> Vec<StyledElement> {
    let pad = 24.0;
    let (red, title) = match status {
        404 => (Color::from_rgb(0.88, 0.18, 0.18), format!("404 — Not Found")),
        403 => (Color::from_rgb(0.88, 0.55, 0.18), format!("403 — Forbidden")),
        500 => (Color::from_rgb(0.88, 0.18, 0.18), format!("500 — Server Error")),
        _ => (Color::from_rgb(0.88, 0.18, 0.18), format!("{} — Error", status)),
    };
    let bg = Color::from_rgb(0.13, 0.13, 0.13);
    let fg = Color::from_rgb(0.95, 0.95, 0.95);
    let muted = Color::from_rgb(0.65, 0.65, 0.65);
    let se = |tag: &str, text: &str, x: f32, y: f32, w: f32, h: f32, color: Color, size: f32, weight: &str, bg: Option<Color>| StyledElement {
        tag: tag.into(), text: text.into(), wrapped_lines: vec![], dom_path: vec![],
        is_link: false, href: None, indent_level: 0, color, font_size: size, font_weight: if weight == "bold" { FontWeight::Bold } else { FontWeight::Normal },
        background_color: bg, border_widths: [0.0; 4], border_color: None, image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None, padding: [0.0; 4], display: Display::Block,
        flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::NoWrap, justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch, align_self: AlignSelf::Auto, box_sizing: BoxSizing::ContentBox, flex_grow: 0.0, flex_shrink: 0.0, flex_basis: None,
        css_width: None, css_height: None, parent_index: None, min_width: None, max_width: None, min_height: None, max_height: None,
        x, y, width: w, height: h, line_height: 1.4, text_decoration: TextDecor::default(), border_radius: [0.0; 4],
        input_type: String::new(), input_value: String::new(), input_placeholder: String::new(), checked: false,
        position: Position::Static, inset_top: 0.0, inset_right: 0.0, inset_bottom: 0.0, inset_left: 0.0,
    };
    vec![
        se("div", "", 0.0, 0.0, content_width, viewport_h, fg, 16.0, "normal", Some(bg)),
        se("h1", &format!("⚠  {}", title), pad, 60.0, content_width - pad * 2.0, 36.0, red, 22.0, "bold", None),
        se("p", &format!("Could not load: {}", url), pad, 110.0, content_width - pad * 2.0, 24.0, muted, 14.0, "normal", None),
        se("p", &format!("{}", reason), pad, 145.0, content_width - pad * 2.0, 20.0, fg, 14.0, "normal", None),
    ]
}

pub async fn fetch_page_content(url: String, content_width: f32, viewport_h: f32, session_history: Vec<String>) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    tokio::task::spawn_blocking(move || do_fetch_page_content_sync(url, content_width, viewport_h, session_history))
        .await
        .unwrap_or_else(|e| {
            plog!("FETCH", "spawn_blocking join error: {}", e);
            (String::new(), vec![], None)
        })
}

fn do_fetch_page_content_sync(url: String, content_width: f32, viewport_h: f32, session_history: Vec<String>) -> (String, Vec<StyledElement>, Option<Arc<Mutex<JsBridge>>>) {
    plog!("FETCH", "URL={}", url);

    if url.starts_with("vayu://") {
        return render_vayu_page(&url, content_width, viewport_h, &session_history);
    }

    let response = match net::fetch_with_redirects(&url, 5, None) {
        Ok(resp) => {
            plog!("FETCH", "Status=OK len={}", resp.body.len());
            resp
        }
        Err(e) => {
            plog!("FETCH", "Fetch error: {}", e);
            let (status, reason) = match e {
                crate::engine::net::FetchError::Http(code, msg) => (code, msg),
                _ => (0, format!("{}", e)),
            };
            return (url.clone(), error_page(&url, &reason, content_width, viewport_h, status), None);
        }
    };
    let net::Response { body: html, headers, final_url: page_url, .. } = response;
    let csp_policy = net::parse_csp_from_headers(&headers);

    let html = apply_html_budget(html, MAX_HTML_BYTES);

    let dom_node = crate::engine::parser::parse_html(&html);
    plog!("PARSE", "DOM root has {} children", dom_node.children.len());

    let mut styles = Vec::new();
    extract_styles(&dom_node, &mut styles, 0);
    plog!("STYLE", "Found {} style blocks", styles.len());

    let inline_styles_ok = net::csp_allows_inline_style(&csp_policy);
    let mut stylesheet = Stylesheet { rules: Vec::new() };
    let mut css_used = 0usize;
    if inline_styles_ok {
        let inline_refs: Vec<&str> = styles
            .iter()
            .map(|s| {
                if s.len() > MAX_CSS_SOURCE_BYTES {
                    plog!("CSS", "Trimmed inline style from {} to {} bytes", s.len(), MAX_CSS_SOURCE_BYTES);
                }
                trim_to_budget(s, MAX_CSS_SOURCE_BYTES)
            })
            .collect();
        let (kept, used) = css_sources_within_total_budget(&inline_refs, css_used, CSS_TOTAL_BUDGET_BYTES);
        if kept.len() < inline_refs.len() {
            plog!("CSS", "Total-CSS budget skipped {} inline style block(s)", inline_refs.len() - kept.len());
        }
        for (si, trimmed) in kept.iter().enumerate() {
            let rules = crate::engine::stratus::parse(trimmed).rules;
            plog!("STYLE", "Parsed {} rules from inline style {}", rules.len(), si);
            stylesheet.rules.extend(rules);
        }
        css_used = used;
    } else {
        plog!("CSP", "Blocked all {} inline style block(s) (no 'unsafe-inline')", styles.len());
    }
    plog!("CSS", "{} rules from inline styles, {} bytes of budget used", stylesheet.rules.len(), css_used);

    let mut link_urls = Vec::new();
    extract_links(&dom_node, &mut link_urls, 0);
    plog!("CSS", "Found {} external CSS links", link_urls.len());
    for link_url in link_urls.iter() {
        // Check before fetching: an exhausted budget must not download sheets
        // only to discard them (net::fetch has no download-size cap).
        if css_used >= CSS_TOTAL_BUDGET_BYTES {
            plog!("CSS", "Total-CSS budget exhausted; skipping remaining external sheets");
            break;
        }
        let resolved = net::resolve_url(link_url, &url);
        if !net::csp_allows_style_url(&resolved, &page_url, &csp_policy) {
            plog!("CSP", "Blocked external CSS: {}", resolved);
            continue;
        }
        if let Ok(mut cache) = css_cache().lock() {
            if let Some((cached, cached_bytes)) = cache.get(&resolved) {
                if css_used + cached_bytes > CSS_TOTAL_BUDGET_BYTES {
                    plog!("CSS", "Total-CSS budget skipped cached sheet {}", resolved);
                    continue;
                }
                plog!("CSS", "Cache HIT: {}", resolved);
                stylesheet.rules.extend(cached.rules.clone());
                css_used += cached_bytes;
                continue;
            }
        }
        plog!("CSS", "Fetching external CSS from {}", resolved);
        match net::fetch(&resolved) {
            Ok((css_content, css_status)) => {
                if css_status >= 400 {
                    plog!("CSS", "External CSS HTTP error {} for {}", css_status, resolved);
                } else {
                    let trimmed = {
                        if css_content.len() > MAX_CSS_SOURCE_BYTES {
                            plog!("CSS", "Trimmed external CSS from {} to {} bytes", css_content.len(), MAX_CSS_SOURCE_BYTES);
                        }
                        trim_to_budget(&css_content, MAX_CSS_SOURCE_BYTES)
                    };
                    let (kept, used_after) =
                        css_sources_within_total_budget(&[trimmed], css_used, CSS_TOTAL_BUDGET_BYTES);
                    if kept.is_empty() {
                        plog!("CSS", "Total-CSS budget skipped external sheet {}", resolved);
                        continue;
                    }
                    let parsed = crate::engine::stratus::parse(trimmed);
                    if let Ok(mut cache) = css_cache().lock() {
                        // ponytail: LruCache::put auto-evicts LRU entry when over capacity
                        cache.put(resolved.clone(), (parsed.clone(), trimmed.len()));
                    }
                    let rules = parsed.rules;
                    let count = rules.len();
                    stylesheet.rules.extend(rules);
                    css_used = used_after;
                    plog!("CSS", "Parsed {} rules from external CSS", count);
                }
            }
            Err(e) => { plog!("CSS", "Failed to fetch external CSS: {}", e); }
        }
    }
    plog!("CSS", "Total stylesheet rules: {}", stylesheet.rules.len());

    let js_enabled = super::is_js_enabled();
    let mut scripts = Vec::new();
    extract_scripts(&dom_node, &mut scripts, 0);
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

    let mut elements = Vec::with_capacity(flat_nodes.len().min(MAX_ELEMENTS));
    extract_elements_flat(&flat_nodes, &mut elements, &stylesheet, content_width, viewport_h);
    plog!("EXTRACT", "Extracted {} elements", elements.len());

    // ponytail: per-page decoded image LRU, max 50 entries
    let mut img_cache: LruCache<String, (f32, f32, Handle)> = LruCache::new(NonZeroUsize::new(50).unwrap());
    let mut decoded_img_bytes: u64 = 0;
    let max_page_img_bytes: u64 = 256 * 1024 * 1024; // 256MB page-level budget
    let mut img_count = 0;
    for el in elements.iter_mut() {
        if let Some(ref img_src) = el.image_url.clone() {
            let resolved = net::resolve_url(img_src, &url);
            if !net::csp_allows_image_url(&resolved, &page_url, &csp_policy) {
                plog!("CSP", "Blocked image: {}", resolved);
                continue;
            }
            img_count += 1;
            if let Some((w, hh, hnd)) = img_cache.get(&resolved).map(|(w, h, h2)| (*w, *h, h2.clone())) {
                el.width = w;
                el.height = hh;
                el.image_handle = Some(hnd);
                continue;
            }
            let bytes = match net::fetch_bytes(&resolved) {
                Ok(b) => b,
                Err(e) => {
                    plog!("IMAGES", "Failed to fetch image: {}", e);
                    continue;
                }
            };
            if bytes.len() >= 5_000_000 {
                plog!("IMAGES", "Image too large ({} bytes), skipping decode", bytes.len());
                continue;
            }
            if is_svg_bytes(&bytes) {
                if let Some(rgba) = decode_svg(&bytes) {
                    let (w, h) = rgba.dimensions();
                    let decoded_bytes = w as u64 * h as u64 * 4;
                    if decoded_bytes > SVG_MAX_BYTES {
                        plog!("IMAGES", "SVG decoded size too large ({} bytes), skipping", decoded_bytes);
                        continue;
                    }
                    decoded_img_bytes += decoded_bytes;
                    if decoded_img_bytes > max_page_img_bytes {
                        plog!("IMAGES", "Page image budget exceeded ({} bytes), skipping remaining", decoded_img_bytes);
                        break;
                    }
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
                    img_cache.put(resolved, (fw, fh, handle));
                } else {
                    plog!("IMAGES", "Failed to decode SVG ({} bytes), trying raster fallback", bytes.len());
                    if let Ok(img) = image::load_from_memory(&bytes) {
                        let rgba = img.to_rgba8();
                        let (w, h) = rgba.dimensions();
                        let decoded_bytes = w as u64 * h as u64 * 4;
                        if decoded_bytes <= SVG_MAX_BYTES && decoded_img_bytes + decoded_bytes <= max_page_img_bytes {
                            decoded_img_bytes += decoded_bytes;
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
                            img_cache.put(resolved, (fw, fh, handle));
                        } else {
                            plog!("IMAGES", "Raster fallback size too large, skipping");
                        }
                    } else {
                        plog!("IMAGES", "Failed to decode image bytes ({} bytes)", bytes.len());
                    }
                }
            } else if let Ok(img) = image::load_from_memory(&bytes) {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                let decoded_bytes = w as u64 * h as u64 * 4;
                if decoded_bytes > SVG_MAX_BYTES {
                    plog!("IMAGES", "Raster decoded size too large ({} bytes), skipping", decoded_bytes);
                    continue;
                }
                decoded_img_bytes += decoded_bytes;
                if decoded_img_bytes > max_page_img_bytes {
                    plog!("IMAGES", "Page image budget exceeded ({} bytes), skipping remaining", decoded_img_bytes);
                    break;
                }
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
                img_cache.put(resolved, (fw, fh, handle));
            } else {
                plog!("IMAGES", "Failed to decode image bytes ({} bytes)", bytes.len());
            }
        }
    }
    plog!("IMAGES", "Loaded {} images", img_count);

    apply_taffy_layout(&mut elements, content_width, viewport_h);
    plog!("CAELUM", "Layout computed for {} elements", elements.len());

    plog!("FINAL", "Done. URL={} elements={}", url, elements.len());

    (url, elements, Some(bridge))
}

fn is_svg_bytes(bytes: &[u8]) -> bool {
    let mut pos = 0;
    if bytes.starts_with(b"\xef\xbb\xbf") {
        pos += 3;
    }
    if bytes[pos..].starts_with(b"<?xml") {
        if let Some(xml_end) = bytes[pos..].iter().position(|&b| b == b'>') {
            pos += xml_end + 1;
        }
    }
    while pos < bytes.len() && matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r') {
        pos += 1;
    }
     bytes[pos..].starts_with(b"<svg") || bytes[pos..].starts_with(b"<SVG")
}

const SVG_MAX_DIM: u32 = 4096;
const SVG_MAX_BYTES: u64 = 67_108_864; // 64MB = 4096*4096*4
const SVG_RENDER_TIMEOUT_MS: u64 = 5_000;

fn decode_svg(bytes: &[u8]) -> Option<image::RgbaImage> {
    let (tx, rx) = std::sync::mpsc::channel::<Option<image::RgbaImage>>();
    let bytes_owned: Vec<u8> = bytes.to_vec();
    std::thread::spawn(move || {
        tx.send(decode_svg_inner(&bytes_owned)).ok();
    });
    match rx.recv_timeout(std::time::Duration::from_millis(SVG_RENDER_TIMEOUT_MS)) {
        Ok(result) => result,
        Err(_) => {
            plog!("SVG", "SVG render timed out (>{})ms, skipping", SVG_RENDER_TIMEOUT_MS);
            None
        }
    }
}

fn decode_svg_inner(bytes: &[u8]) -> Option<image::RgbaImage> {
    let font_db = svg_font_db().clone();
    let options = usvg::Options {
        fontdb: std::sync::Arc::new(font_db),
        ..Default::default()
    };
    let tree = usvg::Tree::from_data(bytes, &options).ok()?;
    let size = tree.size().to_int_size();
    let w = size.width().clamp(1, SVG_MAX_DIM);
    let h = size.height().clamp(1, SVG_MAX_DIM);
    let mut pixmap = tiny_skia::Pixmap::new(w, h)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let rgba = pixmap.data().to_vec();
    image::RgbaImage::from_raw(w, h, rgba)
}

static SVG_FONT_DB: std::sync::OnceLock<usvg::fontdb::Database> = std::sync::OnceLock::new();

fn svg_font_db() -> &'static usvg::fontdb::Database {
    SVG_FONT_DB.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();
        db
    })
}


#[cfg(test)]
mod budget_tests {
    use super::{apply_html_budget, css_sources_within_total_budget};

    const MB: usize = 1_000_000;

    #[test]
    fn html_under_budget_is_untouched() {
        let html = "<html><body>hi</body></html>".to_string();
        assert_eq!(apply_html_budget(html.clone(), 5 * MB), html);
    }

    // ponytail companion check: the cut must respect UTF-8 char boundaries,
    // which the old `html[..max]` slice would have panicked on.
    #[test]
    fn html_budget_cut_respects_char_boundaries() {
        let max = 5 * MB;
        let mut html = "a".repeat(max - 1);
        html.push('\u{00E9}'); // 2-byte char straddling the cut point
        let out = apply_html_budget(html, max);
        assert_eq!(out.len(), max - 1);
        assert!(std::str::from_utf8(out.as_bytes()).is_ok());
    }

    #[test]
    fn css_budget_keeps_everything_under_budget() {
        let sources = vec!["abc", "de"];
        let (kept, total) = css_sources_within_total_budget(&sources, 0, 10);
        assert_eq!(kept, vec!["abc", "de"]);
        assert_eq!(total, 5);
    }

    // Inline and external phases share one cumulative counter via `used`.
    #[test]
    fn css_budget_is_cumulative_across_phases() {
        let (_kept, used) = css_sources_within_total_budget(&["12345"], 0, 10);
        assert_eq!(used, 5);
        let (kept2, used2) = css_sources_within_total_budget(&["67890"], used, 10);
        assert_eq!(kept2, vec!["67890"]);
        assert_eq!(used2, 10);
        let (kept3, _) = css_sources_within_total_budget(&["zzz"], used2, 10);
        assert!(kept3.is_empty(), "exhausted budget must skip later sources");
    }

    // Deterministic: a source that would push past the budget is skipped
    // whole (no mid-source split beyond the caller's per-source trim), and so
    // is everything after it in document order.
    #[test]
    fn css_budget_skips_overflowing_source_and_rest() {
        let sources = vec!["aaaa", "bbbbbbbb", "cc"];
        let (kept, total) = css_sources_within_total_budget(&sources, 3, 10);
        assert_eq!(kept, vec!["aaaa"]);
        assert_eq!(total, 7);
    }

    // Accounting counts what the caller passes: bytes retained after the
    // per-source 500KB trim. Cache hits flow through the same call site.
    #[test]
    fn css_budget_counts_retained_bytes_not_original() {
        let big = "x".repeat(100);
        let (kept, total) = css_sources_within_total_budget(&[big.as_str()], 0, 10);
        assert!(kept.is_empty(), "source larger than the whole budget must be skipped");
        assert_eq!(total, 0);
    }
}
// ?? B3 history UI ??????????????????????????????????????????????????????

#[cfg(test)]
mod b3_history_tests {
    use super::{do_fetch_page_content_sync, history_page};
    use super::super::extractor::StyledElement;

    fn link_hrefs(elements: &[StyledElement]) -> Vec<String> {
        elements
            .iter()
            .filter(|e| e.is_link)
            .filter_map(|e| e.href.clone())
            .collect()
    }

    #[test]
    fn b3_lists_most_recent_first_collapsing_consecutive_only() {
        let page = history_page(
            800.0,
            600.0,
            vec![
                "https://a".to_string(),
                "https://b".to_string(),
                "https://b".to_string(),
                "https://c".to_string(),
            ],
        );
        assert_eq!(
            link_hrefs(&page),
            vec!["https://c", "https://b", "https://a"],
            "most recent first; consecutive dupes collapse, distinct repeats stay"
        );
    }

    #[test]
    fn b3_display_trimmed_but_href_full() {
        let long = format!("https://example.dev/{}", "x".repeat(120));
        let page = history_page(800.0, 600.0, vec![long.clone()]);
        assert_eq!(link_hrefs(&page), vec![long], "href must keep the full URL");
        let entry = page.iter().find(|e| e.is_link).expect("link element");
        assert!(entry.text.len() <= 64, "display text should be trimmed, got {}", entry.text.len());
    }

    #[test]
    fn b3_empty_state_is_not_a_link() {
        let page = history_page(800.0, 600.0, vec![]);
        assert!(link_hrefs(&page).is_empty());
        assert!(
            page.iter().any(|e| e.text.contains("No pages visited yet")),
            "empty state message missing"
        );
    }

    #[test]
    fn b3_end_to_end_sync_pipeline_renders_session_history() {
        let (url, elements, bridge) = do_fetch_page_content_sync(
            "vayu://history".to_string(),
            800.0,
            600.0,
            vec!["https://one".to_string(), "https://two".to_string()],
        );
        assert_eq!(url, "vayu://history");
        assert!(bridge.is_none(), "internal pages have no JS bridge");
        assert_eq!(
            link_hrefs(&elements),
            vec!["https://two", "https://one"],
            "session history must actually reach the vayu renderer"
        );
    }
}
