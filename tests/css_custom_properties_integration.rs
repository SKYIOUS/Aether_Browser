use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::pipeline::extractor::{extract_elements, StyledElement};
use vayu_browser::engine::pipeline::layout::apply_taffy_layout;
use vayu_browser::engine::stratus::{
    self, Color, ComputedStyle, CustomPropertyMap, ElementData, PropertyValue,
};

// ── diagnostic ──

#[test]
fn test_stratus_parse_calc_produces_calc_variant() {
    let sheet = stratus::parse("div { padding: calc(10px + 5px); }");
    assert_eq!(sheet.rules.len(), 1, "should have 1 rule");
    let decls = &sheet.rules[0].declarations;
    assert_eq!(decls.len(), 1, "should have 1 declaration");
    assert_eq!(decls[0].name, "padding");
    match &decls[0].value {
        PropertyValue::Calc(terms) => {
            assert!(!terms.is_empty(), "calc terms should not be empty");
        }
        other => panic!("expected Calc, got {:?}", other),
    }
}

#[test]
fn test_stratus_parse_var_produces_var_variant() {
    let sheet = stratus::parse("div { color: var(--primary); }");
    let decls = &sheet.rules[0].declarations;
    assert_eq!(decls.len(), 1);
    match &decls[0].value {
        PropertyValue::Var { name, .. } => {
            assert_eq!(name, "--primary");
        }
        other => panic!("expected Var, got {:?}", other),
    }
}

#[test]
fn test_stratus_parse_calc_with_var() {
    let sheet = stratus::parse("div { padding: calc(var(--spacing) * 2); }");
    let decls = &sheet.rules[0].declarations;
    assert_eq!(
        decls.len(),
        1,
        "should have 1 declaration, got {}",
        decls.len()
    );
    match &decls[0].value {
        PropertyValue::Calc(terms) => {
            assert!(!terms.is_empty(), "calc terms should not be empty");
        }
        other => panic!("expected Calc, got {:?}", other),
    }
}

#[test]
fn test_direct_resolve_calc() {
    // Construct stylesheet directly (bypasses stratus::parse) to verify resolver
    let sheet = stratus::Stylesheet {
        rules: vec![stratus::Rule {
            selectors: vec![stratus::Selector::Simple(stratus::SimpleSelector {
                tag_name: Some("div".into()),
                id: None,
                class: vec![],
                attribute: None,
                pseudo_class: None,
            })],
            declarations: vec![stratus::Declaration {
                name: "padding".into(),
                value: PropertyValue::Calc(vec![
                    stratus::CalcTerm::Length(stratus::LengthValue {
                        value: 10.0,
                        unit: stratus::Unit::Px,
                    }),
                    stratus::CalcTerm::Add,
                    stratus::CalcTerm::Length(stratus::LengthValue {
                        value: 5.0,
                        unit: stratus::Unit::Px,
                    }),
                ]),
            }],
        }],
    };
    let el = ElementData::new("div".to_string());
    let style =
        stratus::resolve_style_with_vars(&el, &sheet, 800.0, 600.0, &CustomPropertyMap::new());
    assert_eq!(style.padding_top, Some(15.0));
}

// ── helpers ──

fn resolve_with_parent(
    css: &str,
    tag: &str,
    attrs: &[(&str, &str)],
    parent_vars: &CustomPropertyMap,
) -> (ComputedStyle, CustomPropertyMap) {
    let sheet = stratus::parse(css);
    let map: std::collections::HashMap<String, String> = attrs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let el = if map.is_empty() {
        ElementData::new(tag.to_string())
    } else {
        ElementData::with_attributes(tag.to_string(), map)
    };
    let (cs, custom, _mask) =
        stratus::resolve_style_with_vars_and_custom(&el, &sheet, 800.0, 600.0, parent_vars, None);
    (cs, custom)
}

fn extract(html: &str, css: &str) -> Vec<StyledElement> {
    let dom = parse_html(html);
    let sheet = stratus::parse(css);
    let mut elements = Vec::new();
    extract_elements(
        &dom,
        &mut elements,
        0,
        &sheet,
        None,
        None,
        vec![],
        800.0,
        600.0,
        &CustomPropertyMap::new(),
        None,
    );
    elements
}

fn extract_and_layout(html: &str, css: &str, vw: f32, vh: f32) -> Vec<StyledElement> {
    let dom = parse_html(html);
    let sheet = stratus::parse(css);
    let mut elements = Vec::new();
    extract_elements(
        &dom,
        &mut elements,
        0,
        &sheet,
        None,
        None,
        vec![],
        vw,
        vh,
        &CustomPropertyMap::new(),
        None,
    );
    apply_taffy_layout(&mut elements, vw, vh);
    elements
}

