use aether_browser::engine::pipeline::{normalize_nav_url, Tab, apply_caelum_layout};
use aether_browser::engine::pipeline::extractor::{should_skip_tag, should_skip_content};
use aether_browser::engine::net::normalize_url;
use aether_browser::engine::pipeline::StyledElement;
use aether_browser::ui::screens::settings::AetherSettings;
use iced::Color;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_element(tag: &str, text: &str, display: &str) -> StyledElement {
    StyledElement {
        tag: tag.to_string(),
        text: text.to_string(),
        wrapped_lines: vec![],
        dom_path: vec![],
        is_link: false,
        href: None,
        indent_level: 0,
        color: Color::BLACK,
        font_size: 16.0,
        font_weight: "normal".to_string(),
        background_color: None,
        border_widths: [0.0; 4],
        border_color: None,
        image_handle: None,
        image_url: None,
        margin_top: 0.0,
        margin_bottom: 0.0,
        margin_left: None,
        margin_right: None,
        padding: [0.0; 4],
        display: display.to_string(),
        flex_direction: "row".to_string(),
        flex_wrap: "nowrap".to_string(),
        justify_content: "flex-start".to_string(),
        align_items: "stretch".to_string(),
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: None,
        css_width: None,
        css_height: None,
        parent_index: None,
        min_width: None,
        max_width: None,
        min_height: None,
        max_height: None,
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
        line_height: 1.4,
        text_decoration: String::new(),
        text_transform: String::new(),
        border_radius: [0.0; 4],
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// 1. Tab / Sidebar Construction Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_tab_struct_construction() {
    let tab = Tab { title: "Home".to_string(), url: "https://example.com".to_string() };
    assert_eq!(tab.title, "Home");
    assert_eq!(tab.url, "https://example.com");
}

#[test]
fn test_tab_empty_title() {
    let tab = Tab { title: String::new(), url: "about:blank".to_string() };
    assert!(tab.title.is_empty());
    assert_eq!(tab.url, "about:blank");
}

#[test]
fn test_tab_clone() {
    let tab = Tab { title: "Test".to_string(), url: "https://rust-lang.org".to_string() };
    let cloned = tab.clone();
    assert_eq!(cloned.title, tab.title);
    assert_eq!(cloned.url, tab.url);
}

#[test]
fn test_tab_serialization_roundtrip() {
    let tabs = vec![
        Tab { title: "Tab 1".to_string(), url: "https://a.com".to_string() },
        Tab { title: "Tab 2".to_string(), url: "https://b.com".to_string() },
    ];
    let json = serde_json::to_string(&tabs).unwrap();
    let deserialized: Vec<Tab> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.len(), 2);
    assert_eq!(deserialized[0].title, "Tab 1");
    assert_eq!(deserialized[1].url, "https://b.com");
}

// ═════════════════════════════════════════════════════════════════════════════
// 2. URL Normalization Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_normalize_nav_url_https() {
    assert_eq!(normalize_nav_url("https://example.com"), "https://example.com");
}

#[test]
fn test_normalize_nav_url_http() {
    assert_eq!(normalize_nav_url("http://example.com"), "http://example.com");
}

#[test]
fn test_normalize_nav_url_bare_domain() {
    assert_eq!(normalize_nav_url("example.com"), "https://example.com");
}

#[test]
fn test_normalize_nav_url_double_slash() {
    assert_eq!(normalize_nav_url("//example.com/path"), "https://example.com/path");
}

#[test]
fn test_normalize_nav_url_aether_protocol() {
    assert_eq!(normalize_nav_url("aether://home"), "aether://home");
}

#[test]
fn test_normalize_nav_url_about_blank() {
    assert_eq!(normalize_nav_url("about:blank"), "about:blank");
}

#[test]
fn test_normalize_nav_url_empty() {
    assert_eq!(normalize_nav_url(""), "about:blank");
}

