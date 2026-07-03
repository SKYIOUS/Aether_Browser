//! Stratus Style Resolver
//! CSS Cascade and style computation

use super::matcher::{match_rules, ElementData, Specificity};
use super::parser::{Declaration, PropertyValue, Stylesheet};
use super::style_value::{
    AlignItems, AlignSelf, Color, ComputedStyle, Display, FlexDirection, FlexWrap,
    JustifyContent, LengthValue, Position, Transform, Transition, Unit,
};

pub fn resolve_style(element: &ElementData, stylesheet: &Stylesheet) -> ComputedStyle {
    resolve_style_vp(element, stylesheet, 800.0, 600.0)
}

pub fn resolve_style_vp(element: &ElementData, stylesheet: &Stylesheet, viewport_w: f32, viewport_h: f32) -> ComputedStyle {
    let mut style = ComputedStyle::default_style();

    let matched = match_rules(element, stylesheet);

    for (declarations, specificity) in matched {
        apply_declarations_vp(&mut style, declarations, specificity, viewport_w, viewport_h);
    }

    style
}

#[allow(dead_code)]
fn apply_declarations(style: &mut ComputedStyle, declarations: &[Declaration], spec: Specificity) {
    apply_declarations_vp(style, declarations, spec, 800.0, 600.0)
}

fn apply_declarations_vp(style: &mut ComputedStyle, declarations: &[Declaration], _specificity: Specificity, viewport_w: f32, viewport_h: f32) {
    use super::property_names::CssPropertyName;
    use std::str::FromStr;

    for decl in declarations {
        let vw = viewport_w;
        let vh = viewport_h;
        if let Ok(prop) = CssPropertyName::from_str(&decl.name) {
            match prop {
                CssPropertyName::Color => style.color = parse_color(&decl.value),
                CssPropertyName::Background => style.background_color = parse_background(&decl.value),
                CssPropertyName::BackgroundColor => style.background_color = parse_color(&decl.value),
                CssPropertyName::FontSize => style.font_size = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::FontWeight => style.font_weight = parse_keyword(&decl.value),
                CssPropertyName::FontFamily => style.font_family = parse_keyword(&decl.value),
                CssPropertyName::TextAlign => style.text_align = parse_keyword(&decl.value),
                CssPropertyName::Display => style.display = parse_display(&decl.value),
                CssPropertyName::Position => style.position = parse_position(&decl.value),
                CssPropertyName::Overflow => style.overflow = parse_keyword(&decl.value),
                CssPropertyName::Visibility => style.visibility = parse_keyword(&decl.value),
                CssPropertyName::Opacity => style.opacity = parse_keyword(&decl.value).and_then(|v| v.parse::<f32>().ok()),
                CssPropertyName::ZIndex => style.z_index = parse_length_vp(&decl.value, vw, vh).map(|v| v as i32),

                CssPropertyName::Margin | CssPropertyName::MarginTop | CssPropertyName::MarginRight | CssPropertyName::MarginBottom | CssPropertyName::MarginLeft => {
                    apply_sides_vp(&mut style.margin_top, &mut style.margin_right, &mut style.margin_bottom, &mut style.margin_left, &decl.name, &decl.value, vw, vh);
                }
                CssPropertyName::Padding | CssPropertyName::PaddingTop | CssPropertyName::PaddingRight | CssPropertyName::PaddingBottom | CssPropertyName::PaddingLeft => {
                    apply_sides_vp(&mut style.padding_top, &mut style.padding_right, &mut style.padding_bottom, &mut style.padding_left, &decl.name, &decl.value, vw, vh);
                }
                CssPropertyName::BorderWidth | CssPropertyName::BorderTopWidth | CssPropertyName::BorderRightWidth | CssPropertyName::BorderBottomWidth | CssPropertyName::BorderLeftWidth => {
                    apply_sides_vp(&mut style.border_top_width, &mut style.border_right_width, &mut style.border_bottom_width, &mut style.border_left_width, &decl.name, &decl.value, vw, vh);
                }
                CssPropertyName::BorderColor | CssPropertyName::BorderTopColor | CssPropertyName::BorderRightColor | CssPropertyName::BorderBottomColor | CssPropertyName::BorderLeftColor => {
                    apply_border_colors(&mut style.border_top_color, &mut style.border_right_color, &mut style.border_bottom_color, &mut style.border_left_color, &decl.name, &decl.value);
                }

                CssPropertyName::Width => style.width = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::Height => style.height = parse_length_vp_vertical(&decl.value, vw, vh),
                CssPropertyName::MinWidth => style.min_width = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::MinHeight => style.min_height = parse_length_vp_vertical(&decl.value, vw, vh),
                CssPropertyName::MaxWidth => style.max_width = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::MaxHeight => style.max_height = parse_length_vp_vertical(&decl.value, vw, vh),
                CssPropertyName::Top => style.top = parse_length_vp_vertical(&decl.value, vw, vh),
                CssPropertyName::Right => style.right = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::Bottom => style.bottom = parse_length_vp_vertical(&decl.value, vw, vh),
                CssPropertyName::Left => style.left = parse_length_vp(&decl.value, vw, vh),

                CssPropertyName::FlexDirection => style.flex.flex_direction = parse_flex_direction(&decl.value),
                CssPropertyName::FlexWrap => style.flex.flex_wrap = parse_flex_wrap(&decl.value),
                CssPropertyName::JustifyContent => style.flex.justify_content = parse_justify_content(&decl.value),
                CssPropertyName::AlignItems => style.flex.align_items = parse_align_items(&decl.value),
                CssPropertyName::AlignSelf => style.flex.align_self = parse_align_self(&decl.value),
                CssPropertyName::FlexGrow => style.flex.flex_grow = parse_length_vp(&decl.value, vw, vh).unwrap_or(0.0),
                CssPropertyName::FlexShrink => style.flex.flex_shrink = parse_length_vp(&decl.value, vw, vh).unwrap_or(1.0),
                CssPropertyName::FlexBasis => style.flex.flex_basis = parse_length_vp(&decl.value, vw, vh),

                CssPropertyName::Transform => style.transform = parse_transform(&decl.value),
                CssPropertyName::Transition => style.transition = parse_transition(&decl.value),

                CssPropertyName::BoxSizing => style.box_sizing = parse_keyword(&decl.value),

                CssPropertyName::LineHeight => style.line_height = parse_length_vp(&decl.value, vw, vh),
                CssPropertyName::TextDecoration => style.text_decoration = parse_keyword(&decl.value),
                CssPropertyName::Cursor => style.cursor = parse_keyword(&decl.value),
                CssPropertyName::BorderRadius => style.border_radius = parse_length_vp(&decl.value, vw, vh),
            }
        }
    }
}