// ══════════════════════════════════════════════════════════════
// 1. Bootstrap/Tailwind-style: custom properties + var() + calc
// ══════════════════════════════════════════════════════════════

#[test]
fn test_var_substitution_reaches_computed_color() {
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--primary".into(),
        stratus::PropertyValue::Keyword("#336699".into()),
    );
    let css = "div { color: var(--primary); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    let expected = stratus::Color::from_hex("#336699").unwrap();
    assert_eq!(style.color, Some(expected));
}

#[test]
fn test_var_substitution_reaches_computed_padding() {
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--spacing".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 20.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { padding: calc(var(--spacing) * 2); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.padding_top, Some(40.0));
    assert_eq!(style.padding_right, Some(40.0));
    assert_eq!(style.padding_bottom, Some(40.0));
    assert_eq!(style.padding_left, Some(40.0));
}

#[test]
fn test_calc_width_with_var() {
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--spacing".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 20.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { width: calc(100% - var(--spacing)); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    // 100% of 800vw = 800 - 20 = 780
    assert_eq!(style.width, Some(780.0));
}

#[test]
fn test_full_bootstrap_like_pipeline() {
    let css = r#"
        div.card { color: var(--primary); background-color: var(--bg); padding: calc(var(--spacing) * 2); width: calc(100% - var(--spacing)); font-size: var(--text-size); }
    "#;
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--primary".into(),
        stratus::PropertyValue::Keyword("#336699".into()),
    );
    parent.insert(
        "--bg".into(),
        stratus::PropertyValue::Keyword("#ffffff".into()),
    );
    parent.insert(
        "--spacing".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 20.0,
            unit: stratus::Unit::Px,
        }),
    );
    parent.insert(
        "--text-size".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 16.0,
            unit: stratus::Unit::Px,
        }),
    );

    let (style, _) = resolve_with_parent(css, "div", &[("class", "card")], &parent);

    let expected_color = stratus::Color::from_hex("#336699").unwrap();
    assert_eq!(style.color, Some(expected_color));
    assert_eq!(style.padding_top, Some(40.0));
    assert_eq!(style.font_size, Some(16.0));
}

