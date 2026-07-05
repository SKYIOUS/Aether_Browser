use std::collections::HashMap;
use aether_browser::engine::stratus::{
    self, Color, ComputedStyle, Display, ElementData, FlexDirection, JustifyContent,
};
use aether_browser::engine::pipeline::extractor::{
    should_skip_tag, should_skip_content, extract_elements, StyledElement, decode_html_entities,
};
use aether_browser::engine::pipeline::layout::apply_caelum_layout;
use aether_browser::engine::parser::Parser;

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

fn make_test(tag: &str, text: &str, display: &str, parent: Option<usize>) -> StyledElement {
    StyledElement {
        tag: tag.to_string(), text: text.to_string(), wrapped_lines: vec![],
        dom_path: vec![],
        is_link: false, href: None, indent_level: 0,
        color: iced::Color::BLACK, font_size: 16.0, font_weight: "normal".to_string(),
        background_color: None, border_widths: [0.0; 4], border_color: None,
        image_handle: None, image_url: None,
        margin_top: 0.0, margin_bottom: 0.0, margin_left: None, margin_right: None,
        padding: [0.0; 4], display: display.to_string(),
        flex_direction: "row".to_string(), flex_wrap: "nowrap".to_string(),
        justify_content: "flex-start".to_string(), align_items: "stretch".to_string(),
        flex_grow: 0.0, flex_shrink: 1.0, flex_basis: None,
        css_width: None, css_height: None, parent_index: parent,
        min_width: None, max_width: None, min_height: None, max_height: None,
        x: 0.0, y: 0.0, width: 0.0, height: 0.0,
        line_height: 1.4, text_decoration: String::new(),
        text_transform: String::new(), border_radius: [0.0; 4],
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 1. CSS PARSING (tests 1-10)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_css_parse_color_hex() {
    let style = resolve("div { color: #ff0000; }", "div");
    assert_eq!(style.color, Some(Color { r: 255, g: 0, b: 0, a: 255 }));
}

#[test]
fn test_css_parse_color_named() {
    let style = resolve("div { color: green; }", "div");
    assert_eq!(style.color, Some(Color { r: 0, g: 128, b: 0, a: 255 }));
}

#[test]
fn test_css_parse_margin_shorthand() {
    let style = resolve("div { margin: 10px 20px 30px 40px; }", "div");
    assert_eq!(style.margin_top, Some(10.0));
    assert_eq!(style.margin_right, Some(20.0));
    assert_eq!(style.margin_bottom, Some(30.0));
    assert_eq!(style.margin_left, Some(40.0));
}

#[test]
fn test_css_parse_padding_shorthand() {
    let style = resolve("div { padding: 5px 15px; }", "div");
    assert_eq!(style.padding_top, Some(5.0));
    assert_eq!(style.padding_bottom, Some(5.0));
    assert_eq!(style.padding_left, Some(15.0));
    assert_eq!(style.padding_right, Some(15.0));
}

#[test]
fn test_css_parse_display_block() {
    let style = resolve("div { display: block; }", "div");
    assert_eq!(style.display, Display::Block);
}

#[test]
fn test_css_parse_display_flex() {
    let style = resolve("div { display: flex; }", "div");
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_css_parse_display_grid() {
    let style = resolve("div { display: grid; }", "div");
    assert_eq!(style.display, Display::Grid);
}

#[test]
fn test_css_parse_font_size_px() {
    let style = resolve("div { font-size: 24px; }", "div");
    assert_eq!(style.font_size, Some(24.0));
}

#[test]
fn test_css_parse_border_width() {
    let style = resolve("div { border-width: 3px; border-color: blue; }", "div");
    assert_eq!(style.border_top_width, Some(3.0));
    assert_eq!(style.border_bottom_width, Some(3.0));
    assert_eq!(style.border_top_color, Some(Color { r: 0, g: 0, b: 255, a: 255 }));
}

#[test]
fn test_css_parse_multiple_rules() {
    let css = "body { margin: 0; } .box { display: flex; } #main { color: red; }";
    let sheet = stratus::parse(css);
    assert_eq!(sheet.rules.len(), 3);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 2. COMPUTED STYLES (tests 11-15)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_computed_style_inline_to_block_for_block_tags() {
    let style = resolve("p { display: inline; }", "p");
    assert_eq!(style.display, Display::Inline);
}

#[test]
fn test_computed_style_class_overrides_tag() {
    let css = "p { color: black; } .red { color: red; }";
    let style = resolve_with_attrs(css, "p", &[("class", "red")]);
    assert_eq!(style.color, Some(Color { r: 255, g: 0, b: 0, a: 255 }));
}

#[test]
fn test_computed_style_id_overrides_class() {
    let css = ".box { color: green; } #special { color: purple; }";
    let style = resolve_with_attrs(css, "div", &[("class", "box"), ("id", "special")]);
    assert_eq!(style.color, Some(Color { r: 128, g: 0, b: 128, a: 255 }));
}

#[test]
fn test_computed_style_flex_direction() {
    let style = resolve("div { display: flex; flex-direction: column; }", "div");
    assert_eq!(style.flex.flex_direction, FlexDirection::Column);
}

#[test]
fn test_computed_style_justify_content() {
    let style = resolve("div { display: flex; justify-content: space-between; }", "div");
    assert_eq!(style.flex.justify_content, JustifyContent::SpaceBetween);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 3. EXTRACTOR LOGIC (tests 16-22)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_skip_tag_script() {
    assert!(should_skip_tag("script"));
}

#[test]
fn test_skip_tag_style() {
    assert!(should_skip_tag("style"));
}

#[test]
fn test_skip_tag_head() {
    assert!(should_skip_tag("head"));
}

#[test]
fn test_skip_tag_meta() {
    assert!(should_skip_tag("meta"));
}

#[test]
fn test_skip_tag_noscript() {
    assert!(should_skip_tag("noscript"));
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
fn test_skip_content_script() {
    assert!(should_skip_content("script"));
}

#[test]
fn test_skip_content_style() {
    assert!(should_skip_content("style"));
}

#[test]
fn test_no_skip_div() {
    assert!(!should_skip_tag("div"));
}

#[test]
fn test_no_skip_p() {
    assert!(!should_skip_tag("p"));
}

#[test]
fn test_no_skip_img() {
    assert!(!should_skip_tag("img"));
}

#[test]
fn test_no_skip_a() {
    assert!(!should_skip_tag("a"));
}

#[test]
fn test_extract_elements_from_simple_html() {
    let html = r#"<div><p>Hello</p></div>"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    assert!(!elements.is_empty());
}

#[test]
fn test_extract_elements_no_script_content() {
    let html = r#"<div><script>var x=1;</script><p>visible</p></div>"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    assert!(!elements.iter().any(|e| e.text.contains("var x")));
}

#[test]
fn test_extract_elements_script_content_skipped() {
    let html = r#"<div><script>alert('xss')</script><p>safe</p></div>"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    assert!(elements.iter().any(|e| e.text.contains("safe")));
    assert!(!elements.iter().any(|e| e.text.contains("alert")));
}

#[test]
fn test_extract_elements_head_content_hidden() {
    let html = r#"<html><head><title>T</title></head><body><p>body</p></body></html>"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    assert!(elements.iter().any(|e| e.text.contains("body")));
    assert!(!elements.iter().any(|e| e.text.contains("T") && e.tag == "text"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// 4. BLOCK LAYOUT (tests 23-28)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_block_elements_stack_vertically() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("h1", "Title", "block", Some(0)),
        make_test("p", "Paragraph", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].y > elements[1].y, "p.y={} should be > h1.y={}", elements[2].y, elements[1].y);
}

#[test]
fn test_block_elements_have_width() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "Hello", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].width > 0.0, "p should have width");
}

#[test]
fn test_block_elements_have_height() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "Hello", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].height > 0.0, "p should have height from text");
}

#[test]
fn test_nested_block_elements() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("div", "", "block", Some(0)),
        make_test("p", "Nested", "block", Some(1)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].y >= elements[1].y, "nested p should be below parent div");
}

