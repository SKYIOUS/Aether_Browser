use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::{resolve_style, resolve_style_vp, ComputedStyle, ElementData};

pub fn compute_style(node: &Node, stylesheet: &crate::engine::stratus::Stylesheet) -> ComputedStyle {
    let element = match &node.node_type {
        NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    resolve_style(&element, stylesheet)
}

pub fn compute_style_vp(node: &Node, stylesheet: &crate::engine::stratus::Stylesheet, vw: f32, vh: f32) -> ComputedStyle {
    let element = match &node.node_type {
        NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    resolve_style_vp(&element, stylesheet, vw, vh)
}
