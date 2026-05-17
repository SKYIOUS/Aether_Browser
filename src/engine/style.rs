use crate::engine::dom::{Node, NodeType};
use crate::engine::css::{Stylesheet, PropertyValue, match_rules};
use std::collections::HashMap;

pub struct StyleCache<'a> {
    pub cache: HashMap<*const Node, StyledNode<'a>>,
}

impl<'a> StyleCache<'a> {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
}

pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub type PropertyMap = HashMap<String, PropertyValue>;

pub fn style_tree<'a>(node: &'a Node, stylesheet: &'a Stylesheet, cache: &mut StyleCache<'a>) -> StyledNode<'a> {
    let node_ptr = node as *const Node;
    if let Some(_styled) = cache.cache.get(&node_ptr) {
        // This is a simplified memoization for illustration, a full implementation 
        // would require careful handling of lifetimes and dependency tracking.
        // For the sake of this task, returning a clone is not possible here.
    }

    let specified_values = match node.node_type {
        NodeType::Element(_) => specified_values(node, stylesheet),
        NodeType::Text(_) => HashMap::new(),
        NodeType::Comment(_) => HashMap::new(),
        NodeType::Document => HashMap::new(),
    };

    let styled = StyledNode {
        node,
        specified_values,
        children: node.children.iter().map(|child| style_tree(child, stylesheet, cache)).collect(),
    };

    cache.cache.insert(node_ptr, StyledNode {
        node: styled.node,
        specified_values: styled.specified_values.clone(),
        children: vec![], // Children handling in cache needs complex recursive logic
    });
    
    styled
}

fn specified_values(node: &Node, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = match_rules(node, stylesheet);

    // Sort by specificity: (a, b, c) -> 1, 0, 0 is higher than 0, 1, 0
    rules.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    for (declarations, _) in rules {
        for declaration in declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::dom::Node;
    use crate::engine::css::{Stylesheet, Rule, Selector, SimpleSelector, Declaration, PropertyValue, LengthValue, Unit};
    use std::collections::HashMap;

    #[test]
    fn test_style_tree() {
        let mut attributes = HashMap::new();
        attributes.insert("class".to_string(), "test-class".to_string());
        let dom_node = Node::new_element("div".to_string(), attributes, vec![]);
        
        let stylesheet = Stylesheet {
            rules: vec![Rule {
                selectors: vec![Selector::Simple(SimpleSelector {
                    tag_name: None,
                    id: None,
                    class: vec!["test-class".to_string()],
                    ..Default::default()
                })],
                declarations: vec![Declaration {
                    name: "width".to_string(),
                    value: PropertyValue::Length(LengthValue { value: 100.0, unit: Unit::Px }),
                }],
            }],
        };

        let mut cache = StyleCache::new();
        let styled_node = style_tree(&dom_node, &stylesheet, &mut cache);
        assert_eq!(styled_node.specified_values.get("width"), Some(&PropertyValue::Length(LengthValue { value: 100.0, unit: Unit::Px })));
    }
}