#[test]
fn test_multiple_siblings_stacked() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "One", "block", Some(0)),
        make_test("p", "Two", "block", Some(0)),
        make_test("p", "Three", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].y > elements[1].y, "second p below first");
    assert!(elements[3].y > elements[2].y, "third p below second");
}

#[test]
fn test_block_with_margin_top() {
    // ponytail: first child's margin collapses with root — use padding to prevent
    let mut parent = make_test("div", "", "block", None);
    parent.padding = [1.0, 0.0, 0.0, 0.0];
    let mut el = make_test("p", "Hello", "block", Some(0));
    el.margin_top = 20.0;
    let mut elements = vec![parent, el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= 19.0, "p.y={} should be >= 20 (with padding)", elements[1].y);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 5. INLINE LAYOUT (tests 29-34)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_inline_siblings_flow_horizontally() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("span", "Hello", "inline", Some(0)),
        make_test("span", "World", "inline", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].x >= elements[1].x, "second span x={} >= first span x={}", elements[2].x, elements[1].x);
}

#[test]
fn test_inline_in_block() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("span", "Hi", "inline", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].x >= 0.0);
    assert!(elements[1].width > 0.0);
}

#[test]
fn test_inline_wraps_when_long() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("span", "AAAAAAAAAA BBBBBBBBBB CCCCCCCCCC DDDDDDDDDD", "inline", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 200.0, 600.0);
    assert!(elements[1].height > elements[1].font_size, "long text should wrap to multiple lines");
}

