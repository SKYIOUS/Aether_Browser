#![allow(dead_code)]
pub mod tokenizer;
pub mod tree_builder;

use aether_dom::Node;
use std::collections::HashMap;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser { pos: 0, input }
    }

    pub fn parse_document(&mut self) -> Node {
        if self.starts_with("<!DOCTYPE") || self.starts_with("<!doctype") {
            self.consume_while(|c| c != '>');
            if self.next_char() == '>' {
                self.consume_char();
            }
        }

        if self.starts_with("<html") || self.starts_with("<HTML") {
            self.parse_element();
        }

        let children = self.parse_nodes();

        let mut doc_node = Node::new_document();
        doc_node.children = children;
        doc_node
    }

    pub fn parse_node(&mut self) -> Node {
        self.consume_whitespace();

        if self.eof() {
            return Node::new_text(String::new());
        }

        match self.next_char() {
            '<' => {
                if self.input[self.pos..].starts_with("<!--") {
                    return self.parse_comment();
                }
                if self.input[self.pos..].starts_with("<![CDATA[") {
                    return self.parse_cdata();
                }
                self.parse_element()
            }
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Node {
        let text = self.consume_while(|c| c != '<');
        let text = text.trim();
        if text.is_empty() {
            Node::new_text(String::new())
        } else {
            Node::new_text(text.to_string())
        }
    }

    fn parse_comment(&mut self) -> Node {
        self.pos += 4;
        let mut content = String::new();
        while !self.eof() {
            if self.input[self.pos..].starts_with("-->") {
                self.pos += 3;
                break;
            }
            content.push(self.consume_char());
        }
        Node::new_comment(content)
    }

    fn parse_cdata(&mut self) -> Node {
        self.pos += 9;
        let mut content = String::new();
        while !self.eof() {
            if self.input[self.pos..].starts_with("]]>") {
                self.pos += 3;
                break;
            }
            content.push(self.consume_char());
        }
        Node::new_text(content)
    }

    fn parse_element(&mut self) -> Node {
        let _ = self.consume_char();

        if self.next_char() == '/' {
            return Node::new_text(String::new());
        }

        let tag_name = self.consume_while(|c| c != '>' && !c.is_whitespace()).to_lowercase();

        if tag_name == "!doctype" || tag_name == "!DOCTYPE" {
            self.consume_while(|c| c != '>');
            if self.next_char() == '>' {
                self.consume_char();
            }
            return self.parse_node();
        }

        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            let c = self.next_char();
            if c == '>' || c == '/' { break; }
            if c == '\0' || self.eof() { break; }

            let attr_name = self.consume_while(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':');
            if attr_name.is_empty() { break; }

            self.consume_whitespace();
            let next_c = self.next_char();
            if next_c == '=' {
                self.consume_char();
                self.consume_whitespace();
                let quote = self.next_char();
                let value = if quote == '"' || quote == '\'' {
                    self.consume_char();
                    let v = self.consume_while(|c| c != quote);
                    self.consume_char();
                    v
                } else {
                    self.consume_while(|c| !c.is_whitespace() && c != '>')
                };
                attributes.insert(attr_name, value);
            } else {
                attributes.insert(attr_name, "".to_string());
            }
        }

        if self.next_char() == '/' {
            self.consume_char();
        }
        if self.next_char() == '>' {
            self.consume_char();
        } else {
            self.consume_while(|c| c != '>');
            if self.next_char() == '>' {
                self.consume_char();
            }
        }

        let self_closing = ["meta", "link", "br", "img", "input", "hr", "area", "base", "col", "embed", "source", "track", "wbr", "basefont", "frame", "param"];

        let children = if self_closing.contains(&tag_name.as_str()) {
            vec![]
        } else if tag_name == "script" || tag_name == "style" || tag_name == "title" || tag_name == "textarea" || tag_name == "pre" {
            let content = self.consume_until_closing_tag(&tag_name);
            vec![Node::new_text(content)]
        } else {
            self.parse_nodes()
        };

        if !self_closing.contains(&tag_name.as_str()) && self.starts_with("</") {
            self.pos += 2;
            let _closing_tag = self.consume_while(|c| c != '>');
            if self.next_char() == '>' {
                self.consume_char();
            }
        }

        Node::new_element(tag_name, attributes, children)
    }

    fn consume_until_closing_tag(&mut self, tag_name: &str) -> String {
        let closing = format!("</{}", tag_name);
        let closing_upper = format!("</{}", tag_name.to_uppercase());
        let mut content = String::new();
        while !self.eof() {
            if self.starts_with(&closing) || self.starts_with(&closing_upper) {
                break;
            }
            content.push(self.consume_char());
        }
        content.trim().to_string()
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() { break; }
            if self.starts_with("</") { break; }

            let node = self.parse_node();
            match &node.node_type {
                aether_dom::NodeType::Text(t) if t.is_empty() => continue,
                aether_dom::NodeType::Text(_) => nodes.push(node),
                aether_dom::NodeType::Comment(_) => continue,
                _ => nodes.push(node),
            }
        }
        nodes
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_element() {
        let mut parser = Parser::new("<div>Hello</div>".to_string());
        let node = parser.parse_node();
        if let aether_dom::NodeType::Element(data) = node.node_type {
            assert_eq!(data.tag_name, "div");
            assert_eq!(node.children.len(), 1);
            if let aether_dom::NodeType::Text(text) = &node.children[0].node_type {
                assert_eq!(text, "Hello");
            } else {
                panic!("Expected text node child");
            }
        } else {
            panic!("Expected element node");
        }
    }
}
