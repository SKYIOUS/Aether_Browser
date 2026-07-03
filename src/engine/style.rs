use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::{resolve_style, ComputedStyle, ElementData};

pub fn compute_style(node: &Node, stylesheet: &crate::engine::stratus::Stylesheet) -> ComputedStyle {
    let element = match &node.node_type {
        NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    resolve_style(&element, stylesheet)
}
