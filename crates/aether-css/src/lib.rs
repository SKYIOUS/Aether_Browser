#![allow(dead_code)]
pub mod matcher;
pub mod parser;
pub mod property_names;
pub mod resolver;
pub mod style_value;

pub use matcher::{match_element, match_rules, ElementData, Specificity};
pub use parser::{Declaration, PropertyValue, Rule, Selector, SimpleSelector, Stylesheet};
pub use property_names::CssPropertyName;
pub use resolver::{resolve_style, resolve_style_vp, resolve_styles_for_tree};
pub use style_value::{
    AlignItems, AlignSelf, Color, ComputedStyle, Display, FlexDirection, FlexOptions,
    FlexWrap, JustifyContent, LengthValue, Position, Transform, Transition, Unit,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        let stylesheet = Stylesheet {
            rules: vec![
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: Some("body".into()),
                        id: None,
                        class: vec![],
                        attribute: None,
                        pseudo_class: None,
                    })],
                    declarations: vec![
                        Declaration { name: "margin".into(), value: PropertyValue::Number(0.0) },
                        Declaration { name: "padding".into(), value: PropertyValue::Number(0.0) },
                    ],
                },
                Rule {
                    selectors: vec![Selector::Simple(SimpleSelector {
                        tag_name: Some("div".into()),
                        id: None,
                        class: vec!["container".into()],
                        attribute: None,
                        pseudo_class: None,
                    })],
                    declarations: vec![
                        Declaration { name: "display".into(), value: PropertyValue::Keyword("flex".into()) },
                        Declaration { name: "flex-direction".into(), value: PropertyValue::Keyword("column".into()) },
                    ],
                },
            ],
        };
        assert_eq!(stylesheet.rules.len(), 2);

        let mut attrs = std::collections::HashMap::new();
        attrs.insert("class".to_string(), "container".to_string());
        let element = ElementData::with_attributes("div".to_string(), attrs);
        let style = resolve_style(&element, &stylesheet);

        assert_eq!(style.display, Display::Flex);
        assert_eq!(style.flex.flex_direction, FlexDirection::Column);
    }

    #[test]
    fn test_empty_css() {
        let stylesheet = Stylesheet { rules: vec![] };
        assert!(stylesheet.rules.is_empty());

        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.display, Display::Inline);
    }
}