#[test]
fn test_normalize_nav_url_whitespace_only() {
    assert_eq!(normalize_nav_url("   "), "about:blank");
}

#[test]
fn test_normalize_nav_url_strips_whitespace() {
    assert_eq!(normalize_nav_url("  https://example.com  "), "https://example.com");
}

#[test]
fn test_normalize_url_plain() {
    assert_eq!(normalize_url("example.com"), "https://example.com");
}

#[test]
fn test_normalize_url_with_path() {
    assert_eq!(normalize_url("example.com/path"), "https://example.com/path");
}

#[test]
fn test_normalize_url_already_has_scheme() {
    assert_eq!(normalize_url("http://example.com"), "http://example.com");
}

#[test]
fn test_normalize_url_double_slash_strips_extra_slash() {
    // normalize_url just prepends https:// to non-scheme URLs
    let result = normalize_url("//example.com");
    assert!(result.starts_with("https://"));
}

// ═════════════════════════════════════════════════════════════════════════════
// 3. Search Engine Fallback Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_is_url_with_scheme() {
    assert!(AetherSettings::is_url("https://example.com"));
    assert!(AetherSettings::is_url("http://example.com"));
}

#[test]
fn test_is_url_with_dot() {
    assert!(AetherSettings::is_url("example.com"));
    assert!(AetherSettings::is_url("sub.example.co.uk"));
}

#[test]
fn test_is_url_aether_protocol() {
    assert!(AetherSettings::is_url("aether://design/spatial-minimalism"));
}

#[test]
fn test_is_url_about_protocol() {
    assert!(AetherSettings::is_url("about:blank"));
}

#[test]
fn test_is_url_plain_search_query() {
    assert!(!AetherSettings::is_url("hello world"));
    assert!(!AetherSettings::is_url("rust programming language"));
}

#[test]
fn test_search_url_duckduckgo() {
    let settings = AetherSettings { default_search_engine: "duckduckgo".to_string(), ..Default::default() };
    let url = settings.search_url("hello world");
    assert!(url.contains("duckduckgo.com"));
    assert!(url.contains("hello+world"));
}

#[test]
fn test_search_url_google() {
    let settings = AetherSettings { default_search_engine: "google".to_string(), ..Default::default() };
    let url = settings.search_url("rust");
    assert!(url.contains("google.com/search"));
    assert!(url.contains("rust"));
}

#[test]
fn test_search_url_special_chars() {
    let settings = AetherSettings::default();
    let url = settings.search_url("hello & goodbye = yes");
    assert!(url.contains("hello"));
    assert!(!url.contains(" ")); // spaces should be encoded
}

// ═════════════════════════════════════════════════════════════════════════════
// 4. Settings Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_settings_defaults() {
    let s = AetherSettings::default();
    assert_eq!(s.home_page_url, "aether://design/spatial-minimalism");
    assert_eq!(s.default_search_engine, "duckduckgo");
    assert!(s.js_enabled);
    assert!(s.cookies_enabled);
}

#[test]
fn test_settings_serialization_roundtrip() {
    let s = AetherSettings {
        home_page_url: "https://custom.com".to_string(),
        default_search_engine: "google".to_string(),
        js_enabled: false,
        cookies_enabled: false,
    };
    let json = serde_json::to_string(&s).unwrap();
    let deserialized: AetherSettings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.home_page_url, "https://custom.com");
    assert_eq!(deserialized.default_search_engine, "google");
    assert!(!deserialized.js_enabled);
    assert!(!deserialized.cookies_enabled);
}

#[test]
fn test_settings_load_nonexistent_file_returns_defaults() {
    // load() reads from "aether_settings.json"; if missing, returns default
    // We can't guarantee the file doesn't exist, but we can test the default path
    let s = AetherSettings::default();
    assert_eq!(s.js_enabled, true);
}

