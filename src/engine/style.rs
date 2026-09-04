use crate::engine::dom::{Node, NodeType};
use crate::engine::stratus::{
    resolve_style, resolve_style_vp, resolve_style_with_vars_and_custom, ComputedStyle,
    CustomPropertyMap, ElementData,
};

pub fn compute_style(
    node: &Node,
    stylesheet: &crate::engine::stratus::Stylesheet,
) -> ComputedStyle {
    let element = match &node.node_type {
        NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    resolve_style(&element, stylesheet)
}

pub fn compute_style_vp(
    node: &Node,
    stylesheet: &crate::engine::stratus::Stylesheet,
    vw: f32,
    vh: f32,
) -> ComputedStyle {
    let element = match &node.node_type {
        NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    resolve_style_vp(&element, stylesheet, vw, vh)
}

pub fn compute_style_vp_with_vars(
    node: &Node,
    stylesheet: &crate::engine::stratus::Stylesheet,
    vw: f32,
    vh: f32,
    cb_w: f32,
    cb_h: f32,
    parent_vars: &CustomPropertyMap,
    parent_computed: Option<&ComputedStyle>,
) -> (ComputedStyle, CustomPropertyMap) {
    let element = match &node.node_type {
        NodeType::Element(elem) => {
            ElementData::with_attributes(elem.tag_name.clone(), elem.attributes.clone())
        }
        _ => ElementData::new("unknown".to_string()),
    };
    let (cs, custom, _mask) = resolve_style_with_vars_and_custom(
        &element,
        stylesheet,
        vw,
        vh,
        cb_w,
        cb_h,
        parent_vars,
        parent_computed,
    );
    (cs, custom)
}
