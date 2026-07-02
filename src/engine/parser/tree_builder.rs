//! ============================================================
//! Aether Browser — HTML5 Tree Builder (DOM Constructor)
//! src/engine/parser/tree_builder.rs
//!
//! Converts token stream from the tokenizer into a proper DOM
//! tree following the HTML5 tree construction specification.
//! ============================================================

use std::collections::HashMap;
use super::tokenizer::{Token, TagToken, DoctypeToken};
use crate::engine::dom::{Node, NodeType, ElementData};

// ── Insertion Modes ───────────────────────────────────────────

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
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

// ── Tree Builder ──────────────────────────────────────────────

pub struct TreeBuilder {
    pub mode: InsertionMode,
    pub original_mode: Option<InsertionMode>,
    pub stack: Vec<Node>,          // open element stack
    pub head_element: Option<Node>,
    pub form_element: Option<Node>,
    pub document: Node,
    pub errors: Vec<String>,

    // Pending table character tokens
    pending_table_chars: Vec<char>,
    frameset_ok: bool,
    scripting: bool,
    foster_parenting: bool,
}

/// Table of "void elements" — elements that cannot have children
fn is_void_element(tag: &str) -> bool {
    matches!(tag,
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" |
        "input" | "link" | "meta" | "param" | "source" | "track" | "wbr"
    )
}

/// "Formatting elements" that require special adoption agency handling
fn is_formatting_element(tag: &str) -> bool {
    matches!(tag,
        "a" | "b" | "big" | "code" | "em" | "font" | "i" | "nobr" |
        "s" | "small" | "strike" | "strong" | "tt" | "u"
    )
}

/// Elements that act as scope boundaries
fn is_scope_element(tag: &str) -> bool {
    matches!(tag,
        "applet" | "caption" | "html" | "table" | "td" | "th" | "marquee" |
        "object" | "template" | "math" | "mi" | "mo" | "mn" | "ms" | "mtext" |
        "annotation-xml" | "foreignObject" | "desc" | "title"
    )
}

fn make_element(tag: &str, attrs: HashMap<String, String>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_name: tag.to_string(),
            attributes: attrs,
        }),
        children: Vec::new(),
    }
}

fn make_text(text: String) -> Node {
    Node {
        node_type: NodeType::Text(text),
        children: Vec::new(),
    }
}

fn make_comment(comment: String) -> Node {
    Node {
        node_type: NodeType::Comment(comment),
        children: Vec::new(),
    }
}

impl TreeBuilder {
    pub fn new() -> Self {
        let document = Node {
            node_type: NodeType::Document,
            children: Vec::new(),
        };
        Self {
            mode: InsertionMode::Initial,
            original_mode: None,
            stack: Vec::new(),
            head_element: None,
            form_element: None,
            document,
            errors: Vec::new(),
            pending_table_chars: Vec::new(),
            frameset_ok: true,
            scripting: false,
            foster_parenting: false,
        }
    }

    fn error(&mut self, msg: &str) {
        self.errors.push(msg.to_string());
    }

    fn current_node_tag(&self) -> Option<&str> {
        self.stack.last().map(|n| {
            if let NodeType::Element(e) = &n.node_type { e.tag_name.as_str() } else { "" }
        })
    }

    fn is_in_scope(&self, target_tag: &str) -> bool {
        for node in self.stack.iter().rev() {
            if let NodeType::Element(e) = &node.node_type {
                if e.tag_name == target_tag { return true; }
                if is_scope_element(&e.tag_name) { return false; }
            }
        }
        false
    }

    fn is_in_button_scope(&self, target_tag: &str) -> bool {
        for node in self.stack.iter().rev() {
            if let NodeType::Element(e) = &node.node_type {
                if e.tag_name == target_tag { return true; }
                if is_scope_element(&e.tag_name) || e.tag_name == "button" { return false; }
            }
        }
        false
    }

    fn pop_until(&mut self, target: &str) {
        while let Some(node) = self.stack.last() {
            let tag = if let NodeType::Element(e) = &node.node_type { e.tag_name.clone() } else { String::new() };
            self.stack.pop();
            if tag == target { break; }
        }
    }

    fn pop_until_any(&mut self, targets: &[&str]) {
        while let Some(node) = self.stack.last() {
            let tag = if let NodeType::Element(e) = &node.node_type { e.tag_name.clone() } else { String::new() };
            self.stack.pop();
            if targets.contains(&tag.as_str()) { break; }
        }
    }

    fn append_to_current(&mut self, node: Node) {
        if let Some(current) = self.stack.last_mut() {
            current.children.push(node);
        } else {
            self.document.children.push(node);
        }
    }

    fn insert_element(&mut self, tag: &str, attrs: HashMap<String, String>) {
        let node = make_element(tag, attrs);
        if !is_void_element(tag) {
            // Push to open elements stack, element gets appended when popped
            // For simplicity we push and let pop handle nesting
            self.stack.push(node);
        } else {
            // void elements go directly into document/current
            self.append_to_current(node);
        }
    }