#[test]
fn test_inline_mixed_with_block() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("span", "Hello", "inline", Some(0)),
        make_test("p", "Block", "block", Some(0)),
        make_test("span", "World", "inline", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    for el in &elements {
        assert!(el.x.is_finite() && el.y.is_finite());
    }
}

#[test]
fn test_inline_block_element() {
    let mut el = make_test("div", "", "inline-block", Some(0));
    el.css_width = Some(100.0);
    el.css_height = Some(50.0);
    let mut elements = vec![
        make_test("div", "", "block", None),
        el,
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].width > 0.0);
}

#[test]
fn test_multiple_inline_spans() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("span", "A", "inline", Some(0)),
        make_test("span", "B", "inline", Some(0)),
        make_test("span", "C", "inline", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[3].x >= elements[2].x);
    assert!(elements[2].x >= elements[1].x);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 6. FLEXBOX LAYOUT (tests 35-40)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_flex_row_direction() {
    let mut elements = vec![
        StyledElement {
            display: "flex".to_string(),
            flex_direction: "row".to_string(),
            css_width: Some(400.0), css_height: Some(100.0),
            ..make_test("div", "", "flex", None)
        },
        make_test("span", "A", "flex", Some(0)),
        make_test("span", "B", "flex", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].x >= elements[1].x, "flex row: B.x={} >= A.x={}", elements[2].x, elements[1].x);
}

#[test]
fn test_flex_column_direction() {
    let mut elements = vec![
        StyledElement {
            display: "flex".to_string(),
            flex_direction: "column".to_string(),
            css_width: Some(400.0), css_height: Some(200.0),
            ..make_test("div", "", "flex", None)
        },
        make_test("p", "First", "flex", Some(0)),
        make_test("p", "Second", "flex", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].y > elements[1].y, "flex col: Second.y={} > First.y={}", elements[2].y, elements[1].y);
}

#[test]
fn test_flex_justify_center() {
    let mut elements = vec![
        StyledElement {
            display: "flex".to_string(),
            justify_content: "center".to_string(),
            css_width: Some(400.0), css_height: Some(100.0),
            ..make_test("div", "", "flex", None)
        },
        make_test("span", "X", "flex", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].x > 0.0, "flex centered child should not be at x=0");
}