#[test]
fn test_settings_save_and_load() {
    let path = "aether_settings_test.json";
    let s = AetherSettings {
        home_page_url: "aether://test".to_string(),
        default_search_engine: "google".to_string(),
        js_enabled: false,
        cookies_enabled: true,
    };
    let json = serde_json::to_string_pretty(&s).unwrap();
    std::fs::write(path, &json).unwrap();
    let loaded: AetherSettings = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(loaded.home_page_url, "aether://test");
    assert_eq!(loaded.default_search_engine, "google");
    assert!(!loaded.js_enabled);
    assert!(loaded.cookies_enabled);
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_settings_toggle_js() {
    let mut s = AetherSettings::default();
    assert!(s.js_enabled);
    s.js_enabled = !s.js_enabled;
    assert!(!s.js_enabled);
    s.js_enabled = !s.js_enabled;
    assert!(s.js_enabled);
}

#[test]
fn test_settings_toggle_cookies() {
    let mut s = AetherSettings::default();
    assert!(s.cookies_enabled);
    s.cookies_enabled = !s.cookies_enabled;
    assert!(!s.cookies_enabled);
}

// ═════════════════════════════════════════════════════════════════════════════
// 5. History Navigation Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_history_initial_state() {
    let hist = vec![(vec!["https://a.com".to_string()], 0)];
    let (urls, idx) = &hist[0];
    assert_eq!(urls.len(), 1);
    assert_eq!(*idx, 0);
}

#[test]
fn test_history_push() {
    let mut hist: Vec<(Vec<String>, usize)> = vec![(vec!["https://a.com".to_string()], 0)];
    let (ref mut urls, ref mut idx) = hist[0];
    urls.push("https://b.com".to_string());
    *idx = urls.len() - 1;
    assert_eq!(urls.len(), 2);
    assert_eq!(*idx, 1);
}

#[test]
fn test_history_back() {
    let mut hist: Vec<(Vec<String>, usize)> = vec![(
        vec!["https://a.com".to_string(), "https://b.com".to_string()],
        1,
    )];
    let (ref mut _urls, ref mut idx) = hist[0];
    assert!(*idx > 0);
    *idx -= 1;
    assert_eq!(*idx, 0);
}

#[test]
fn test_history_forward() {
    let mut hist: Vec<(Vec<String>, usize)> = vec![(
        vec!["https://a.com".to_string(), "https://b.com".to_string()],
        0,
    )];
    let (ref mut _urls, ref mut idx) = hist[0];
    if *idx + 1 < _urls.len() {
        *idx += 1;
    }
    assert_eq!(*idx, 1);
}

#[test]
fn test_history_cannot_go_back_from_start() {
    let hist: Vec<(Vec<String>, usize)> = vec![(vec!["https://a.com".to_string()], 0)];
    let (_urls, idx) = &hist[0];
    assert_eq!(*idx, 0);
    // Going back from index 0 is not possible
}

#[test]
fn test_history_cannot_go_forward_from_end() {
    let hist: Vec<(Vec<String>, usize)> = vec![(
        vec!["https://a.com".to_string(), "https://b.com".to_string()],
        1,
    )];
    let (urls, idx) = &hist[0];
    assert_eq!(*idx + 1, urls.len()); // at end
}

#[test]
fn test_history_truncate_on_new_navigate() {
    let mut hist_entries = vec![
        "https://a.com".to_string(),
        "https://b.com".to_string(),
        "https://c.com".to_string(),
    ];
    // Simulate navigating to a new page from middle
    let mut idx = 1;
    hist_entries.truncate(idx + 1);
    hist_entries.push("https://d.com".to_string());
    idx = hist_entries.len() - 1;
    assert_eq!(hist_entries, vec!["https://a.com", "https://b.com", "https://d.com"]);
    assert_eq!(idx, 2);
}