    /// Main processing function
    pub fn process_token(&mut self, token: Token) {
        match self.mode.clone() {
            InsertionMode::Initial => self.handle_initial(token),
            InsertionMode::BeforeHtml => self.handle_before_html(token),
            InsertionMode::BeforeHead => self.handle_before_head(token),
            InsertionMode::InHead => self.handle_in_head(token),
            InsertionMode::InHeadNoScript => self.handle_in_head_noscript(token),
            InsertionMode::AfterHead => self.handle_after_head(token),
            InsertionMode::InBody => self.handle_in_body(token),
            InsertionMode::Text => self.handle_text(token),
            InsertionMode::InTable => self.handle_in_table(token),
            InsertionMode::InTableBody => self.handle_in_table_body(token),
            InsertionMode::InRow => self.handle_in_row(token),
            InsertionMode::InCell => self.handle_in_cell(token),
            InsertionMode::InSelect => self.handle_in_select(token),
            InsertionMode::AfterBody => self.handle_after_body(token),
            InsertionMode::InFrameset => self.handle_in_frameset(token),
            InsertionMode::AfterFrameset => self.handle_after_frameset(token),
            InsertionMode::AfterAfterBody => self.handle_after_after_body(token),
            _ => self.handle_in_body(token), // fallback
        }
    }

    fn handle_initial(&mut self, token: Token) {
        match token {
            Token::Whitespace(_) => {} // ignore
            Token::Comment(c) => {
                let node = make_comment(c);
                self.document.children.push(node);
            }
            Token::Doctype(d) => {
                let mut attrs = HashMap::new();
                if let Some(name) = &d.name { attrs.insert("name".to_string(), name.clone()); }
                let node = Node { node_type: NodeType::Doctype, children: Vec::new() };
                self.document.children.push(node);
                self.mode = InsertionMode::BeforeHtml;
            }
            _ => {
                // Parse error, switch to before_html and reprocess
                self.mode = InsertionMode::BeforeHtml;
                self.process_token(token);
            }
        }
    }

    fn handle_before_html(&mut self, token: Token) {
        match token {
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::Comment(c) => {
                let node = make_comment(c);
                self.document.children.push(node);
            }
            Token::Whitespace(_) => {}
            Token::StartTag(ref t) if t.tag_name == "html" => {
                let attrs = t.attrs_map();
                let html_node = make_element("html", attrs);
                self.stack.push(html_node);
                self.mode = InsertionMode::BeforeHead;
            }
            Token::EndTag(ref t) if !matches!(t.tag_name.as_str(), "head" | "body" | "html" | "br") => {
                self.error("unexpected-end-tag-before-html");
            }
            _ => {
                // Create implicit html element
                let html_node = make_element("html", HashMap::new());
                self.stack.push(html_node);
                self.mode = InsertionMode::BeforeHead;
                self.process_token(token);
            }
        }
    }

    fn handle_before_head(&mut self, token: Token) {
        match token {
            Token::Whitespace(_) => {}
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::StartTag(ref t) if t.tag_name == "head" => {
                let head = make_element("head", t.attrs_map());
                self.stack.push(head);
                self.mode = InsertionMode::InHead;
            }
            Token::EndTag(ref t) if !matches!(t.tag_name.as_str(), "head" | "body" | "html" | "br") => {
                self.error("unexpected-end-tag-before-head");
            }
            _ => {
                // Insert implicit head
                let head = make_element("head", HashMap::new());
                self.stack.push(head);
                self.mode = InsertionMode::InHead;
                self.process_token(token);
            }
        }
    }