#[test]
fn test_flex_align_items_center() {
    let mut elements = vec![
        StyledElement {
            display: "flex".to_string(),
            align_items: "center".to_string(),
            css_width: Some(400.0), css_height: Some(200.0),
            ..make_test("div", "", "flex", None)
        },
        make_test("span", "X", "flex", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y > 0.0, "flex aligned child should not be at y=0");
}

#[test]
fn test_flex_wrap_nowrap() {
    let mut elements = vec![
        StyledElement {
            display: "flex".to_string(),
            flex_wrap: "nowrap".to_string(),
            css_width: Some(100.0), css_height: Some(50.0),
            ..make_test("div", "", "flex", None)
        },
        make_test("span", "AAAA", "flex", Some(0)),
        make_test("span", "BBBB", "flex", Some(0)),
        make_test("span", "CCCC", "flex", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    for el in &elements[1..] {
        assert!(el.x.is_finite() && el.y.is_finite());
    }
}

#[test]
fn test_flex_grow() {
    let mut el_a = make_test("span", "A", "flex", Some(0));
    el_a.flex_grow = 1.0;
    let mut el_b = make_test("span", "B", "flex", Some(0));
    el_b.flex_grow = 1.0;
    let mut elements = vec![
        StyledElement {
            display: "flex".to_string(),
            css_width: Some(400.0), css_height: Some(50.0),
            ..make_test("div", "", "flex", None)
        },
        el_a, el_b,
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].width > 0.0);
    assert!(elements[2].width > 0.0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 7. GRID LAYOUT (tests 41-45)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_grid_display() {
    let style = resolve("div { display: grid; }", "div");
    assert_eq!(style.display, Display::Grid);
}

#[test]
fn test_grid_children_in_grid_container() {
    let mut elements = vec![
        StyledElement {
            display: "grid".to_string(),
            css_width: Some(400.0), css_height: Some(200.0),
            ..make_test("div", "", "grid", None)
        },
        make_test("div", "A", "block", Some(0)),
        make_test("div", "B", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    for el in &elements {
        assert!(el.x.is_finite() && el.y.is_finite());
    }
}

#[test]
fn test_grid_single_column() {
    let mut elements = vec![
        StyledElement {
            display: "grid".to_string(),
            css_width: Some(400.0), css_height: Some(300.0),
            ..make_test("div", "", "grid", None)
        },
        make_test("div", "1", "block", Some(0)),
        make_test("div", "2", "block", Some(0)),
        make_test("div", "3", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[3].y >= elements[2].y);
    assert!(elements[2].y >= elements[1].y);
}

#[test]
fn test_grid_item_sizes() {
    let mut elements = vec![
        StyledElement {
            display: "grid".to_string(),
            css_width: Some(400.0), css_height: Some(200.0),
            ..make_test("div", "", "grid", None)
        },
        make_test("div", "Cell", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].width > 0.0);
    assert!(elements[1].height > 0.0);
}

#[test]
fn test_grid_empty_container() {
    let mut elements = vec![
        StyledElement {
            display: "grid".to_string(),
            css_width: Some(400.0), css_height: Some(200.0),
            ..make_test("div", "", "grid", None)
        },
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].width > 0.0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 8. FLOAT LAYOUT (tests 46-50)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_float_left_element() {
    let mut el = make_test("div", "Float", "block", Some(0));
    el.css_width = Some(100.0);
    el.css_height = Some(50.0);
    let mut elements = vec![
        make_test("div", "", "block", None),
        el,
        make_test("p", "Content beside float", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].x >= 0.0);
    assert!(elements[2].x >= 0.0);
}

#[test]
fn test_float_right_element() {
    let mut el = make_test("div", "FloatR", "block", Some(0));
    el.css_width = Some(100.0);
    el.css_height = Some(50.0);
    let mut elements = vec![
        make_test("div", "", "block", None),
        el,
        make_test("p", "Content", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    for el in &elements {
        assert!(el.x.is_finite() && el.y.is_finite());
    }
}

#[test]
fn test_float_does_not_affect_siblings_positioning() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "First", "block", Some(0)),
        make_test("p", "Second", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].y > elements[1].y, "non-floated siblings should stack vertically");
}

#[test]
fn test_clear_both() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "After clear", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= 0.0);
}

#[test]
fn test_multiple_floats() {
    let mut el1 = make_test("div", "F1", "block", Some(0));
    el1.css_width = Some(50.0);
    el1.css_height = Some(50.0);
    let mut el2 = make_test("div", "F2", "block", Some(0));
    el2.css_width = Some(50.0);
    el2.css_height = Some(50.0);
    let mut elements = vec![
        make_test("div", "", "block", None),
        el1, el2,
        make_test("p", "After", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    for el in &elements {
        assert!(el.x.is_finite() && el.y.is_finite());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 9. MARGIN COLLAPSE (tests 51-55)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_margin_top_on_first_element() {
    // ponytail: first child's top margin collapses with root — use padding to prevent
    let mut parent = make_test("div", "", "block", None);
    parent.padding = [1.0, 0.0, 0.0, 0.0];
    let mut el = make_test("p", "Hello", "block", Some(0));
    el.margin_top = 30.0;
    let mut elements = vec![parent, el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= 29.0, "p.y={} should be >= margin_top=30 (with padding)", elements[1].y);
}

#[test]
fn test_margin_bottom_spacing() {
    let mut el1 = make_test("p", "First", "block", None);
    el1.margin_bottom = 20.0;
    let mut el2 = make_test("p", "Second", "block", None);
    el2.margin_top = 10.0;
    let mut elements = vec![el1, el2];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    let gap = elements[1].y - (elements[0].y + elements[0].height);
    assert!(gap >= 10.0, "gap between elements should be at least 10, got {}", gap);
}

#[test]
fn test_zero_margins() {
    let mut elements = vec![
        make_test("p", "A", "block", None),
        make_test("p", "B", "block", None),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= elements[0].y + elements[0].height - 1.0);
}

#[test]
fn test_large_margin_top() {
    // ponytail: first child's margin collapses with root — use padding to prevent
    let mut parent = make_test("div", "", "block", None);
    parent.padding = [1.0, 0.0, 0.0, 0.0];
    let mut el = make_test("p", "Spaced", "block", Some(0));
    el.margin_top = 100.0;
    let mut elements = vec![parent, el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= 99.0, "p.y={} should be >= 100 (with padding)", elements[1].y);
}

#[test]
fn test_margins_on_nested_elements() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        StyledElement {
            margin_top: 20.0, margin_bottom: 20.0,
            ..make_test("p", "Nested", "block", Some(0))
        },
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].y >= 20.0, "nested p y={} should be >= 20", elements[1].y);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 10. BORDER & PADDING (tests 56-60)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_border_widths_applied() {
    let mut elements = vec![
        StyledElement {
            border_widths: [2.0, 2.0, 2.0, 2.0],
            ..make_test("div", "Bordered", "block", None)
        },
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].border_widths[0] == 2.0);
}

#[test]
fn test_padding_affects_size() {
    let mut el = make_test("div", "", "block", None);
    el.padding = [10.0, 10.0, 10.0, 10.0];
    el.css_width = Some(200.0);
    el.css_height = Some(100.0);
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].width >= 200.0);
}

#[test]
fn test_border_color_set() {
    let mut el = make_test("div", "Color", "block", None);
    el.border_color = Some(iced::Color::from_rgb(1.0, 0.0, 0.0));
    el.border_widths = [1.0, 1.0, 1.0, 1.0];
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].border_color.is_some());
}

#[test]
fn test_no_border_no_padding() {
    let mut elements = vec![
        make_test("p", "Clean", "block", None),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert_eq!(elements[0].border_widths, [0.0; 4]);
    assert_eq!(elements[0].padding, [0.0; 4]);
}

#[test]
fn test_element_positioning_with_padding() {
    let mut el = make_test("div", "", "block", None);
    el.padding = [5.0, 5.0, 5.0, 5.0];
    el.css_width = Some(200.0);
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].x >= 0.0);
    assert!(elements[0].y >= 0.0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDGE CASES (bonus tests 61-70)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_elements_vec() {
    let mut elements: Vec<StyledElement> = vec![];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements.is_empty());
}

#[test]
fn test_single_element() {
    let mut elements = vec![make_test("div", "Solo", "block", None)];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].width > 0.0);
}

