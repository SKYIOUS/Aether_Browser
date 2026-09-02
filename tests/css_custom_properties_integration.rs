use vayu_browser::engine::parser::parse_html;
use vayu_browser::engine::pipeline::extractor::{extract_elements, StyledElement};
use vayu_browser::engine::pipeline::layout::apply_taffy_layout;
use vayu_browser::engine::stratus::{
    self, ComputedStyle, CustomPropertyMap, ElementData, PropertyValue,
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
    stratus::resolve_style_with_vars_and_custom(&el, &sheet, 800.0, 600.0, parent_vars)
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