#[test]
fn test_full_bootstrap_like_layout() {
    // Define custom properties on the div itself since :root only matches Document.
    let css = r#"
        div { --primary: #336699; --spacing: 20px; --text-size: 16px; }
        div.card { color: var(--primary); padding: calc(var(--spacing) * 2); width: calc(100% - var(--spacing)); font-size: var(--text-size); }
    "#;
    let elements = extract_and_layout(r#"<div class="card">Hello</div>"#, css, 800.0, 600.0);
    let card = elements
        .iter()
        .find(|e| e.text == "Hello")
        .expect("should find card text");
    assert!(card.x.is_finite(), "card x should be finite");
    assert!(card.y.is_finite(), "card y should be finite");
    assert!(card.width > 0.0, "card width should be positive");
    assert!(card.height > 0.0, "card height should be positive");
}

// ══════════════════════════════════════════════════════════════
// 2. Inherited variables across nested elements
// ══════════════════════════════════════════════════════════════

#[test]
fn test_inherited_variable_child_picks_up_parent() {
    let mut root_vars = CustomPropertyMap::new();
    root_vars.insert(
        "--c".into(),
        stratus::PropertyValue::Keyword("green".into()),
    );
    let css = "div { color: var(--c); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &root_vars);
    assert_eq!(
        style.color,
        Some(stratus::Color::from_named("green").unwrap())
    );
}

#[test]
fn test_inherited_variable_grandchild() {
    let mut parent_vars = CustomPropertyMap::new();
    parent_vars.insert(
        "--x".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 12.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { font-size: var(--x); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent_vars);
    assert_eq!(style.font_size, Some(12.0));
}

#[test]
fn test_local_override_wins_over_inherited() {
    let mut parent_vars = CustomPropertyMap::new();
    parent_vars.insert(
        "--c".into(),
        stratus::PropertyValue::Keyword("green".into()),
    );
    let css = "div { --c: red; color: var(--c); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent_vars);
    assert_eq!(
        style.color,
        Some(stratus::Color::from_named("red").unwrap())
    );
}

#[test]
fn test_inheritance_through_extract_elements() {
    // Define --accent on div itself, use on child span
    let css = r#"
        div { --accent: #ff0000; }
        span.child { color: var(--accent); }
    "#;
    let html = r#"<div><span class="child">text</span></div>"#;
    let elements = extract(html, css);
    let child = elements
        .iter()
        .find(|e| e.text == "text")
        .expect("should find child text");
    let expected = stratus::Color::from_hex("#ff0000").unwrap();
    let expected_iced = iced::Color::from_rgb(
        expected.r as f32 / 255.0,
        expected.g as f32 / 255.0,
        expected.b as f32 / 255.0,
    );
    assert_eq!(child.color, expected_iced);
}

// ══════════════════════════════════════════════════════════════
// 3. Fallback values
// ══════════════════════════════════════════════════════════════

#[test]
fn test_missing_var_with_fallback_uses_fallback() {
    let css = "div { color: var(--missing, red); }";
    let parent = CustomPropertyMap::new();
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(
        style.color,
        Some(stratus::Color::from_named("red").unwrap())
    );
}

#[test]
fn test_missing_var_without_fallback_invalidates() {
    let css = "div { color: var(--undefined); }";
    let parent = CustomPropertyMap::new();
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    // No var, no fallback → empty keyword → parse_color fails → default BLACK
    assert_eq!(style.color, Some(stratus::Color::BLACK));
}

#[test]
fn test_fallback_is_ignored_when_var_exists() {
    let mut parent = CustomPropertyMap::new();
    parent.insert("--c".into(), stratus::PropertyValue::Keyword("blue".into()));
    let css = "div { color: var(--c, red); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(
        style.color,
        Some(stratus::Color::from_named("blue").unwrap())
    );
}

#[test]
fn test_nested_fallback() {
    let css = "div { padding-top: var(--a, var(--b, 10px)); }";
    let parent = CustomPropertyMap::new();
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.padding_top, Some(10.0));
}

#[test]
fn test_nested_fallback_inner_hit() {
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--b".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 20.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { padding-top: var(--a, var(--b, 10px)); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.padding_top, Some(20.0));
}

// ══════════════════════════════════════════════════════════════
// 4. calc() expressions
// ══════════════════════════════════════════════════════════════

#[test]
fn test_calc_addition_two_lengths() {
    let css = "div { padding: calc(10px + 5px); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(style.padding_top, Some(15.0));
    assert_eq!(style.padding_right, Some(15.0));
    assert_eq!(style.padding_bottom, Some(15.0));
    assert_eq!(style.padding_left, Some(15.0));
}

#[test]
fn test_calc_subtraction_with_viewport() {
    let css = "div { width: calc(100% - 40px); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(style.width, Some(760.0));
}

#[test]
fn test_calc_precedence() {
    // calc(2 + 3 * 4) = 2 + 12 = 14
    let css = "div { width: calc(2 + 3 * 4); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(style.width, Some(14.0));
}

#[test]
fn test_calc_with_var_and_literal() {
    // --spacing: 20px; padding: calc(var(--spacing) + 10px) = 30px
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--spacing".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 20.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { padding: calc(var(--spacing) + 10px); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.padding_top, Some(30.0));
}

#[test]
fn test_calc_division_by_zero_is_safe() {
    let css = "div { width: calc(100px / 0); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    // Division by zero → invalid calc → None
    assert_eq!(style.width, None);
}

// ══════════════════════════════════════════════════════════════
// 5. Variable in shorthand
// ══════════════════════════════════════════════════════════════

#[test]
fn test_var_in_margin_shorthand() {
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--m".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 16.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { margin: var(--m); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.margin_top, Some(16.0));
    assert_eq!(style.margin_right, Some(16.0));
}

#[test]
fn test_var_in_padding_shorthand() {
    let mut parent = CustomPropertyMap::new();
    parent.insert(
        "--p".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 8.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { padding: var(--p); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.padding_top, Some(8.0));
    assert_eq!(style.padding_right, Some(8.0));
    assert_eq!(style.padding_bottom, Some(8.0));
    assert_eq!(style.padding_left, Some(8.0));
}

// ══════════════════════════════════════════════════════════════
// 6. Invalid/edge cases — fail safely
// ══════════════════════════════════════════════════════════════

#[test]
fn test_circular_reference_does_not_panic() {
    let css = "div { --a: var(--b); --b: var(--a); color: var(--a); }";
    let parent = CustomPropertyMap::new();
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    // Circular → resolved to empty keyword → default BLACK
    assert_eq!(style.color, Some(stratus::Color::BLACK));
}

#[test]
fn test_undefined_var_no_fallback_no_panic() {
    let css = "div { color: var(--nonexistent); }";
    let parent = CustomPropertyMap::new();
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    assert_eq!(style.color, Some(stratus::Color::BLACK));
}

#[test]
fn test_invalid_calc_does_not_panic() {
    let css = "div { width: calc(10px / 0); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(style.width, None);
}

#[test]
fn test_var_with_invalid_fallback_does_not_panic() {
    let css = "div { color: var(--a, var(--b)); }";
    let parent = CustomPropertyMap::new();
    let (style, _) = resolve_with_parent(css, "div", &[], &parent);
    // Both undefined → empty keyword → default BLACK
    assert_eq!(style.color, Some(stratus::Color::BLACK));
}

#[test]
fn test_custom_properties_not_leaked_to_computed_style() {
    let css = "div { --my-custom: 42px; width: 100px; }";
    let (_, custom) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert!(custom.contains_key("--my-custom"));
}

#[test]
fn test_empty_stylesheet_no_crash() {
    let (style, _) = resolve_with_parent("", "div", &[], &CustomPropertyMap::new());
    assert_eq!(style.color, Some(stratus::Color::BLACK));
}

#[test]
fn test_extract_with_custom_properties_no_crash() {
    let css = r#"
        div { --primary: #336699; --spacing: 20px; }
        div.card { color: var(--primary); padding: calc(var(--spacing) * 2); }
    "#;
    let html = r#"<div class="card"><p>inner</p></div>"#;
    let elements = extract(html, css);
    assert!(!elements.is_empty(), "should extract at least one element");
}

#[test]
fn test_extract_deep_nesting_inherits_vars() {
    let css = r#"
        div { --color: orange; }
        div.a { --color: blue; }
        span { color: var(--color); }
    "#;
    let html = r#"<div class="a"><span>text</span></div>"#;
    let elements = extract(html, css);
    let span = elements
        .iter()
        .find(|e| e.text == "text")
        .expect("should find span text");
    let expected = stratus::Color::from_named("blue").unwrap();
    let expected_iced = iced::Color::from_rgb(
        expected.r as f32 / 255.0,
        expected.g as f32 / 255.0,
        expected.b as f32 / 255.0,
    );
    assert_eq!(span.color, expected_iced);
}

#[test]
fn test_extract_layout_with_calc_width() {
    let css = r#"div { --sidebar: 200px; } div.main { width: calc(100% - var(--sidebar)); }"#;
    let html = r#"<div class="main">content</div>"#;
    let elements = extract_and_layout(html, css, 1000.0, 600.0);
    let main = elements
        .iter()
        .find(|e| e.text == "content")
        .expect("should find main");
    // width = 1000 - 200 = 800
    assert!(
        main.width >= 790.0 && main.width <= 810.0,
        "main width should be ~800, got {}",
        main.width
    );
}

// ══════════════════════════════════════════════════════════════
// 7. Custom properties are returned for child inheritance
// ══════════════════════════════════════════════════════════════

#[test]
fn test_resolve_returns_custom_map_for_children() {
    let css = "div { --x: 10px; }";
    let (_, child_vars) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert!(child_vars.contains_key("--x"), "child should receive --x");
}

#[test]
fn test_resolve_with_inherited_parent_vars() {
    let mut parent_vars = CustomPropertyMap::new();
    parent_vars.insert(
        "--x".into(),
        stratus::PropertyValue::Length(stratus::LengthValue {
            value: 42.0,
            unit: stratus::Unit::Px,
        }),
    );
    let css = "div { font-size: var(--x); }";
    let (style, _) = resolve_with_parent(css, "div", &[], &parent_vars);
    assert_eq!(style.font_size, Some(42.0));
}

// ══════════════════════════════════════════════════════════════
// 8. CSS standard property inheritance
// ══════════════════════════════════════════════════════════════

use aether_css::{
    apply_inheritance, resolve_style_with_vars_and_custom, Display, InheritMask, InitialMask,
};

fn resolve_with_parent_computed(
    css: &str,
    tag: &str,
    classes: &[&str],
    parent_vars: &CustomPropertyMap,
    parent_computed: Option<&ComputedStyle>,
) -> (ComputedStyle, CustomPropertyMap) {
    let sheet = stratus::parse(css);
    let mut map = std::collections::HashMap::new();
    for c in classes {
        map.insert("class".to_string(), c.to_string());
    }
    let el = if map.is_empty() {
        ElementData::new(tag.to_string())
    } else {
        ElementData::with_attributes(tag.to_string(), map)
    };
    let (cs, custom, _mask) =
        resolve_style_with_vars_and_custom(&el, &sheet, 800.0, 600.0, parent_vars, parent_computed);
    (cs, custom)
}

#[test]
fn test_font_size_inherits_from_parent() {
    let parent_css = "div { font-size: 24px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.font_size,
        Some(24.0),
        "child should inherit font-size from parent"
    );
}

#[test]
fn test_font_weight_inherits_from_parent() {
    let parent_css = "div { font-weight: bold; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.font_weight.as_deref(),
        Some("bold"),
        "child should inherit font-weight"
    );
}

#[test]
fn test_color_inherits_from_parent() {
    let parent_css = "div { color: red; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert!(
        child.color.is_some(),
        "child should inherit color from parent"
    );
}

#[test]
fn test_font_family_inherits_from_parent() {
    let parent_css = "div { font-family: monospace; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.font_family.as_deref(),
        Some("monospace"),
        "child should inherit font-family"
    );
}

