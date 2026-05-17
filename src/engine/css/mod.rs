//! CSS parsing and style computation.
//! This module is a thin shim over the Stratus CSS engine for backward compatibility.

pub use crate::engine::stratus::{
    AlignItems, AlignSelf, Color, ComputedStyle, Declaration, Display, FlexDirection,
    FlexOptions, FlexWrap, JustifyContent, LengthValue, Position, PropertyValue, Rule,
    Selector, SimpleSelector, Specificity, Stylesheet, Transform, Transition, Unit,
    resolve_style,
};

pub use crate::engine::stratus::parse as stratus_parse;

/// Matched rule type for backward compatibility.
pub type MatchedRule<'a> = (&'a Selector, &'a Vec<Declaration>);

/// Parser wrapper that delegates to the Stratus engine.
pub struct Parser {
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser { input }
    }

    pub fn parse_rules(&mut self) -> Vec<Rule> {
        let stylesheet = stratus_parse(&self.input);
        stylesheet.rules
    }
}

/// Match CSS rules against a DOM node by converting it to ElementData.
pub fn match_rules<'a>(
    node: &crate::engine::dom::Node,
    stylesheet: &'a Stylesheet,
) -> Vec<(&'a Vec<Declaration>, Specificity)> {
    use crate::engine::stratus::ElementData;
    let element = match &node.node_type {
        crate::engine::dom::NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    crate::engine::stratus::match_rules(&element, stylesheet)
}

/// Compute style for a DOM node.
pub fn compute_style(
    node: &crate::engine::dom::Node,
    stylesheet: &Stylesheet,
) -> ComputedStyle {
    use crate::engine::stratus::ElementData;
    let element = match &node.node_type {
        crate::engine::dom::NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    resolve_style(&element, stylesheet)
}

/// Compute styles for an entire DOM tree.
pub fn compute_styles_for_tree(
    node: &crate::engine::dom::Node,
    stylesheet: &Stylesheet,
    results: &mut std::collections::HashMap<u64, ComputedStyle>,
) {
    for child in &node.children {
        let key = child as *const _ as u64;
        let style = compute_style(child, stylesheet);
        results.insert(key, style);
        compute_styles_for_tree(child, stylesheet, results);
    }
}