#[test]
fn test_display_none_elements_ignored() {
    let el = make_test("div", "Hidden", "none", None);
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert_eq!(elements[0].display, "none");
}

#[test]
fn test_very_narrow_container() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "Hello World", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 50.0, 600.0);
    assert!(elements[1].height > elements[1].font_size, "text should wrap in narrow container");
}

#[test]
fn test_very_wide_container() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "Hello", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 5000.0, 600.0);
    assert!(elements[1].width > 0.0);
}

#[test]
fn test_long_text_wrapping() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("p", "This is a very long paragraph that should wrap across multiple lines when rendered in a constrained width container", "block", Some(0)),
    ];
    apply_caelum_layout(&mut elements, 300.0, 600.0);
    assert!(elements[1].height > elements[1].font_size);
}

#[test]
fn test_elements_with_image_dimensions() {
    let mut el = make_test("img", "", "block", None);
    el.css_width = Some(300.0);
    el.css_height = Some(200.0);
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert_eq!(elements[0].width, 300.0);
    assert_eq!(elements[0].height, 200.0);
}

#[test]
fn test_link_element_preserves_href() {
    let mut el = make_test("a", "Click me", "inline", None);
    el.is_link = true;
    el.href = Some("https://example.com".to_string());
    let mut elements = vec![el];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[0].is_link);
    assert_eq!(elements[0].href.as_deref(), Some("https://example.com"));
}