#[test]
fn test_text_align_inherits_from_parent() {
    let parent_css = "div { text-align: center; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.text_align.as_deref(),
        Some("center"),
        "child should inherit text-align"
    );
}

#[test]
fn test_visibility_inherits_from_parent() {
    let parent_css = "div { visibility: hidden; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.visibility.as_deref(),
        Some("hidden"),
        "child should inherit visibility"
    );
}

#[test]
fn test_child_override_prevents_inheritance() {
    let parent_css = "div { font-size: 24px; color: red; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { font-size: 12px; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.font_size,
        Some(12.0),
        "child's own font-size should override parent"
    );
    assert!(
        child.color.is_some(),
        "child should still inherit color (not overridden)"
    );
}

#[test]
fn test_inherit_keyword_pulls_from_parent() {
    let parent_css = "div { font-size: 24px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { font-size: inherit; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.font_size,
        Some(24.0),
        "inherit keyword should pull parent's value"
    );
}

#[test]
fn test_non_inheritable_properties_not_inherited() {
    let parent_css = "div { margin-top: 10px; padding: 5px; border-width: 2px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.margin_top, None,
        "margin should NOT inherit (initial is None)"
    );
    assert_eq!(
        child.padding_top,
        Some(0.0),
        "padding should NOT inherit (stays initial 0.0)"
    );
    assert_eq!(
        child.border_top_width,
        Some(0.0),
        "border-width should NOT inherit (stays initial 0.0)"
    );
}

