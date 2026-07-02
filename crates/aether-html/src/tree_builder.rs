use std::collections::HashMap;
use super::tokenizer::{Token, TagToken, DoctypeToken};
use aether_dom::{Node, NodeType, ElementData};

#[derive(Debug, Clone, PartialEq)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoScript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    Select,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

pub struct TreeBuilder {
    tokenizer: super::tokenizer::Tokenizer,
    pub document: Node,
    open_elements: Vec<Node>,
    insertion_mode: InsertionMode,
    original_insertion_mode: InsertionMode,
    foster_parenting: bool,
    scripting: bool,
    form_element: Option<usize>,
    frameset_ok: bool,
}

impl TreeBuilder {
    pub fn new(input: String) -> Self {
        Self {
            tokenizer: super::tokenizer::Tokenizer::new(input),
            document: Node::new_document(),
            open_elements: Vec::new(),
            insertion_mode: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            foster_parenting: false,
            scripting: true,
            form_element: None,
            frameset_ok: true,
        }
    }

    pub fn build(&mut self) -> Node {
        self.tokenizer.tokenize();
        let tokens = self.tokenizer.tokens.clone();
        for token in &tokens {
            self.process_token(token);
        }
        self.document.clone()
    }

    fn process_token(&mut self, token: &Token) {
        match self.insertion_mode {
            InsertionMode::Initial => self.handle_initial(token),
            InsertionMode::BeforeHtml => self.handle_before_html(token),
            InsertionMode::BeforeHead => self.handle_before_head(token),
            InsertionMode::InHead => self.handle_in_head(token),
            InsertionMode::AfterHead => self.handle_after_head(token),
            InsertionMode::InBody => self.handle_in_body(token),
            InsertionMode::Text => self.handle_text(token),
            InsertionMode::AfterBody => self.handle_after_body(token),
            InsertionMode::AfterAfterBody => self.handle_after_after_body(token),
            _ => {}
        }
    }

    fn current_node(&self) -> Option<&Node> {
        self.open_elements.last()
    }

    fn current_node_mut(&mut self) -> Option<&mut Node> {
        self.open_elements.last_mut()
    }

    fn insert_element(&mut self, tag: &str, attrs: HashMap<String, String>) {
        let el = Node::new_element(tag.to_string(), attrs, vec![]);
        if let Some(current) = self.open_elements.last_mut() {
            current.children.push(el.clone());
        } else {
            self.document.children.push(el.clone());
        }
        self.open_elements.push(el);
    }

    fn insert_character(&mut self, c: char) {
        let text = c.to_string();
        let text_node = Node::new_text(text);
        if self.foster_parenting {
            // TODO: proper foster parenting
            if let Some(current) = self.open_elements.last_mut() {
                current.children.push(text_node);
            } else {
                self.document.children.push(text_node);
            }
        } else {
            if let Some(current) = self.open_elements.last_mut() {
                current.children.push(text_node);
            } else {
                self.document.children.push(text_node);
            }
        }
    }

    fn handle_initial(&mut self, token: &Token) {
        match token {
            Token::Doctype(_) => {
                self.insertion_mode = InsertionMode::BeforeHtml;
            }
            _ => {
                self.insertion_mode = InsertionMode::BeforeHtml;
                self.process_token(token);
            }
        }
    }

    fn handle_before_html(&mut self, token: &Token) {
        match token {
            Token::StartTag(tag) if tag.name == "html" => {
                self.insert_element("html", tag.attrs.clone());
                self.insertion_mode = InsertionMode::BeforeHead;
            }
            Token::EndTag(_) => {}
            _ => {
                self.insert_element("html", HashMap::new());
                self.insertion_mode = InsertionMode::BeforeHead;
                self.process_token(token);
            }
        }
    }

    fn handle_before_head(&mut self, token: &Token) {
        match token {
            Token::StartTag(tag) if tag.name == "head" => {
                self.insert_element("head", tag.attrs.clone());
                self.insertion_mode = InsertionMode::InHead;
            }
            Token::EndTag(_) => {}
            _ => {
                self.insert_element("head", HashMap::new());
                self.insertion_mode = InsertionMode::InHead;
                self.process_token(token);
            }
        }
    }

    fn handle_in_head(&mut self, token: &Token) {
        match token {
            Token::StartTag(tag) => {
                match tag.name.as_str() {
                    "meta" | "link" | "base" | "basefont" | "bgsound" | "command" => {
                        self.insert_element(&tag.name, tag.attrs.clone());
                        self.open_elements.pop();
                    }
                    "title" | "noscript" | "noframes" | "style" => {
                        self.insert_element(&tag.name, tag.attrs.clone());
                        self.original_insertion_mode = self.insertion_mode.clone();
                        self.insertion_mode = InsertionMode::Text;
                    }
                    "script" => {
                        self.insert_element("script", tag.attrs.clone());
                        self.original_insertion_mode = self.insertion_mode.clone();
                        self.insertion_mode = InsertionMode::Text;
                    }
                    "head" => {} // stray head tag - ignore
                    _ => {
                        self.open_elements.pop();
                        self.insertion_mode = InsertionMode::AfterHead;
                        self.process_token(token);
                    }
                }
            }
            Token::EndTag(tag) => {
                if tag.name == "head" {
                    self.open_elements.pop();
                    self.insertion_mode = InsertionMode::AfterHead;
                }
            }
            Token::Character(c) if c.is_whitespace() => {}
            _ => {
                self.open_elements.pop();
                self.insertion_mode = InsertionMode::AfterHead;
                self.process_token(token);
            }
        }
    }