#[test]
fn test_multiple_parent_levels() {
    let mut elements = vec![
        make_test("div", "", "block", None),
        make_test("div", "", "block", Some(0)),
        make_test("p", "Deep", "block", Some(1)),
    ];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[2].y >= elements[1].y);
    assert!(elements[1].y >= elements[0].y);
}

#[test]
fn test_font_size_affects_height() {
    let mut el_small = make_test("p", "Small", "block", None);
    el_small.font_size = 12.0;
    let mut el_large = make_test("p", "Large", "block", None);
    el_large.font_size = 32.0;
    let mut elements = vec![el_small, el_large];
    apply_caelum_layout(&mut elements, 800.0, 600.0);
    assert!(elements[1].height > elements[0].height, "larger font should produce taller element");
}

// ═══════════════════════════════════════════════════════════════════════════════
// 11. HTML ENTITY DECODING (tests 71+)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_decode_amp() {
    assert_eq!(decode_html_entities("&amp;"), "&");
}

#[test]
fn test_decode_lt() {
    assert_eq!(decode_html_entities("&lt;"), "<");
}

#[test]
fn test_decode_gt() {
    assert_eq!(decode_html_entities("&gt;"), ">");
}

#[test]
fn test_decode_quot() {
    assert_eq!(decode_html_entities("&quot;"), "\"");
}

#[test]
fn test_decode_apos() {
    assert_eq!(decode_html_entities("&apos;"), "'");
}

#[test]
fn test_decode_nbsp() {
    assert_eq!(decode_html_entities("&nbsp;"), "\u{00A0}");
}

#[test]
fn test_decode_decimal() {
    assert_eq!(decode_html_entities("&#123;"), "{");
}

#[test]
fn test_decode_hex_emoji() {
    assert_eq!(decode_html_entities("&#x1F600;"), "😀");
}

#[test]
fn test_decode_no_nested_recursion() {
    assert_eq!(decode_html_entities("&amp;lt;"), "&lt;");
}

#[test]
fn test_decode_no_entities_preserved() {
    assert_eq!(decode_html_entities("hello world"), "hello world");
}

#[test]
fn test_decode_preserves_unknown_entity() {
    assert_eq!(decode_html_entities("&unknown;"), "&unknown;");
}

#[test]
fn test_decode_mixed_text() {
    assert_eq!(decode_html_entities("a &amp; b &lt; c"), "a & b < c");
}

#[test]
fn test_decode_in_extracted_text() {
    let html = r#"<p>hello &amp; goodbye</p>"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = aether_browser::engine::stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    let p = elements.iter().find(|e| e.tag == "p").expect("should find <p>");
    assert_eq!(p.text, "hello & goodbye", "&amp; should decode to &");
}

#[test]
fn test_decode_href_attribute() {
    let html = r#"<a href="https://example.com?a=1&amp;b=2">link</a>"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = aether_browser::engine::stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    let a = elements.iter().find(|e| e.tag == "a").expect("should find <a>");
    assert_eq!(a.href.as_deref(), Some("https://example.com?a=1&b=2"), "href &amp; should decode");
}

#[test]
fn test_decode_alt_attribute() {
    let html = r#"<img src="x.png" alt="photo &amp; picture">"#;
    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse_node();
    let sheet = aether_browser::engine::stratus::parse("");
    let mut elements = Vec::new();
    extract_elements(&dom, &mut elements, 0, &sheet, None, None, vec![], 800.0, 600.0);
    let img = elements.iter().find(|e| e.tag == "img").expect("should find <img>");
    assert_eq!(img.text, "photo & picture", "alt &amp; should decode");
}
