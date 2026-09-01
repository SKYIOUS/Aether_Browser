//! Stratus CSS Parser
//! Zero-copy string parser for CSS stylesheets

use super::style_value::{Color, LengthValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
    Composite(Vec<SimpleSelector>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
    pub attribute: Option<(String, String)>,
    pub pseudo_class: Option<String>,
}

impl SimpleSelector {
    pub fn new() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
            attribute: None,
            pseudo_class: None,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Number(f32),
    Keyword(String),
    Length(LengthValue),
    Color(Color),
    Shorthand(Vec<PropertyValue>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stylesheet_constructs() {
        let sheet = Stylesheet { rules: vec![] };
        assert!(sheet.rules.is_empty());
    }

    #[test]
    fn test_selector_simple() {
        let sel = SimpleSelector {
            tag_name: Some("div".into()),
            id: None,
            class: vec![],
            attribute: None,
            pseudo_class: None,
        };
        assert!(matches!(
            sel,
            SimpleSelector {
                tag_name: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn test_property_value_color() {
        let pv = PropertyValue::Color(Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        });
        assert!(matches!(pv, PropertyValue::Color(c) if c.r == 255));
    }
}