    fn handle_after_head(&mut self, token: &Token) {
        match token {
            Token::StartTag(tag) if tag.name == "body" => {
                self.insert_element("body", tag.attrs.clone());
                self.frameset_ok = false;
                self.insertion_mode = InsertionMode::InBody;
            }
            Token::StartTag(tag) if tag.name == "frameset" => {
                self.insert_element("frameset", tag.attrs.clone());
                self.insertion_mode = InsertionMode::InFrameset;
            }
            Token::EndTag(_) => {}
            _ => {
                self.insert_element("body", HashMap::new());
                self.insertion_mode = InsertionMode::InBody;
                self.process_token(token);
            }
        }
    }

    fn handle_in_body(&mut self, token: &Token) {
        match token {
            Token::StartTag(tag) => {
                let attrs = tag.attrs.clone();
                match tag.name.as_str() {
                    "html" | "head" | "body" => {} // ignore
                    "div" | "p" | "span" | "section" | "article" | "nav" | "header" | "footer" |
                    "main" | "aside" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" |
                    "ul" | "ol" | "li" | "dl" | "dt" | "dd" |
                    "table" | "form" | "fieldset" | "legend" |
                    "blockquote" | "figure" | "figcaption" |
                    "pre" | "code" | "em" | "strong" | "i" | "b" | "u" | "s" |
                    "small" | "sub" | "sup" | "mark" | "ins" | "del" |
                    "abbr" | "address" | "cite" | "dfn" | "kbd" | "samp" | "var" |
                    "time" | "data" | "video" | "audio" | "picture" | "source" |
                    "canvas" | "details" | "summary" | "dialog" | "menu" |
                    "a" | "br" | "hr" | "img" | "input" | "button" | "label" |
                    "select" | "textarea" | "option" | "optgroup" |
                    "iframe" | "embed" | "object" | "param" => {
                        // Handle special case for <a>
                        if tag.name == "a" {
                            // Check for nested <a>
                            for el in self.open_elements.iter().rev() {
                                if el.tag_name() == Some("a") {
                                    // Re-parent any remaining <a> content
                                    break;
                                }
                            }
                        }
                        // Handle special case for <p>
                        if tag.name == "p" {
                            // Auto-close any open <p>
                            // (simplified)
                        }
                        self.insert_element(&tag.name, attrs);
                        if tag.self_closing {
                            self.open_elements.pop();
                        }
                    }
                    _ => {
                        // Unknown tags - still insert
                        self.insert_element(&tag.name, attrs);
                        if tag.self_closing {
                            self.open_elements.pop();
                        }
                    }
                }
            }
            Token::EndTag(tag) => {
                // Find matching open element and close
                let mut found = false;
                for i in (0..self.open_elements.len()).rev() {
                    if self.open_elements[i].tag_name() == Some(&tag.name) {
                        // Close elements down to and including the matching one
                        let _removed: Vec<Node> = self.open_elements.drain(i..).collect();
                        // Add removed children back to the parent (implied close)
                        found = true;
                        break;
                    }
                }
                if !found {
                    // Ignore unmatched end tags
                }
            }
            Token::Character(c) => {
                if *c == '\0' { return; }
                self.insert_character(*c);
            }
            Token::Whitespace(c) => {
                self.insert_character(*c);
            }
            Token::Comment(text) => {
                if let Some(current) = self.open_elements.last_mut() {
                    current.children.push(Node::new_comment(text.clone()));
                }
            }
            Token::Eof => {}
            Token::Doctype(_) => {}
        }
    }

    fn handle_text(&mut self, token: &Token) {
        match token {
            Token::Character(c) | Token::Whitespace(c) => {
                self.insert_character(*c);
            }
            Token::EndTag(tag) if tag.name == "script" => {
                self.open_elements.pop();
                self.insertion_mode = self.original_insertion_mode.clone();
            }
            Token::EndTag(_) => {
                self.open_elements.pop();
                self.insertion_mode = self.original_insertion_mode.clone();
            }
            _ => {
                self.open_elements.pop();
                self.insertion_mode = self.original_insertion_mode.clone();
                self.process_token(token);
            }
        }
    }

    fn handle_after_body(&mut self, token: &Token) {
        match token {
            Token::EndTag(tag) if tag.name == "html" => {
                self.insertion_mode = InsertionMode::AfterAfterBody;
            }
            _ => self.process_token(token),
        }
    }

    fn handle_after_after_body(&mut self, _token: &Token) {
        // ignore everything after </html>
    }
}