fn parse_color(value: &PropertyValue) -> Option<Color> {
    match value {
        PropertyValue::Color(c) => Some(c.clone()),
        PropertyValue::Keyword(s) => Color::from_named(s),
        _ => None,
    }
}

fn parse_background(value: &PropertyValue) -> Option<Color> {
    parse_color(value)
}

// ── Viewport-resolution helpers ──
// These functions resolve vw/vh/percent units using the given viewport dimensions.
// parse_length is retained for backward compat (uses default 800×600 viewport).

fn lv_to_px(lv: &LengthValue, vw: f32, vh: f32) -> f32 {
    match lv.unit {
        Unit::Px => lv.value,
        Unit::Vw => lv.value * vw / 100.0,
        Unit::Vh => lv.value * vh / 100.0,
        Unit::Percent => lv.value * vw / 100.0,
        Unit::Em | Unit::Rem => lv.value * 16.0,
        _ => lv.value,
    }
}

fn parse_length(value: &PropertyValue) -> Option<f32> {
    parse_length_vp(value, 800.0, 600.0)
}

#[allow(dead_code)]
fn resolve_length_for_unit(lv: &LengthValue, vw: f32, vh: f32) -> f32 {
    lv_to_px(lv, vw, vh)
}

fn parse_length_vp(value: &PropertyValue, vw: f32, vh: f32) -> Option<f32> {
    match value {
        PropertyValue::Length(lv) => Some(lv_to_px(lv, vw, vh)),
        PropertyValue::Keyword(s) => s.parse().ok(),
        _ => None,
    }
}

