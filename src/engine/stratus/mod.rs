//! Stratus CSSOM Module
//! Aether Browser's custom CSSOM and Style Resolver engine
//!
//! This module provides:
//! - CSS parsing (parser.rs) - converts raw CSS strings to structured AST
//! - Style values (style_value.rs) - strongly-typed CSS property representations
//! - Selector matching (matcher.rs) - DOM element to CSS rule matching
//! - Style resolution (resolver.rs) - CSS cascade and computed style computation
//!
//! ## Architecture
//!
//! ```text
//! Raw CSS String → parser::parse() → Stylesheet
//!                        ↓
//!              matcher::match_element() → Vec<MatchedRule>
//!                        ↓
//!              resolver::resolve_style() → ComputedStyle
//!                        ↓
//!                       Caelum (Layout)
//! ```

pub mod matcher;
pub mod parser;
pub mod resolver;
pub mod style_value;

pub use matcher::{match_element, match_rules, ElementData, Specificity};
pub use parser::{Declaration, parse, PropertyValue, Rule, Selector, SimpleSelector, Stylesheet};
pub use resolver::{resolve_style, resolve_styles_for_tree};
pub use style_value::{
    AlignItems, AlignSelf, Color, ComputedStyle, Display, FlexDirection, FlexOptions,
    FlexWrap, JustifyContent, LengthValue, Position, Transform, Transition, Unit,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        let css = r#"
            body { margin: 0; padding: 0; }
            .container { display: flex; flex-direction: column; }
            #header { background: #333; color: white; }
        "#;

        let stylesheet = parse(css);
        assert_eq!(stylesheet.rules.len(), 3);

        let mut attrs = std::collections::HashMap::new();
        attrs.insert("class".to_string(), "container".to_string());
        let element = ElementData::with_attributes("div".to_string(), attrs);
        let style = resolve_style(&element, &stylesheet);

        assert_eq!(style.display, Display::Flex);
        assert_eq!(style.flex.flex_direction, FlexDirection::Column);
    }

    #[test]
    fn test_empty_css() {
        let stylesheet = parse("");
        assert!(stylesheet.rules.is_empty());

        let element = ElementData::new("div".to_string());
        let style = resolve_style(&element, &stylesheet);
        assert_eq!(style.display, Display::Inline);
    }
}