#[test]
fn test_root_element_gets_initial_values_when_no_parent() {
    let css = "div { }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    let initial = ComputedStyle::default_style();
    assert_eq!(
        style.font_size, initial.font_size,
        "root should get initial font-size"
    );
}

#[test]
fn test_apply_inheritance_fills_none_fields() {
    let mut child = ComputedStyle::default_style();
    child.color = None;
    child.font_size = None;
    child.font_weight = None;

    let mut parent = ComputedStyle::default_style();
    parent.color = Some(aether_css::Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    });
    parent.font_size = Some(20.0);
    parent.font_weight = Some("bold".into());

    apply_inheritance(
        &mut child,
        Some(&parent),
        InheritMask::default(),
        InitialMask::default(),
        InheritMask::default(),
    );

    assert!(child.color.is_some(), "color should be inherited");
    assert_eq!(child.font_size, Some(20.0), "font_size should be inherited");
    assert_eq!(
        child.font_weight.as_deref(),
        Some("bold"),
        "font_weight should be inherited"
    );
}

#[test]
fn test_apply_inheritance_uses_initial_when_no_parent() {
    let mut child = ComputedStyle::default_style();
    child.color = None;
    child.font_size = None;

    apply_inheritance(
        &mut child,
        None,
        InheritMask::default(),
        InitialMask::default(),
        InheritMask::default(),
    );

    let initial = ComputedStyle::default_style();
    assert_eq!(
        child.font_size, initial.font_size,
        "should use initial value when no parent"
    );
}

#[test]
fn test_line_height_child_override_preserves_explicit_value() {
    let parent_css = "div { line-height: 20px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { line-height: 1.5; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.line_height,
        Some(1.5),
        "child's explicit line-height should NOT be overwritten by parent"
    );
}

