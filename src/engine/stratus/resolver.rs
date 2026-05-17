//! Stratus Style Resolver
//! CSS Cascade and style computation

use super::matcher::{match_rules, ElementData, Specificity};
use super::parser::{Declaration, PropertyValue, Stylesheet};
use super::style_value::{
    AlignItems, AlignSelf, Color, ComputedStyle, Display, FlexDirection, FlexWrap,
    JustifyContent, Position, Transform, Transition,
};

pub fn resolve_style(element: &ElementData, stylesheet: &Stylesheet) -> ComputedStyle {
    let mut style = ComputedStyle::default_style();

    let matched = match_rules(element, stylesheet);

    for (declarations, specificity) in matched {
        apply_declarations(&mut style, declarations, specificity);
    }

    style
}

fn apply_declarations(style: &mut ComputedStyle, declarations: &[Declaration], _specificity: Specificity) {
    for decl in declarations {
        match decl.name.as_str() {
            "color" => style.color = parse_color(&decl.value),
            "background" => style.background_color = parse_background(&decl.value),
            "background-color" => style.background_color = parse_color(&decl.value),
            "font-size" => style.font_size = parse_length(&decl.value),
            "font-weight" => style.font_weight = parse_keyword(&decl.value),
            "font-family" => style.font_family = parse_keyword(&decl.value),
            "text-align" => style.text_align = parse_keyword(&decl.value),
            "display" => style.display = parse_display(&decl.value),
            "position" => style.position = parse_position(&decl.value),
            "overflow" => style.overflow = parse_keyword(&decl.value),
            "visibility" => style.visibility = parse_keyword(&decl.value),
            "opacity" => style.opacity = parse_length(&decl.value).map(|v| v / 100.0),
            "z-index" => style.z_index = parse_length(&decl.value).map(|v| v as i32),

            "margin" | "margin-top" | "margin-right" | "margin-bottom" | "margin-left" => {
                apply_sides(&mut style.margin_top, &mut style.margin_right, &mut style.margin_bottom, &mut style.margin_left, &decl.name, &decl.value);
            }
            "padding" | "padding-top" | "padding-right" | "padding-bottom" | "padding-left" => {
                apply_sides(&mut style.padding_top, &mut style.padding_right, &mut style.padding_bottom, &mut style.padding_left, &decl.name, &decl.value);
            }
            "border-width" | "border-top-width" | "border-right-width" | "border-bottom-width" | "border-left-width" => {
                apply_sides(&mut style.border_top_width, &mut style.border_right_width, &mut style.border_bottom_width, &mut style.border_left_width, &decl.name, &decl.value);
            }
            "border-color" | "border-top-color" | "border-right-color" | "border-bottom-color" | "border-left-color" => {
                apply_border_colors(&mut style.border_top_color, &mut style.border_right_color, &mut style.border_bottom_color, &mut style.border_left_color, &decl.name, &decl.value);
            }

            "width" => style.width = parse_length(&decl.value),
            "height" => style.height = parse_length(&decl.value),
            "min-width" => style.min_width = parse_length(&decl.value),
            "min-height" => style.min_height = parse_length(&decl.value),
            "max-width" => style.max_width = parse_length(&decl.value),
            "max-height" => style.max_height = parse_length(&decl.value),
            "top" => style.top = parse_length(&decl.value),
            "right" => style.right = parse_length(&decl.value),
            "bottom" => style.bottom = parse_length(&decl.value),
            "left" => style.left = parse_length(&decl.value),

            "flex-direction" => style.flex.flex_direction = parse_flex_direction(&decl.value),
            "flex-wrap" => style.flex.flex_wrap = parse_flex_wrap(&decl.value),
            "justify-content" => style.flex.justify_content = parse_justify_content(&decl.value),
            "align-items" => style.flex.align_items = parse_align_items(&decl.value),
            "align-self" => style.flex.align_self = parse_align_self(&decl.value),
            "flex-grow" => style.flex.flex_grow = parse_length(&decl.value).unwrap_or(0.0),
            "flex-shrink" => style.flex.flex_shrink = parse_length(&decl.value).unwrap_or(1.0),
            "flex-basis" => style.flex.flex_basis = parse_length(&decl.value),

            "transform" => style.transform = parse_transform(&decl.value),
            "transition" => style.transition = parse_transition(&decl.value),

            "box-sizing" => style.box_sizing = parse_keyword(&decl.value),

            _ => {}
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

fn parse_length(value: &PropertyValue) -> Option<f32> {
    match value {
        PropertyValue::Length(lv) => Some(lv.value),
        PropertyValue::Keyword(s) => s.parse().ok(),
        _ => None,
    }
}

fn parse_keyword(value: &PropertyValue) -> Option<String> {
    match value {
        PropertyValue::Keyword(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_display(value: &PropertyValue) -> Display {
    match value {
        PropertyValue::Keyword(s) => Display::from_str(s),
        _ => Display::Inline,
    }
}

fn parse_position(value: &PropertyValue) -> Position {
    match value {
        PropertyValue::Keyword(s) => Position::from_str(s),
        _ => Position::Static,
    }
}

fn parse_flex_direction(value: &PropertyValue) -> FlexDirection {
    match value {
        PropertyValue::Keyword(s) => FlexDirection::from_str(s),
        _ => FlexDirection::Row,
    }
}

fn parse_flex_wrap(value: &PropertyValue) -> FlexWrap {
    match value {
        PropertyValue::Keyword(s) => FlexWrap::from_str(s),
        _ => FlexWrap::NoWrap,
    }
}

fn parse_justify_content(value: &PropertyValue) -> JustifyContent {
    match value {
        PropertyValue::Keyword(s) => JustifyContent::from_str(s),
        _ => JustifyContent::FlexStart,
    }
}

fn parse_align_items(value: &PropertyValue) -> AlignItems {
    match value {
        PropertyValue::Keyword(s) => AlignItems::from_str(s),
        _ => AlignItems::Stretch,
    }
}

fn parse_align_self(value: &PropertyValue) -> AlignSelf {
    match value {
        PropertyValue::Keyword(s) => AlignSelf::from_str(s),
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

fn apply_sides(
    top: &mut Option<f32>,
    right: &mut Option<f32>,
    bottom: &mut Option<f32>,
    left: &mut Option<f32>,
    name: &str,
    value: &PropertyValue,
) {
    let len = parse_length(value);
    if len.is_none() { return; }
    let len = len.unwrap();

    match name {
        "margin-top" => *top = Some(len),
        "margin-right" => *right = Some(len),
        "margin-bottom" => *bottom = Some(len),
        "margin-left" => *left = Some(len),
        "margin" => {
            *top = Some(len);
            *right = Some(len);
            *bottom = Some(len);
            *left = Some(len);
        }
        "padding-top" => *top = Some(len),
        "padding-right" => *right = Some(len),
        "padding-bottom" => *bottom = Some(len),
        "padding-left" => *left = Some(len),
        "padding" => {
            *top = Some(len);
            *right = Some(len);
            *bottom = Some(len);
            *left = Some(len);
        }
        "border-top-width" => *top = Some(len),
        "border-right-width" => *right = Some(len),
        "border-bottom-width" => *bottom = Some(len),
        "border-left-width" => *left = Some(len),
        "border-width" => {
            *top = Some(len);
            *right = Some(len);
            *bottom = Some(len);
            *left = Some(len);
        }
        _ => {}
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
    let color = parse_color(value);
    if color.is_none() { return; }
    let color = color.unwrap();

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