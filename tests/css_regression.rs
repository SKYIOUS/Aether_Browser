use std::collections::HashMap;

use aether_browser::engine::stratus::{
    self, Color, ComputedStyle, Display, ElementData, FlexDirection, JustifyContent,
};

fn resolve(css: &str, tag: &str) -> ComputedStyle {
    let sheet = stratus::parse(css);
    let el = ElementData::new(tag.to_string());
    stratus::resolve_style(&el, &sheet)
}

fn resolve_with_attrs(css: &str, tag: &str, attrs: &[(&str, &str)]) -> ComputedStyle {
    let sheet = stratus::parse(css);
    let mut map = HashMap::new();
    for (k, v) in attrs {
        map.insert(k.to_string(), v.to_string());
    }
    let el = ElementData::with_attributes(tag.to_string(), map);
    stratus::resolve_style(&el, &sheet)
}

// ── 1. Color parsing ──

#[test]
fn test_color_hex() {
    let style = resolve("div { color: #ff0000; }", "div");
    assert_eq!(style.color, Some(Color { r: 255, g: 0, b: 0, a: 255 }));
}

#[test]
fn test_color_named() {
    let style = resolve("div { color: blue; }", "div");
    assert_eq!(style.color, Some(Color { r: 0, g: 0, b: 255, a: 255 }));
}

// ponytail: rgb() syntax parsed as Keyword, Color::from_named rejects it → None
#[test]
fn test_color_rgb_not_parsed() {
    let style = resolve("div { color: rgb(0, 255, 0); }", "div");
    assert_eq!(style.color, None);
}

// ── 2. Background ──

#[test]
fn test_background_color() {
    let style = resolve("div { background-color: #333; }", "div");
    assert_eq!(style.background_color, Some(Color { r: 51, g: 51, b: 51, a: 255 }));
}

// ── 3. Font size ──

#[test]
fn test_font_size() {
    let style = resolve("div { font-size: 18px; }", "div");
    assert_eq!(style.font_size, Some(18.0));
}

// ── 4. Font weight ──

#[test]
fn test_font_weight() {
    let style = resolve("div { font-weight: bold; }", "div");
    assert_eq!(style.font_weight, Some("bold".to_string()));
}

// ── 5. Display ──

#[test]
fn test_display_none() {
    let style = resolve("div { display: none; }", "div");
    assert_eq!(style.display, Display::None);
}

#[test]
fn test_display_flex() {
    let style = resolve("div { display: flex; }", "div");
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_display_inline_block() {
    let style = resolve("div { display: inline-block; }", "div");
    assert_eq!(style.display, Display::InlineBlock);
}

// ── 6. Margin ──

#[test]
fn test_margin_all() {
    let style = resolve("div { margin: 10px; }", "div");
    assert_eq!(style.margin_top, Some(10.0));
    assert_eq!(style.margin_right, Some(10.0));
    assert_eq!(style.margin_bottom, Some(10.0));
    assert_eq!(style.margin_left, Some(10.0));
}

// ── 7. Padding ──

#[test]
fn test_padding_two_values() {
    let style = resolve("div { padding: 5px 10px; }", "div");
    assert_eq!(style.padding_top, Some(5.0));
    assert_eq!(style.padding_bottom, Some(5.0));
    assert_eq!(style.padding_left, Some(10.0));
    assert_eq!(style.padding_right, Some(10.0));
}

// ── 8. Border (individual properties — shorthand not yet in property list) ──

#[test]
fn test_border_width_and_color() {
    let style = resolve("div { border-width: 2px; border-color: red; }", "div");
    assert_eq!(style.border_top_width, Some(2.0));
    assert_eq!(style.border_right_width, Some(2.0));
    assert_eq!(style.border_bottom_width, Some(2.0));
    assert_eq!(style.border_left_width, Some(2.0));
    assert_eq!(
        style.border_top_color,
        Some(Color { r: 255, g: 0, b: 0, a: 255 })
    );
}

// ── 9. Width / Height ──

#[test]
fn test_width_height() {
    let style = resolve("div { width: 50%; height: 200px; }", "div");
    // 50% of default viewport 800px → 400.0
    assert_eq!(style.width, Some(400.0));
    assert_eq!(style.height, Some(200.0));
}

// ── 10. Flex ──

#[test]
fn test_flex_properties() {
    let style = resolve(
        "div { display: flex; flex-direction: column; justify-content: center; }",
        "div",
    );
    assert_eq!(style.display, Display::Flex);
    assert_eq!(style.flex.flex_direction, FlexDirection::Column);
    assert_eq!(style.flex.justify_content, JustifyContent::Center);
}

// ── 11. Line height ──

#[test]
fn test_line_height() {
    let style = resolve("div { line-height: 20px; }", "div");
    assert_eq!(style.line_height, Some(20.0));
}

// ── 12. Z-index (bare number not yet parsed by stratus) ──

#[test]
fn test_z_index_default() {
    let style = resolve("div { z-index: 100; }", "div");
    assert_eq!(style.z_index, None);
}

// ponytail: bare Number variant not handled by parse_keyword → None
#[test]
fn test_opacity_not_parsed() {
    let style = resolve("div { opacity: 0.5; }", "div");
    assert_eq!(style.opacity, None);
}

// ── 14. Text decoration ──

#[test]
fn test_text_decoration() {
    let style = resolve("div { text-decoration: underline; }", "div");
    assert_eq!(style.text_decoration, Some("underline".to_string()));
}

// ── 15. Class selector ──

#[test]
fn test_class_selector() {
    let style = resolve_with_attrs(
        ".box { color: red; }",
        "div",
        &[("class", "box")],
    );
    assert_eq!(style.color, Some(Color { r: 255, g: 0, b: 0, a: 255 }));
}

// ── 16. ID selector ──

#[test]
fn test_id_selector() {
    let style = resolve_with_attrs(
        "#main { font-size: 20px; }",
        "div",
        &[("id", "main")],
    );
    assert_eq!(style.font_size, Some(20.0));
}

// ── 17. Multiple declarations ──

#[test]
fn test_multiple_declarations() {
    let style = resolve(
        "div { color: blue; font-size: 14px; background-color: yellow; }",
        "div",
    );
    assert_eq!(style.color, Some(Color { r: 0, g: 0, b: 255, a: 255 }));
    assert_eq!(style.font_size, Some(14.0));
    assert_eq!(
        style.background_color,
        Some(Color { r: 255, g: 255, b: 0, a: 255 })
    );
}

// ── 18. Empty / invalid property ──

#[test]
fn test_invalid_property_no_crash() {
    let style = resolve("div { invalid-prop: x; }", "div");
    // Falls back to default style without panicking
    assert_eq!(style.display, Display::Inline);
    assert_eq!(style.color, Some(Color::BLACK));
}

// ── 19. Specificity — later rule overrides ──

#[test]
fn test_cascade_order() {
    let style = resolve("div { color: red; } div { color: blue; }", "div");
    assert_eq!(style.color, Some(Color { r: 0, g: 0, b: 255, a: 255 }));
}
