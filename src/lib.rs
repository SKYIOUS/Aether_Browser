pub mod engine;
pub mod ui;
pub mod logging;

#[cfg(test)]
mod tests {
    use crate::engine::parser::Parser;
    use crate::engine::dom::NodeType;

    #[test]
    fn test_browser_integration() {
        let html = "<div>Hello</div>".to_string();
        let mut parser = Parser::new(html);
        let node = parser.parse_node();
        
        if let NodeType::Element(data) = node.node_type {
            assert_eq!(data.tag_name, "div");
            assert_eq!(node.children.len(), 1);
        } else {
            panic!("Parsing failed");
        }
    }
}