    fn handle_in_head(&mut self, token: Token) {
        match token {
            Token::Whitespace(c) => self.append_to_current(make_text(c.to_string())),
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "base" | "basefont" | "bgsound" | "link") => {
                let node = make_element(&t.tag_name, t.attrs_map());
                self.append_to_current(node);
            }
            Token::StartTag(ref t) if t.tag_name == "meta" => {
                let node = make_element("meta", t.attrs_map());
                self.append_to_current(node);
            }
            Token::StartTag(ref t) if t.tag_name == "title" => {
                let el = make_element("title", t.attrs_map());
                self.stack.push(el);
                self.original_mode = Some(InsertionMode::InHead);
                self.mode = InsertionMode::Text;
            }
            Token::StartTag(ref t) if t.tag_name == "noscript" && !self.scripting => {
                let el = make_element("noscript", t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InHeadNoScript;
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "noframes" | "style") => {
                let el = make_element(&t.tag_name, t.attrs_map());
                self.stack.push(el);
                self.original_mode = Some(InsertionMode::InHead);
                self.mode = InsertionMode::Text;
            }
            Token::StartTag(ref t) if t.tag_name == "script" => {
                let el = make_element("script", t.attrs_map());
                self.stack.push(el);
                self.original_mode = Some(InsertionMode::InHead);
                self.mode = InsertionMode::Text;
            }
            Token::EndTag(ref t) if t.tag_name == "head" => {
                // Pop head from stack and move to after_head
                if let Some(head) = self.stack.pop() {
                    self.append_to_current(head);
                }
                self.mode = InsertionMode::AfterHead;
            }
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(), "body" | "html" | "br") => {
                if let Some(head) = self.stack.pop() {
                    self.append_to_current(head);
                }
                self.mode = InsertionMode::AfterHead;
                self.process_token(token);
            }
            Token::StartTag(ref t) if t.tag_name == "template" => {
                let el = make_element("template", t.attrs_map());
                self.stack.push(el);
            }
            Token::EndTag(ref t) if t.tag_name == "template" => {
                self.pop_until("template");
            }
            Token::StartTag(ref t) if t.tag_name == "head" => {
                self.error("unexpected-start-tag-in-head");
            }
            Token::EndTag(_) => {
                self.error("unexpected-end-tag-in-head");
            }
            _ => {
                if let Some(head) = self.stack.pop() {
                    self.append_to_current(head);
                }
                self.mode = InsertionMode::AfterHead;
                self.process_token(token);
            }
        }
    }

    fn handle_in_head_noscript(&mut self, token: Token) {
        match token {
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::EndTag(ref t) if t.tag_name == "noscript" => {
                self.stack.pop();
                self.mode = InsertionMode::InHead;
            }
            Token::Whitespace(_) | Token::Comment(_) => self.handle_in_head(token),
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(),
                "basefont" | "bgsound" | "link" | "meta" | "noframes" | "style") => {
                self.handle_in_head(token);
            }
            Token::EndTag(ref t) if t.tag_name == "br" => {
                self.error("unexpected-end-tag-br-in-head-noscript");
                self.stack.pop();
                self.mode = InsertionMode::InHead;
                self.process_token(token);
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "head" | "noscript") => {
                self.error("unexpected-tag-in-head-noscript");
            }
            Token::EndTag(_) => {
                self.error("unexpected-end-tag-in-head-noscript");
            }
            _ => {
                self.error("unexpected-token-in-head-noscript");
                self.stack.pop();
                self.mode = InsertionMode::InHead;
                self.process_token(token);
            }
        }
    }

    fn handle_after_head(&mut self, token: Token) {
        match token {
            Token::Whitespace(c) => self.append_to_current(make_text(c.to_string())),
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::StartTag(ref t) if t.tag_name == "body" => {
                let body = make_element("body", t.attrs_map());
                self.stack.push(body);
                self.frameset_ok = true;
                self.mode = InsertionMode::InBody;
            }
            Token::StartTag(ref t) if t.tag_name == "frameset" => {
                let el = make_element("frameset", t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InFrameset;
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(),
                "base" | "basefont" | "bgsound" | "link" | "meta" |
                "noframes" | "script" | "style" | "template" | "title") => {
                self.error("unexpected-tag-after-head");
                // Re-insert head and reprocess
                self.mode = InsertionMode::InHead;
                self.process_token(token);
            }
            Token::EndTag(ref t) if t.tag_name == "template" => {
                self.handle_in_head(token);
            }
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(), "body" | "html" | "br") => {
                // treat as "anything else"
                let body = make_element("body", HashMap::new());
                self.stack.push(body);
                self.mode = InsertionMode::InBody;
                self.process_token(token);
            }
            Token::StartTag(ref t) if t.tag_name == "head" => {
                self.error("unexpected-head-start-tag");
            }
            Token::EndTag(_) => {
                self.error("unexpected-end-tag-after-head");
            }
            _ => {
                let body = make_element("body", HashMap::new());
                self.stack.push(body);
                self.mode = InsertionMode::InBody;
                self.process_token(token);
            }
        }
    }

    fn handle_in_body(&mut self, token: Token) {
        match token {
            Token::Whitespace(c) => {
                // Reconstruct active formatting elements (simplified)
                self.append_to_current(make_text(c.to_string()));
            }
            Token::Character(c) => {
                self.frameset_ok = false;
                self.append_to_current(make_text(c.to_string()));
            }
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),

            Token::StartTag(ref t) if t.tag_name == "html" => {
                self.error("unexpected-html-start-in-body");
                // Merge attributes into html element if it exists on stack
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(),
                "base" | "basefont" | "bgsound" | "link" | "meta" |
                "noframes" | "script" | "style" | "template" | "title") => {
                self.handle_in_head(token);
            }

            Token::StartTag(ref t) if t.tag_name == "body" => {
                self.error("unexpected-body-tag-in-body");
                // Merge attrs into existing body element
            }

            Token::StartTag(ref t) if t.tag_name == "frameset" => {
                self.error("unexpected-frameset-tag-in-body");
            }

            Token::Eof => {
                // Stop parsing
            }

            Token::EndTag(ref t) if t.tag_name == "body" => {
                if !self.is_in_scope("body") {
                    self.error("end-tag-body-not-in-scope");
                } else {
                    self.mode = InsertionMode::AfterBody;
                }
            }

            Token::EndTag(ref t) if t.tag_name == "html" => {
                if !self.is_in_scope("body") {
                    self.error("end-tag-html-not-in-scope");
                } else {
                    self.mode = InsertionMode::AfterBody;
                    self.process_token(token);
                }
            }

            // Block elements
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(),
                "address" | "article" | "aside" | "blockquote" | "center" | "details" |
                "dialog" | "dir" | "div" | "dl" | "fieldset" | "figcaption" | "figure" |
                "footer" | "header" | "hgroup" | "main" | "menu" | "nav" | "ol" | "p" |
                "section" | "summary" | "ul") => {
                if self.is_in_button_scope("p") {
                    self.close_p_element();
                }
                self.insert_element(&t.tag_name.clone(), t.attrs_map());
            }

            // Headings
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6") => {
                if self.is_in_button_scope("p") {
                    self.close_p_element();
                }
                if let Some(tag) = self.current_node_tag() {
                    if matches!(tag, "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
                        self.error("unexpected-heading-in-heading");
                        self.stack.pop();
                    }
                }
                self.insert_element(&t.tag_name.clone(), t.attrs_map());
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "pre" | "listing") => {
                if self.is_in_button_scope("p") { self.close_p_element(); }
                self.insert_element(&t.tag_name.clone(), t.attrs_map());
                self.frameset_ok = false;
            }

            Token::StartTag(ref t) if t.tag_name == "form" => {
                if self.form_element.is_some() {
                    self.error("unexpected-form-in-form");
                } else {
                    if self.is_in_button_scope("p") { self.close_p_element(); }
                    let el = make_element("form", t.attrs_map());
                    self.stack.push(el);
                }
            }

            Token::StartTag(ref t) if t.tag_name == "li" => {
                self.frameset_ok = false;
                // Run the list item algorithm
                let mut to_pop = false;
                if let Some(tag) = self.current_node_tag() {
                    if tag == "li" { to_pop = true; }
                }
                if to_pop { self.pop_until("li"); }
                if self.is_in_button_scope("p") { self.close_p_element(); }
                self.insert_element("li", t.attrs_map());
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "dd" | "dt") => {
                self.frameset_ok = false;
                let tag_name = t.tag_name.clone();
                let attrs = t.attrs_map();
                self.insert_element(&tag_name, attrs);
            }

            Token::StartTag(ref t) if t.tag_name == "plaintext" => {
                if self.is_in_button_scope("p") { self.close_p_element(); }
                self.insert_element("plaintext", t.attrs_map());
            }

            Token::StartTag(ref t) if t.tag_name == "button" => {
                if self.is_in_scope("button") {
                    self.error("unexpected-button-in-scope");
                    self.pop_until("button");
                }
                self.insert_element("button", t.attrs_map());
                self.frameset_ok = false;
            }

            // End tags for block elements
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(),
                "address" | "article" | "aside" | "blockquote" | "button" | "center" |
                "details" | "dialog" | "dir" | "div" | "dl" | "fieldset" | "figcaption" |
                "figure" | "footer" | "header" | "hgroup" | "listing" | "main" | "menu" |
                "nav" | "ol" | "pre" | "section" | "summary" | "ul") => {
                let tag = t.tag_name.clone();
                if !self.is_in_scope(&tag) {
                    self.error("end-tag-without-matching-open");
                } else {
                    self.generate_implied_end_tags(&tag);
                    if self.current_node_tag() != Some(&tag) {
                        self.error("unexpected-end-tag");
                    }
                    self.pop_until(&tag);
                }
            }

            Token::EndTag(ref t) if t.tag_name == "form" => {
                let node = self.form_element.take();
                if node.is_none() {
                    self.error("no-form-to-close");
                } else {
                    self.generate_implied_end_tags("form");
                    self.pop_until("form");
                }
            }

            Token::EndTag(ref t) if t.tag_name == "p" => {
                if !self.is_in_button_scope("p") {
                    self.error("no-p-in-scope");
                    // Insert empty p
                    self.insert_element("p", HashMap::new());
                }
                self.close_p_element();
            }

            Token::EndTag(ref t) if t.tag_name == "li" => {
                if !self.is_in_scope("li") {
                    self.error("no-li-in-scope");
                } else {
                    self.generate_implied_end_tags("li");
                    self.pop_until("li");
                }
            }

            Token::EndTag(ref t) if matches!(t.tag_name.as_str(), "dd" | "dt") => {
                let tag = t.tag_name.clone();
                if !self.is_in_scope(&tag) {
                    self.error("no-dd-dt-in-scope");
                } else {
                    self.generate_implied_end_tags(&tag);
                    self.pop_until(&tag);
                }
            }

            Token::EndTag(ref t) if matches!(t.tag_name.as_str(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6") => {
                let headings = ["h1", "h2", "h3", "h4", "h5", "h6"];
                let in_scope = headings.iter().any(|h| self.is_in_scope(h));
                if !in_scope {
                    self.error("no-heading-in-scope");
                } else {
                    self.generate_implied_end_tags(&t.tag_name);
                    self.pop_until_any(&headings);
                }
            }

            // Formatted inline elements (a, b, em, i, s, strong, etc.)
            Token::StartTag(ref t) if is_formatting_element(&t.tag_name) => {
                let tag = t.tag_name.clone();
                let attrs = t.attrs_map();
                self.insert_element(&tag, attrs);
            }

            Token::EndTag(ref t) if is_formatting_element(&t.tag_name) => {
                let tag = t.tag_name.clone();
                self.generate_implied_end_tags(&tag);
                self.pop_until(&tag);
            }

            // Void/self-closing elements
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "area" | "br" | "embed" | "img" | "keygen" | "wbr") => {
                let tag = t.tag_name.clone();
                let attrs = t.attrs_map();
                let node = make_element(&tag, attrs);
                self.append_to_current(node);
                self.frameset_ok = false;
            }

            Token::StartTag(ref t) if t.tag_name == "input" => {
                let attrs = t.attrs_map();
                let is_hidden = attrs.get("type").map(|t| t.eq_ignore_ascii_case("hidden")).unwrap_or(false);
                let node = make_element("input", attrs);
                self.append_to_current(node);
                if !is_hidden { self.frameset_ok = false; }
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "param" | "source" | "track") => {
                let tag = t.tag_name.clone();
                let attrs = t.attrs_map();
                let node = make_element(&tag, attrs);
                self.append_to_current(node);
            }

            Token::StartTag(ref t) if t.tag_name == "hr" => {
                if self.is_in_button_scope("p") { self.close_p_element(); }
                let node = make_element("hr", t.attrs_map());
                self.append_to_current(node);
                self.frameset_ok = false;
            }

            Token::StartTag(ref t) if t.tag_name == "image" => {
                self.error("tag-image-changed-to-img");
                let mut attrs = t.attrs_map();
                let node = make_element("img", attrs);
                self.append_to_current(node);
            }

            Token::StartTag(ref t) if t.tag_name == "textarea" => {
                let el = make_element("textarea", t.attrs_map());
                self.stack.push(el);
                self.frameset_ok = false;
                self.original_mode = Some(InsertionMode::InBody);
                self.mode = InsertionMode::Text;
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "xmp") => {
                if self.is_in_button_scope("p") { self.close_p_element(); }
                let el = make_element("xmp", t.attrs_map());
                self.stack.push(el);
                self.frameset_ok = false;
                self.original_mode = Some(InsertionMode::InBody);
                self.mode = InsertionMode::Text;
            }

            Token::StartTag(ref t) if t.tag_name == "iframe" => {
                self.frameset_ok = false;
                let el = make_element("iframe", t.attrs_map());
                self.stack.push(el);
                self.original_mode = Some(InsertionMode::InBody);
                self.mode = InsertionMode::Text;
            }

            Token::StartTag(ref t) if t.tag_name == "select" => {
                let el = make_element("select", t.attrs_map());
                self.stack.push(el);
                self.frameset_ok = false;
                self.mode = InsertionMode::InSelect;
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "optgroup" | "option") => {
                if self.current_node_tag() == Some("option") {
                    self.stack.pop();
                }
                let tag = t.tag_name.clone();
                let el = make_element(&tag, t.attrs_map());
                self.stack.push(el);
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "rb" | "rtc") => {
                if self.is_in_scope("ruby") {
                    self.generate_implied_end_tags("");
                }
                let tag = t.tag_name.clone();
                let el = make_element(&tag, t.attrs_map());
                self.stack.push(el);
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "rp" | "rt") => {
                if self.is_in_scope("ruby") {
                    self.generate_implied_end_tags("rtc");
                }
                let tag = t.tag_name.clone();
                let el = make_element(&tag, t.attrs_map());
                self.stack.push(el);
            }

            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "caption" | "col" | "colgroup" | "frame" | "head" | "tbody" | "td" | "tfoot" | "th" | "thead" | "tr") => {
                self.error("unexpected-table-tag-in-body");
            }

            Token::StartTag(ref t) => {
                let tag = t.tag_name.clone();
                let attrs = t.attrs_map();
                self.insert_element(&tag, attrs);
            }

            Token::EndTag(ref t) => {
                let tag = t.tag_name.clone();
                // Find the element in the open elements stack and pop to it
                let found = self.stack.iter().rev().any(|n| {
                    if let NodeType::Element(e) = &n.node_type { e.tag_name == tag } else { false }
                });
                if found {
                    self.generate_implied_end_tags(&tag);
                    self.pop_until(&tag);
                } else {
                    self.error("end-tag-without-matching-open");
                }
            }
        }
    }

    fn handle_text(&mut self, token: Token) {
        match token {
            Token::Character(c) => {
                if let Some(current) = self.stack.last_mut() {
                    // Merge text into current element's last text child if possible
                    if let Some(last_child) = current.children.last_mut() {
                        if let NodeType::Text(ref mut t) = last_child.node_type {
                            t.push(c);
                            return;
                        }
                    }
                    current.children.push(make_text(c.to_string()));
                }
            }
            Token::Whitespace(c) => {
                if let Some(current) = self.stack.last_mut() {
                    current.children.push(make_text(c.to_string()));
                }
            }
            Token::Eof => {
                self.error("eof-in-text");
                if let Some(el) = self.stack.pop() {
                    self.append_to_current(el);
                }
                let mode = self.original_mode.take().unwrap_or(InsertionMode::InBody);
                self.mode = mode;
                self.process_token(Token::Eof);
            }
            Token::EndTag(ref t) => {
                if let Some(el) = self.stack.pop() {
                    self.append_to_current(el);
                }
                let mode = self.original_mode.take().unwrap_or(InsertionMode::InBody);
                self.mode = mode;
            }
            _ => {}
        }
    }

    fn handle_in_table(&mut self, token: Token) {
        match token {
            Token::Character(_) | Token::Whitespace(_) => {
                self.pending_table_chars.push(
                    if let Token::Character(c) | Token::Whitespace(c) = token { c } else { ' ' }
                );
            }
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "caption" => {
                let el = make_element("caption", t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InCaption;
            }
            Token::StartTag(ref t) if t.tag_name == "colgroup" => {
                let el = make_element("colgroup", t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InColumnGroup;
            }
            Token::StartTag(ref t) if t.tag_name == "col" => {
                let el = make_element("colgroup", HashMap::new());
                self.stack.push(el);
                self.mode = InsertionMode::InColumnGroup;
                self.process_token(token);
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "tbody" | "tfoot" | "thead") => {
                let tag = t.tag_name.clone();
                let el = make_element(&tag, t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InTableBody;
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "td" | "th" | "tr") => {
                let el = make_element("tbody", HashMap::new());
                self.stack.push(el);
                self.mode = InsertionMode::InTableBody;
                self.process_token(token);
            }
            Token::StartTag(ref t) if t.tag_name == "table" => {
                self.error("nested-table");
                if self.is_in_scope("table") {
                    self.pop_until("table");
                    self.reset_insertion_mode();
                    self.process_token(token);
                }
            }
            Token::EndTag(ref t) if t.tag_name == "table" => {
                if !self.is_in_scope("table") {
                    self.error("no-table-in-scope");
                } else {
                    self.pop_until("table");
                    self.reset_insertion_mode();
                }
            }
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(),
                "body" | "caption" | "col" | "colgroup" | "html" | "tbody" | "td" | "tfoot" | "th" | "thead" | "tr") => {
                self.error("unexpected-end-tag-in-table");
            }
            Token::Eof => self.emit(Token::Eof),
            _ => {
                // Foster parenting: insert before the table
                self.error("foster-parenting");
                self.handle_in_body(token);
            }
        }
    }

    fn handle_in_table_body(&mut self, token: Token) {
        match token {
            Token::StartTag(ref t) if t.tag_name == "tr" => {
                let el = make_element("tr", t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InRow;
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "th" | "td") => {
                self.error("table-cell-outside-tr");
                let el = make_element("tr", HashMap::new());
                self.stack.push(el);
                self.mode = InsertionMode::InRow;
                self.process_token(token);
            }
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(), "tbody" | "tfoot" | "thead") => {
                let tag = t.tag_name.clone();
                if !self.is_in_scope(&tag) {
                    self.error("no-table-body-in-scope");
                } else {
                    self.pop_until_any(&["tbody", "tfoot", "thead"]);
                    self.mode = InsertionMode::InTable;
                }
            }
            Token::EndTag(ref t) if t.tag_name == "table" => {
                if !self.is_in_scope("tbody") && !self.is_in_scope("thead") && !self.is_in_scope("tfoot") {
                    self.error("no-table-body-to-close");
                } else {
                    self.pop_until_any(&["tbody", "tfoot", "thead"]);
                    self.mode = InsertionMode::InTable;
                    self.process_token(token);
                }
            }
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(),
                "body" | "caption" | "col" | "colgroup" | "html" | "td" | "th" | "tr") => {
                self.error("unexpected-end-tag-in-table-body");
            }
            _ => self.handle_in_table(token),
        }
    }

    fn handle_in_row(&mut self, token: Token) {
        match token {
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(), "th" | "td") => {
                let tag = t.tag_name.clone();
                let el = make_element(&tag, t.attrs_map());
                self.stack.push(el);
                self.mode = InsertionMode::InCell;
            }
            Token::EndTag(ref t) if t.tag_name == "tr" => {
                if !self.is_in_scope("tr") {
                    self.error("no-tr-in-scope");
                } else {
                    self.generate_implied_end_tags("tr");
                    self.pop_until("tr");
                    self.mode = InsertionMode::InTableBody;
                }
            }
            Token::EndTag(ref t) if t.tag_name == "table" => {
                if !self.is_in_scope("tr") {
                    self.error("no-tr-in-scope");
                } else {
                    self.pop_until("tr");
                    self.mode = InsertionMode::InTableBody;
                    self.process_token(token);
                }
            }
            _ => self.handle_in_table(token),
        }
    }

    fn handle_in_cell(&mut self, token: Token) {
        match token {
            Token::EndTag(ref t) if matches!(t.tag_name.as_str(), "td" | "th") => {
                let tag = t.tag_name.clone();
                if !self.is_in_scope(&tag) {
                    self.error("no-cell-in-scope");
                } else {
                    self.generate_implied_end_tags(&tag);
                    self.pop_until(&tag);
                    self.mode = InsertionMode::InRow;
                }
            }
            Token::StartTag(ref t) if matches!(t.tag_name.as_str(),
                "caption" | "col" | "colgroup" | "tbody" | "td" | "tfoot" | "th" | "thead" | "tr") => {
                if !self.is_in_scope("td") && !self.is_in_scope("th") {
                    self.error("no-cell-to-close");
                } else {
                    self.pop_until_any(&["td", "th"]);
                    self.mode = InsertionMode::InRow;
                    self.process_token(token);
                }
            }
            _ => self.handle_in_body(token),
        }
    }

    fn handle_in_select(&mut self, token: Token) {
        match token {
            Token::Character(c) => self.append_to_current(make_text(c.to_string())),
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => {}
            Token::StartTag(ref t) if t.tag_name == "option" => {
                if self.current_node_tag() == Some("option") { self.stack.pop(); }
                let el = make_element("option", t.attrs_map());
                self.stack.push(el);
            }
            Token::StartTag(ref t) if t.tag_name == "optgroup" => {
                if self.current_node_tag() == Some("option") { self.stack.pop(); }
                if self.current_node_tag() == Some("optgroup") { self.stack.pop(); }
                let el = make_element("optgroup", t.attrs_map());
                self.stack.push(el);
            }
            Token::EndTag(ref t) if t.tag_name == "optgroup" => {
                if self.current_node_tag() == Some("option") {
                    // check if second from top is optgroup
                    let len = self.stack.len();
                    if len >= 2 {
                        if let NodeType::Element(e) = &self.stack[len - 2].node_type {
                            if e.tag_name == "optgroup" { self.stack.pop(); }
                        }
                    }
                }
                if self.current_node_tag() == Some("optgroup") {
                    self.stack.pop();
                } else {
                    self.error("no-optgroup-in-select");
                }
            }
            Token::EndTag(ref t) if t.tag_name == "option" => {
                if self.current_node_tag() == Some("option") {
                    self.stack.pop();
                } else {
                    self.error("no-option-in-select");
                }
            }
            Token::EndTag(ref t) if t.tag_name == "select" => {
                if !self.is_in_scope("select") {
                    self.error("no-select-in-scope");
                } else {
                    self.pop_until("select");
                    self.reset_insertion_mode();
                }
            }
            Token::StartTag(ref t) if t.tag_name == "select" => {
                self.error("unexpected-select-in-select");
                self.pop_until("select");
                self.reset_insertion_mode();
            }
            Token::Eof => self.emit(Token::Eof),
            _ => self.error("unexpected-token-in-select"),
        }
    }

    fn handle_after_body(&mut self, token: Token) {
        match token {
            Token::Whitespace(_) => self.handle_in_body(token),
            Token::Comment(c) => {
                // Append to html element
                if let Some(html) = self.stack.first_mut() {
                    html.children.push(make_comment(c));
                }
            }
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::EndTag(ref t) if t.tag_name == "html" => {
                self.mode = InsertionMode::AfterAfterBody;
            }
            Token::Eof => {} // done
            _ => {
                self.error("unexpected-token-after-body");
                self.mode = InsertionMode::InBody;
                self.process_token(token);
            }
        }
    }

    fn handle_in_frameset(&mut self, token: Token) {
        match token {
            Token::Whitespace(c) => self.append_to_current(make_text(c.to_string())),
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::StartTag(ref t) if t.tag_name == "frameset" => {
                let el = make_element("frameset", t.attrs_map());
                self.stack.push(el);
            }
            Token::EndTag(ref t) if t.tag_name == "frameset" => {
                if self.stack.len() == 1 {
                    self.error("frameset-at-document-root");
                } else {
                    self.stack.pop();
                    if self.current_node_tag() != Some("frameset") {
                        self.mode = InsertionMode::AfterFrameset;
                    }
                }
            }
            Token::StartTag(ref t) if t.tag_name == "frame" => {
                let node = make_element("frame", t.attrs_map());
                self.append_to_current(node);
            }
            Token::StartTag(ref t) if t.tag_name == "noframes" => {
                self.handle_in_head(token);
            }
            Token::Eof => {}
            _ => self.error("unexpected-token-in-frameset"),
        }
    }

    fn handle_after_frameset(&mut self, token: Token) {
        match token {
            Token::Whitespace(c) => self.append_to_current(make_text(c.to_string())),
            Token::Comment(c) => self.append_to_current(make_comment(c)),
            Token::Doctype(_) => self.error("misplaced-doctype"),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::EndTag(ref t) if t.tag_name == "html" => {
                self.mode = InsertionMode::AfterAfterFrameset;
            }
            Token::StartTag(ref t) if t.tag_name == "noframes" => {
                self.handle_in_head(token);
            }
            Token::Eof => {}
            _ => self.error("unexpected-token-after-frameset"),
        }
    }

    fn handle_after_after_body(&mut self, token: Token) {
        match token {
            Token::Comment(c) => self.document.children.push(make_comment(c)),
            Token::Doctype(_) | Token::Whitespace(_) => self.handle_in_body(token),
            Token::StartTag(ref t) if t.tag_name == "html" => self.handle_in_body(token),
            Token::Eof => {} // done
            _ => {
                self.error("unexpected-token-after-after-body");
                self.mode = InsertionMode::InBody;
                self.process_token(token);
            }
        }
    }

    // ── Helpers ───────────────────────────────────────────────

    fn emit(&mut self, _token: Token) {}

    fn close_p_element(&mut self) {
        self.generate_implied_end_tags("p");
        if self.current_node_tag() != Some("p") {
            self.error("unclosed-p-element");
        }
        self.pop_until("p");
    }

    fn generate_implied_end_tags(&mut self, exclude: &str) {
        loop {
            match self.current_node_tag() {
                Some(tag) if matches!(tag, "dd" | "dt" | "li" | "optgroup" | "option" | "p" | "rb" | "rp" | "rt" | "rtc") => {
                    if tag == exclude { break; }
                    let el = self.stack.pop().unwrap();
                    self.append_to_current(el);
                }
                _ => break,
            }
        }
    }

    fn reset_insertion_mode(&mut self) {
        for node in self.stack.iter().rev() {
            if let NodeType::Element(e) = &node.node_type {
                match e.tag_name.as_str() {
                    "select" => { self.mode = InsertionMode::InSelect; return; }
                    "td" | "th" => { self.mode = InsertionMode::InCell; return; }
                    "tr" => { self.mode = InsertionMode::InRow; return; }
                    "tbody" | "thead" | "tfoot" => { self.mode = InsertionMode::InTableBody; return; }
                    "caption" => { self.mode = InsertionMode::InCaption; return; }
                    "colgroup" => { self.mode = InsertionMode::InColumnGroup; return; }
                    "table" => { self.mode = InsertionMode::InTable; return; }
                    "template" => { self.mode = InsertionMode::InTemplate; return; }
                    "head" => { self.mode = InsertionMode::InHead; return; }
                    "body" => { self.mode = InsertionMode::InBody; return; }
                    "frameset" => { self.mode = InsertionMode::InFrameset; return; }
                    "html" => { self.mode = InsertionMode::BeforeHead; return; }
                    _ => {}
                }
            }
        }
        self.mode = InsertionMode::InBody;
    }

    /// Flush the open element stack and build the final document tree
    pub fn finish(&mut self) -> Node {
        // Pop remaining elements and nest them
        while self.stack.len() > 1 {
            let child = self.stack.pop().unwrap();
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(child);
            }
        }
        // Attach the remaining root to the document
        if let Some(root) = self.stack.pop() {
            self.document.children.push(root);
        }
        std::mem::replace(&mut self.document, Node {
            node_type: NodeType::Document,
            children: Vec::new(),
        })
    }
}

// ── High-Level Parse Function ─────────────────────────────────

pub fn parse_html(source: &str) -> Node {
    let tokens = super::tokenizer::tokenize(source);
    let mut builder = TreeBuilder::new();
    for tok in tokens {
        builder.process_token(tok);
    }
    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_document() {
        let doc = parse_html("<!DOCTYPE html><html><head><title>Test</title></head><body><p>Hello</p></body></html>");
        // Should have a document with html child
        assert!(!doc.children.is_empty(), "Document should have children");
    }

    #[test]
    fn test_implicit_html() {
        // Even without explicit html/head/body tags, the tree should be constructed
        let doc = parse_html("<p>Hello world</p>");
        assert!(!doc.children.is_empty(), "Document should have children");
    }

    #[test]
    fn test_nested_elements() {
        let doc = parse_html("<div><p><span>text</span></p></div>");
        assert!(!doc.children.is_empty());
    }
}