fn parse_length_vp_vertical(value: &PropertyValue, vw: f32, vh: f32) -> Option<f32> {
    match value {
        PropertyValue::Length(lv) => Some(match lv.unit {
            Unit::Px => lv.value,
            Unit::Vw => lv.value * vw / 100.0,
            Unit::Vh => lv.value * vh / 100.0,
            Unit::Percent => lv.value * vh / 100.0,
            Unit::Em | Unit::Rem => lv.value * 16.0,
            _ => lv.value,
        }),
        PropertyValue::Keyword(s) => s.parse().ok(),
        _ => None,
    }
}

fn parse_side_shorthand_vp(value: &PropertyValue, vw: f32, vh: f32) -> Option<[Option<f32>; 4]> {
    let s = match value {
        PropertyValue::Keyword(s) => s,
        _ => return None,
    };
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.is_empty() || parts.len() > 4 { return None; }

    let mut vals: Vec<Option<f32>> = Vec::with_capacity(parts.len());
    for p in &parts {
        if *p == "auto" { vals.push(None); continue; }
        if let Some(lv) = LengthValue::from_str(p) {
            vals.push(Some(lv_to_px(&lv, vw, vh)));
        } else if let Ok(n) = p.parse::<f32>() {
            vals.push(Some(n));
        } else { return None; }
    }

    Some(match vals.len() {
        1 => [vals[0], vals[0], vals[0], vals[0]],
        2 => [vals[0], vals[1], vals[0], vals[1]],
        3 => [vals[0], vals[1], vals[2], vals[1]],
        4 => [vals[0], vals[1], vals[2], vals[3]],
        _ => return None,
    })
}

fn parse_keyword(value: &PropertyValue) -> Option<String> {
    match value {
        PropertyValue::Keyword(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_display(value: &PropertyValue) -> Display {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => Display::Inline,
    }
}

fn parse_position(value: &PropertyValue) -> Position {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => Position::Static,
    }
}

fn parse_flex_direction(value: &PropertyValue) -> FlexDirection {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => FlexDirection::Row,
    }
}

fn parse_flex_wrap(value: &PropertyValue) -> FlexWrap {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => FlexWrap::NoWrap,
    }
}

fn parse_justify_content(value: &PropertyValue) -> JustifyContent {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => JustifyContent::FlexStart,
    }
}

fn parse_align_items(value: &PropertyValue) -> AlignItems {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => AlignItems::Stretch,
    }
}

fn parse_align_self(value: &PropertyValue) -> AlignSelf {
    match value {
        PropertyValue::Keyword(s) => s.parse().unwrap_or_default(),
        _ => AlignSelf::Auto,
    }
}