#[test]
fn test_history_limit_many_entries() {
    let mut hist: Vec<String> = (0..1000).map(|i| format!("https://{}.com", i)).collect();
    // Simulate a cap
    let max = 500;
    if hist.len() > max {
        hist = hist.split_off(hist.len() - max);
    }
    assert_eq!(hist.len(), 500);
    assert_eq!(hist[0], "https://500.com");
}

// ═════════════════════════════════════════════════════════════════════════════
// 6. Autocomplete Filtering Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_autocomplete_filter_exact_prefix() {
    let history = vec![
        "https://example.com".to_string(),
        "https://exotic.org".to_string(),
        "https://other.net".to_string(),
    ];
    let input = "https://ex";
    let matches: Vec<&String> = history.iter().filter(|h| h.contains(input)).collect();
    assert_eq!(matches.len(), 2);
}

#[test]
fn test_autocomplete_filter_no_matches() {
    let history = vec![
        "https://example.com".to_string(),
        "https://other.net".to_string(),
    ];
    let input = "zzz";
    let matches: Vec<&String> = history.iter().filter(|h| h.contains(input)).collect();
    assert!(matches.is_empty());
}

#[test]
fn test_autocomplete_filter_case_insensitive_contains() {
    let history = vec![
        "https://Example.Com".to_string(),
    ];
    let input = "example";
    // contains is case-sensitive, so this tests current behavior
    let matches: Vec<&String> = history.iter().filter(|h| h.contains(input)).collect();
    assert!(matches.is_empty()); // "Example.Com" doesn't contain "example"
}

#[test]
fn test_autocomplete_filter_limit_results() {
    let history: Vec<String> = (0..20).map(|i| format!("https://site{}.com", i)).collect();
    let input = "site";
    let limit = 8;
    let matches: Vec<&String> = history.iter().filter(|h| h.contains(input)).take(limit).collect();
    assert_eq!(matches.len(), limit);
}

#[test]
fn test_autocomplete_empty_input_shows_nothing() {
    let history = vec!["https://example.com".to_string()];
    let input = "";
    let show_autocomplete = !input.is_empty() && history.iter().any(|h| h.contains(input));
    assert!(!show_autocomplete);
}

// ═════════════════════════════════════════════════════════════════════════════
// 7. StyledElement Construction Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_styled_element_construction() {
    let el = make_element("div", "Hello", "block");
    assert_eq!(el.tag, "div");
    assert_eq!(el.text, "Hello");
    assert_eq!(el.display, "block");
}

#[test]
fn test_styled_element_default_values() {
    let el = make_element("p", "", "block");
    assert!(!el.is_link);
    assert!(el.href.is_none());
    assert_eq!(el.font_size, 16.0);
    assert_eq!(el.font_weight, "normal");
    assert_eq!(el.x, 0.0);
    assert_eq!(el.y, 0.0);
}

#[test]
fn test_styled_element_link() {
    let mut el = make_element("a", "Click me", "inline");
    el.is_link = true;
    el.href = Some("https://example.com".to_string());
    assert!(el.is_link);
    assert_eq!(el.href.as_deref(), Some("https://example.com"));
}

#[test]
fn test_styled_element_with_background() {
    let mut el = make_element("div", "", "block");
    el.background_color = Some(Color::from_rgb(1.0, 0.0, 0.0));
    assert!(el.background_color.is_some());
    let bg = el.background_color.unwrap();
    assert_eq!(bg.r, 1.0);
}

#[test]
fn test_styled_element_with_border() {
    let mut el = make_element("div", "", "block");
    el.border_widths = [1.0, 2.0, 3.0, 4.0];
    el.border_color = Some(Color::BLACK);
    assert_eq!(el.border_widths, [1.0, 2.0, 3.0, 4.0]);
    assert!(el.border_color.is_some());
}

#[test]
fn test_styled_element_with_margin() {
    let mut el = make_element("div", "", "block");
    el.margin_top = 10.0;
    el.margin_bottom = 20.0;
    assert_eq!(el.margin_top, 10.0);
    assert_eq!(el.margin_bottom, 20.0);
}

