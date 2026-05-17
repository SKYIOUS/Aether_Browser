#[derive(Debug, Clone)]
pub enum NodeType {
    Document,
    Text(String),
    Comment(String),
    Element(ElementData),
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

impl Node {
    pub fn new_document() -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Document,
        }
    }

    pub fn new_text(text: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(text),
        }
    }

    pub fn new_comment(comment: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Comment(comment),
        }
    }

    pub fn new_element(tag_name: String, attributes: std::collections::HashMap<String, String>, children: Vec<Node>) -> Self {
        Node {
            children,
            node_type: NodeType::Element(ElementData { tag_name, attributes }),
        }
    }

    pub fn is_element(&self) -> bool {
        matches!(self.node_type, NodeType::Element(_))
    }

    pub fn is_text(&self) -> bool {
        matches!(self.node_type, NodeType::Text(_))
    }

    pub fn is_comment(&self) -> bool {
        matches!(self.node_type, NodeType::Comment(_))
    }

    pub fn is_document(&self) -> bool {
        matches!(self.node_type, NodeType::Document)
    }

    pub fn tag_name(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Element(data) => Some(&data.tag_name),
            _ => None,
        }
    }

    pub fn text_content(&self) -> String {
        match &self.node_type {
            NodeType::Text(s) => s.clone(),
            NodeType::Element(_) => {
                self.children.iter()
                    .map(|c| c.text_content())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            _ => String::new(),
        }
    }

    pub fn get_elements_by_tag_name(&self, tag: &str) -> Vec<&Node> {
        let mut results = Vec::new();
        self.collect_by_tag(tag, &mut results);
        results
    }

    fn collect_by_tag<'a>(&'a self, tag: &str, results: &mut Vec<&'a Node>) {
        if let NodeType::Element(data) = &self.node_type {
            if data.tag_name.to_lowercase() == tag.to_lowercase() {
                results.push(self);
            }
        }
        for child in &self.children {
            child.collect_by_tag(tag, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let text_node = Node::new_text("Hello".to_string());
        assert!(matches!(text_node.node_type, NodeType::Text(_)));
    }

    #[test]
    fn test_element_tag_name() {
        let node = Node::new_element("div".to_string(), std::collections::HashMap::new(), vec![]);
        assert_eq!(node.tag_name(), Some("div"));
    }
}