fn parse_transform(value: &PropertyValue) -> Option<Transform> {
    match value {
        PropertyValue::Keyword(s) => {
            let mut t = Transform::default();
            let s_lower = s.to_lowercase();

            if s_lower.contains("translate") {
                t.translate_x = Some(0.0);
                t.translate_y = Some(0.0);
            }
            if s_lower.contains("rotate") {
                t.rotate = Some(0.0);
            }
            if s_lower.contains("scale") {
                t.scale_x = Some(1.0);
                t.scale_y = Some(1.0);
            }

            if t.translate_x.is_some() || t.rotate.is_some() || t.scale_x != Some(1.0) {
                Some(t)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_transition(value: &PropertyValue) -> Option<Transition> {
    match value {
        PropertyValue::Keyword(s) => {
            let parts: Vec<&str> = s.split_whitespace().collect();
            Some(Transition {
                property: parts.first().map(|s| s.to_string()).unwrap_or_else(|| "all".to_string()),
                duration: parts.get(1).and_then(|v| v.trim_end_matches("s").parse().ok()).unwrap_or(0.3),
                timing_function: parts.get(2).map(|s| s.to_string()).unwrap_or_else(|| "ease".to_string()),
                delay: 0.0,
            })
        }
        _ => None,
    }
}

/// Parse a space-separated CSS shorthand (1-4 values) into side values.
fn apply_sides(
    top: &mut Option<f32>,
    right: &mut Option<f32>,
    bottom: &mut Option<f32>,
    left: &mut Option<f32>,
    name: &str,
    value: &PropertyValue,
) {
    apply_sides_vp(top, right, bottom, left, name, value, 800.0, 600.0)
}

fn apply_sides_vp(
    top: &mut Option<f32>,
    right: &mut Option<f32>,
    bottom: &mut Option<f32>,
    left: &mut Option<f32>,
    name: &str,
    value: &PropertyValue,
    vw: f32,
    vh: f32,
) {
    // First try single length
    if let Some(len) = parse_length_vp(value, vw, vh) {
        match name {
            "margin-top" => *top = Some(len),
            "margin-right" => *right = Some(len),
            "margin-bottom" => *bottom = Some(len),
            "margin-left" => *left = Some(len),
            "margin" => { *top = Some(len); *right = Some(len); *bottom = Some(len); *left = Some(len); }
            "padding-top" => *top = Some(len),
            "padding-right" => *right = Some(len),
            "padding-bottom" => *bottom = Some(len),
            "padding-left" => *left = Some(len),
            "padding" => { *top = Some(len); *right = Some(len); *bottom = Some(len); *left = Some(len); }
            "border-top-width" => *top = Some(len),
            "border-right-width" => *right = Some(len),
            "border-bottom-width" => *bottom = Some(len),
            "border-left-width" => *left = Some(len),
            "border-width" => { *top = Some(len); *right = Some(len); *bottom = Some(len); *left = Some(len); }
            _ => {}
        }
        return;
    }

    // Try shorthand (e.g. "5em auto" → [5em, auto, 5em, auto])
    if let Some(quads) = parse_side_shorthand_vp(value, vw, vh) {
        let is_shorthand = matches!(name, "margin" | "padding" | "border-width");
        if is_shorthand || name == "margin-top" || name == "padding-top" || name == "border-top-width" {
            if let Some(v) = quads[0] { *top = Some(v); }
        }
        if is_shorthand || name == "margin-right" || name == "padding-right" || name == "border-right-width" {
            if let Some(v) = quads[1] { *right = Some(v); }
        }
        if is_shorthand || name == "margin-bottom" || name == "padding-bottom" || name == "border-bottom-width" {
            if let Some(v) = quads[2] { *bottom = Some(v); }
        }
        if is_shorthand || name == "margin-left" || name == "padding-left" || name == "border-left-width" {
            if let Some(v) = quads[3] { *left = Some(v); }
        }
    }
}

fn apply_border_colors(
    top: &mut Option<Color>,
    right: &mut Option<Color>,
    bottom: &mut Option<Color>,
    left: &mut Option<Color>,
    name: &str,
    value: &PropertyValue,
) {
    let Some(color) = parse_color(value) else { return; };

    match name {
        "border-top-color" => *top = Some(color),
        "border-right-color" => *right = Some(color),
        "border-bottom-color" => *bottom = Some(color),
        "border-left-color" => *left = Some(color),
        "border-color" => {
            *top = Some(color.clone());
            *right = Some(color.clone());
            *bottom = Some(color.clone());
            *left = Some(color);
        }
        _ => {}
    }
}

pub fn resolve_styles_for_tree(
    element: &ElementData,
    stylesheet: &Stylesheet,
    results: &mut std::collections::HashMap<String, ComputedStyle>,
) {
    let style = resolve_style(element, stylesheet);
    results.insert(element.tag_name.clone(), style);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser::parse;

    #[test]
    fn test_resolve_simple() {
        let css = "div { color: red; }";
        let stylesheet = parse(css);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.color, Some(Color { r: 255, g: 0, b: 0, a: 255 }));
    }

    #[test]
    fn test_resolve_display() {
        let css = "div { display: flex; }";
        let stylesheet = parse(css);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.display, Display::Flex);
    }

    #[test]
    fn test_resolve_flex() {
        let css = "div { display: flex; flex-direction: column; justify-content: center; }";
        let stylesheet = parse(css);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.display, Display::Flex);
        assert_eq!(style.flex.flex_direction, FlexDirection::Column);
        assert_eq!(style.flex.justify_content, JustifyContent::Center);
    }

    #[test]
    fn test_cascade_override() {
        let css = "div { color: red; } div { color: blue; }";
        let stylesheet = parse(css);
        let element = ElementData::new("div".to_string());

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.color, Some(Color { r: 0, g: 0, b: 255, a: 255 }));
    }

    #[test]
    fn test_specificity_override() {
        let css = "div { color: red; } #id { color: blue; }";
        let stylesheet = parse(css);
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "id".to_string());
        let element = ElementData::with_attributes("div".to_string(), attrs);

        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.color, Some(Color { r: 0, g: 0, b: 255, a: 255 }));
    }
}