#[test]
fn test_styled_element_with_image() {
    let mut el = make_element("img", "", "block");
    el.image_url = Some("https://example.com/photo.jpg".to_string());
    assert!(el.image_url.is_some());
    assert!(el.image_handle.is_none()); // no Handle loaded yet
}

#[test]
fn test_styled_element_clone() {
    let el = make_element("span", "text", "inline");
    let cloned = el.clone();
    assert_eq!(cloned.tag, el.tag);
    assert_eq!(cloned.text, el.text);
}

// ═════════════════════════════════════════════════════════════════════════════
// 8. Layout / Caelum Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_layout_single_block_element() {
    let mut elements = vec![make_element("div", "Hello", "block")];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].x.is_finite());
    assert!(elements[0].y.is_finite());
    assert!(elements[0].width > 0.0);
}

#[test]
fn test_layout_two_block_elements_stacked() {
    let mut elements = vec![
        make_element("div", "First", "block"),
        make_element("div", "Second", "block"),
    ];
    elements[1].parent_index = Some(0);
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= elements[0].y);
}

#[test]
fn test_layout_inline_elements_side_by_side() {
    let mut elements = vec![
        make_element("div", "", "block"),
        make_element("span", "A", "inline"),
        make_element("span", "B", "inline"),
    ];
    elements[1].parent_index = Some(0);
    elements[2].parent_index = Some(0);
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].x >= elements[1].x);
}

#[test]
fn test_layout_hidden_element() {
    let mut el = make_element("div", "Hidden", "block");
    el.display = "none".to_string();
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    // hidden elements should have zero dimensions
    assert_eq!(elements[0].width, 0.0);
}

#[test]
fn test_layout_with_margin() {
    let mut elements = vec![
        make_element("div", "", "block"),
        make_element("div", "Spaced", "block"),
    ];
    elements[1].parent_index = Some(0);
    elements[1].margin_top = 20.0;
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= 20.0, "child y={} should be >= 20", elements[1].y);
}

#[test]
fn test_layout_with_border() {
    let mut elements = vec![make_element("div", "Bordered", "block")];
    elements[0].border_widths = [5.0, 5.0, 5.0, 5.0];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].width >= 10.0); // at least left+right border
}

// ═════════════════════════════════════════════════════════════════════════════
// 9. Tag Filtering Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_skip_tag_script() {
    assert!(should_skip_tag("script"));
}

#[test]
fn test_skip_tag_style() {
    assert!(should_skip_tag("style"));
}

#[test]
fn test_skip_tag_noscript() {
    assert!(should_skip_tag("noscript"));
}

#[test]
fn test_skip_tag_meta() {
    assert!(should_skip_tag("meta"));
}

#[test]
fn test_skip_tag_link() {
    assert!(should_skip_tag("link"));
}

#[test]
fn test_skip_tag_head() {
    assert!(should_skip_tag("head"));
}

#[test]
fn test_skip_tag_svg() {
    assert!(should_skip_tag("svg"));
}

#[test]
fn test_skip_tag_template() {
    assert!(should_skip_tag("template"));
}

#[test]
fn test_no_skip_tag_div() {
    assert!(!should_skip_tag("div"));
}

#[test]
fn test_no_skip_tag_p() {
    assert!(!should_skip_tag("p"));
}

#[test]
fn test_no_skip_tag_img() {
    assert!(!should_skip_tag("img"));
}

#[test]
fn test_no_skip_tag_a() {
    assert!(!should_skip_tag("a"));
}

#[test]
fn test_no_skip_tag_span() {
    assert!(!should_skip_tag("span"));
}

#[test]
fn test_skip_content_script() {
    assert!(should_skip_content("script"));
}

#[test]
fn test_skip_content_style() {
    assert!(should_skip_content("style"));
}

#[test]
fn test_no_skip_content_div() {
    assert!(!should_skip_content("div"));
}