#[test]
fn test_line_height_inherit_keyword_pulls_from_parent() {
    let parent_css = "div { line-height: 20px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { line-height: inherit; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.line_height,
        Some(20.0),
        "inherit keyword should copy parent's line-height"
    );
}

#[test]
fn test_line_height_no_parent_gets_explicit_value() {
    let css = "div { line-height: 24px; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(
        style.line_height,
        Some(24.0),
        "line-height without parent should be the explicit value"
    );
}

// ══════════════════════════════════════════════════════════════
// 9. CSS `initial` and `unset` keyword semantics
// ══════════════════════════════════════════════════════════════

#[test]
fn test_initial_color_ignores_parent() {
    let parent_css = "div { color: red; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { color: initial; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    let initial = ComputedStyle::default_style();
    assert_eq!(
        child.color, initial.color,
        "color: initial should reset to initial value, not inherit red"
    );
}

#[test]
fn test_unset_color_inherits_from_parent() {
    let parent_css = "div { color: red; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { color: unset; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.color, parent.color,
        "color: unset on inheritable property should inherit from parent"
    );
}

#[test]
fn test_initial_font_size_ignores_parent() {
    let parent_css = "div { font-size: 24px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { font-size: initial; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    let initial = ComputedStyle::default_style();
    assert_eq!(
        child.font_size, initial.font_size,
        "font-size: initial should reset to initial value"
    );
}

#[test]
fn test_unset_font_size_inherits_from_parent() {
    let parent_css = "div { font-size: 24px; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { font-size: unset; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.font_size, parent.font_size,
        "font-size: unset should inherit from parent"
    );
}

#[test]
fn test_initial_margin_uses_initial_value() {
    let css = "div { margin: initial; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(
        style.margin_top, None,
        "margin: initial should use CSS initial value (auto/None)"
    );
}

#[test]
fn test_unset_margin_uses_initial_value() {
    let css = "div { margin: unset; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(
        style.margin_top, None,
        "margin: unset on non-inheritable should use initial value"
    );
}

#[test]
fn test_initial_padding_uses_initial_value() {
    let css = "div { padding: initial; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(
        style.padding_top,
        Some(0.0),
        "padding: initial should use CSS initial value (0)"
    );
}

#[test]
fn test_root_inherit_uses_initial() {
    let css = "div { color: inherit; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    let initial = ComputedStyle::default_style();
    assert_eq!(
        style.color, initial.color,
        "root element with inherit should use initial value"
    );
}

#[test]
fn test_root_unset_uses_initial() {
    let css = "div { color: unset; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    let initial = ComputedStyle::default_style();
    assert_eq!(
        style.color, initial.color,
        "root element with unset should use initial value"
    );
}

#[test]
fn test_explicit_value_overrides_initial() {
    let css = "div { color: blue; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    let blue = Color::from_named("blue").unwrap();
    assert_eq!(
        style.color,
        Some(blue),
        "explicit color should override initial"
    );
}

#[test]
fn test_initial_display_uses_initial_value() {
    let css = "div { display: initial; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(
        style.display,
        Display::Inline,
        "display: initial should use CSS initial value (inline)"
    );
}

#[test]
fn test_unset_display_uses_initial_value() {
    let css = "div { display: unset; }";
    let (style, _) = resolve_with_parent(css, "div", &[], &CustomPropertyMap::new());
    assert_eq!(
        style.display,
        aether_css::Display::Inline,
        "display: unset on non-inheritable should use initial value"
    );
}

#[test]
fn test_initial_visibility_ignores_parent() {
    let parent_css = "div { visibility: hidden; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { visibility: initial; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    let initial = ComputedStyle::default_style();
    assert_eq!(
        child.visibility, initial.visibility,
        "visibility: initial should reset to initial value, not inherit hidden"
    );
}

#[test]
fn test_unset_visibility_inherits_from_parent() {
    let parent_css = "div { visibility: hidden; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { visibility: unset; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.visibility, parent.visibility,
        "visibility: unset should inherit from parent"
    );
}

#[test]
fn test_inherit_still_works_after_initial_unset() {
    let parent_css = "div { color: green; }";
    let (parent, _) = resolve_with_parent(parent_css, "div", &[], &CustomPropertyMap::new());
    let child_css = "span { color: inherit; }";
    let (child, _) = resolve_with_parent_computed(
        child_css,
        "span",
        &[],
        &CustomPropertyMap::new(),
        Some(&parent),
    );
    assert_eq!(
        child.color, parent.color,
        "inherit keyword should still work correctly"
    );
}