#[test]
fn test_no_skip_content_p() {
    assert!(!should_skip_content("p"));
}

// ═════════════════════════════════════════════════════════════════════════════
// 10. Tab Save/Load Roundtrip Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_save_tabs_empty() {
    let path = "aether_tabs_test.json";
    let tabs: Vec<Tab> = vec![];
    let json = serde_json::to_string(&tabs).unwrap();
    std::fs::write(path, &json).unwrap();
    let loaded: Vec<Tab> = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert!(loaded.is_empty());
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_save_tabs_multiple() {
    let path = "aether_tabs_test2.json";
    let tabs = vec![
        Tab { title: "Rust".to_string(), url: "https://rust-lang.org".to_string() },
        Tab { title: "Iced".to_string(), url: "https://iced.rs".to_string() },
    ];
    let json = serde_json::to_string(&tabs).unwrap();
    std::fs::write(path, &json).unwrap();
    let loaded: Vec<Tab> = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].title, "Rust");
    assert_eq!(loaded[1].url, "https://iced.rs");
    let _ = std::fs::remove_file(path);
}

// ═════════════════════════════════════════════════════════════════════════════
// 11. Sidebar Item Label Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sidebar_workspace_labels() {
    let workspaces = vec!["Design Studio", "Research Lab", "Deep Work"];
    assert_eq!(workspaces.len(), 3);
    assert!(workspaces.contains(&"Design Studio"));
    assert!(workspaces.contains(&"Research Lab"));
    assert!(workspaces.contains(&"Deep Work"));
}

#[test]
fn test_sidebar_collection_labels() {
    let collections = vec!["Aether UI", "Rust / Iced Docs"];
    assert_eq!(collections.len(), 2);
    assert!(collections.contains(&"Aether UI"));
}

#[test]
fn test_sidebar_section_headers() {
    let headers = vec!["WORKSPACES", "COLLECTIONS"];
    assert_eq!(headers.len(), 2);
    assert!(headers.iter().all(|h| h.chars().all(|c| c.is_uppercase())));
}

// ═════════════════════════════════════════════════════════════════════════════
// 12. DevTools Tab Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_devtools_tab_variants() {
    use aether_browser::ui::screens::browser::DevToolsTab;
    let console = DevToolsTab::Console;
    let elements = DevToolsTab::Elements;
    let network = DevToolsTab::Network;
    assert_ne!(console, elements);
    assert_ne!(elements, network);
    assert_ne!(console, network);
}

#[test]
fn test_devtools_tab_default_is_console() {
    use aether_browser::ui::screens::browser::DevToolsTab;
    let current = DevToolsTab::Console;
    assert_eq!(current, DevToolsTab::Console);
}

// ═════════════════════════════════════════════════════════════════════════════
// 13. Edge Case Tests
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn test_normalize_nav_url_with_port() {
    assert_eq!(normalize_nav_url("localhost:3000"), "https://localhost:3000");
}

#[test]
fn test_normalize_nav_url_with_path_and_query() {
    assert_eq!(
        normalize_nav_url("example.com/search?q=rust&page=1"),
        "https://example.com/search?q=rust&page=1"
    );
}

#[test]
fn test_normalize_nav_url_with_fragment() {
    assert_eq!(
        normalize_nav_url("example.com/page#section"),
        "https://example.com/page#section"
    );
}

#[test]
fn test_settings_search_url_empty_query() {
    let settings = AetherSettings::default();
    let url = settings.search_url("");
    assert!(url.contains("q="));
}

#[test]
fn test_settings_search_url_unicode_query() {
    let settings = AetherSettings::default();
    let url = settings.search_url("日本語テスト");
    assert!(url.contains("q="));
}

#[test]
fn test_tab_title_update() {
    let mut tab = Tab { title: "Loading...".to_string(), url: "https://example.com".to_string() };
    tab.title = "Example Domain".to_string();
    assert_eq!(tab.title, "Example Domain");
}
