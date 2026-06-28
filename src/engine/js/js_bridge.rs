use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::engine::dom::{ElementData, Node, NodeType};

// ── Flat DOM node representation ────────────────────────────────────

#[derive(Debug, Clone)]
struct FlatNode {
    parent: Option<u32>,
    children: Vec<u32>,
    tag: String,
    attrs: HashMap<String, String>,
    text: String,
    is_text: bool,
    is_document: bool,
    inline_styles: HashMap<String, String>,
}

impl FlatNode {
    fn document() -> Self {
        Self { parent: None, children: vec![], tag: String::new(), attrs: HashMap::new(), text: String::new(), is_text: false, is_document: true, inline_styles: HashMap::new() }
    }
    fn element(tag: &str) -> Self {
        Self { parent: None, children: vec![], tag: tag.to_lowercase(), attrs: HashMap::new(), text: String::new(), is_text: false, is_document: false, inline_styles: HashMap::new() }
    }
    fn text(content: &str) -> Self {
        Self { parent: None, children: vec![], tag: String::new(), attrs: HashMap::new(), text: content.to_string(), is_text: true, is_document: false, inline_styles: HashMap::new() }
    }
}

// ── CSS Selector types ──────────────────────────────────────────────

#[derive(Debug, Clone)]
enum SimpleSel {
    Universal,
    Tag(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone)]
enum Combinator {
    Descendant,
    Child,
}

#[derive(Debug, Clone)]
struct CompoundSel {
    simples: Vec<SimpleSel>,
}

#[derive(Debug, Clone)]
struct ComplexSel {
    compound: CompoundSel,
    combinator: Option<(Combinator, Box<ComplexSel>)>,
}

fn parse_simple_selector(s: &str, pos: &mut usize) -> Option<SimpleSel> {
    let chars: Vec<char> = s.chars().collect();
    if *pos >= chars.len() { return None; }
    match chars[*pos] {
        '*' => { *pos += 1; Some(SimpleSel::Universal) }
        '.' => {
            *pos += 1;
            let start = *pos;
            while *pos < chars.len() && chars[*pos] != '.' && chars[*pos] != '#' && chars[*pos] != '[' && !chars[*pos].is_whitespace() && chars[*pos] != '>' { *pos += 1; }
            if *pos > start { Some(SimpleSel::Class(s[start..*pos].to_string())) } else { None }
        }
        '#' => {
            *pos += 1;
            let start = *pos;
            while *pos < chars.len() && chars[*pos] != '.' && chars[*pos] != '#' && chars[*pos] != '[' && !chars[*pos].is_whitespace() && chars[*pos] != '>' { *pos += 1; }
            if *pos > start { Some(SimpleSel::Id(s[start..*pos].to_string())) } else { None }
        }
        c if c.is_alphanumeric() || c == '-' => {
            let start = *pos;
            while *pos < chars.len() && (chars[*pos].is_alphanumeric() || chars[*pos] == '-') { *pos += 1; }
            Some(SimpleSel::Tag(s[start..*pos].to_string()))
        }
        _ => None,
    }
}

fn parse_compound(s: &str, pos: &mut usize) -> CompoundSel {
    let mut simples = vec![];
    while *pos < s.len() {
        skip_ws(s, pos);
        if *pos >= s.len() || s.as_bytes()[*pos] == b'>' { break; }
        if let Some(simple) = parse_simple_selector(s, pos) {
            simples.push(simple);
        } else { break; }
    }
    if simples.is_empty() { simples.push(SimpleSel::Universal); }
    CompoundSel { simples }
}

fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && s.as_bytes()[*pos].is_ascii_whitespace() { *pos += 1; }
}

fn parse_combinator(s: &str, pos: &mut usize) -> Option<Combinator> {
    skip_ws(s, pos);
    if *pos < s.len() && s.as_bytes()[*pos] == b'>' {
        *pos += 1;
        skip_ws(s, pos);
        Some(Combinator::Child)
    } else if *pos < s.len() && (s.as_bytes()[*pos] as char).is_whitespace() {
        skip_ws(s, pos);
        if *pos < s.len() { Some(Combinator::Descendant) } else { None }
    } else {
        None
    }
}

fn parse_complex(s: &str) -> Option<ComplexSel> {
    let s = s.trim();
    if s.is_empty() { return None; }
    let mut pos = 0;
    let compound = parse_compound(s, &mut pos);
    let combinator = if pos < s.len() {
        let comb = parse_combinator(s, &mut pos);
        comb.map(|c| {
            let rest = parse_complex(&s[pos..]).unwrap_or(ComplexSel { compound: CompoundSel { simples: vec![SimpleSel::Universal] }, combinator: None });
            (c, Box::new(rest))
        })
    } else { None };
    Some(ComplexSel { compound, combinator })
}

fn matches_simple(node: &FlatNode, sel: &SimpleSel) -> bool {
    if node.is_text || node.is_document { return false; }
    match sel {
        SimpleSel::Universal => true,
        SimpleSel::Tag(t) => node.tag == *t,
        SimpleSel::Class(c) => node.attrs.get("class").map_or(false, |v| v.split_whitespace().any(|p| p == c)),
        SimpleSel::Id(id) => node.attrs.get("id").map_or(false, |v| v == id),
    }
}

fn matches_compound(node: &FlatNode, sel: &CompoundSel) -> bool {
    sel.simples.iter().all(|s| matches_simple(node, s))
}

fn matches_complex(nodes: &[FlatNode], node_id: u32, sel: &ComplexSel) -> bool {
    if let Some(node) = nodes.get(node_id as usize) {
        if !matches_compound(node, &sel.compound) { return false; }
        if let Some((combinator, rest)) = &sel.combinator {
            match combinator {
                Combinator::Descendant => {
                    let mut current = node.parent;
                    while let Some(pid) = current {
                        if matches_complex(nodes, pid, rest) { return true; }
                        current = nodes.get(pid as usize).and_then(|n| n.parent);
                    }
                    false
                }
                Combinator::Child => {
                    node.parent.map_or(false, |pid| matches_complex(nodes, pid, rest))
                }
            }
        } else { true }
    } else { false }
}

// ── Timer entry ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct TimerEntry {
    id: u32,
    source: String,
    delay_ms: u64,
    is_interval: bool,
    fire_at: std::time::Instant,
}

// ── Event listener entry ────────────────────────────────────────────

#[derive(Debug, Clone)]
struct EventListenerEntry {
    node_id: u32,
    event_type: String,
    source: String,
}

// ── URL parts helper ────────────────────────────────────────────────

struct UrlParts {
    protocol: String,
    hostname: String,
    port: String,
    pathname: String,
    search: String,
    hash: String,
}

impl Default for UrlParts {
    fn default() -> Self {
        Self { protocol: "https:".into(), hostname: String::new(), port: String::new(), pathname: "/".into(), search: String::new(), hash: String::new() }
    }
}

fn parse_url(url: &str) -> UrlParts {
    let mut parts = UrlParts::default();
    let s = url.trim();
    if s.is_empty() { return parts; }

    // Protocol
    let rest = if let Some(pos) = s.find("://") {
        parts.protocol = s[..pos+1].to_string();
        &s[pos+3..]
    } else if s.starts_with("//") {
        parts.protocol = "https:".into();
        &s[2..]
    } else {
        parts.protocol = "https:".into();
        s
    };

    // Hash
    let rest = if let Some(pos) = rest.find('#') {
        parts.hash = rest[pos..].to_string();
        &rest[..pos]
    } else { rest };

    // Search
    let rest = if let Some(pos) = rest.find('?') {
        parts.search = rest[pos..].to_string();
        &rest[..pos]
    } else { rest };

    // Pathname
    let rest = if let Some(pos) = rest.find('/') {
        parts.pathname = rest[pos..].to_string();
        &rest[..pos]
    } else {
        parts.pathname = "/".to_string();
        rest
    };

    // Hostname:port
    if let Some(pos) = rest.find(':') {
        parts.hostname = rest[..pos].to_string();
        parts.port = rest[pos+1..].to_string();
    } else {
        parts.hostname = rest.to_string();
    }

    parts
}

// ── JsBridge ────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct JsBridge {
    pub write_buffer: String,
    nodes: Vec<FlatNode>,
    pub body_id: Option<u32>,
    pub current_url: String,
    pub pending_navigation: Option<String>,
    next_timer_id: u32,
    timers: Vec<TimerEntry>,
    event_listeners: Vec<EventListenerEntry>,
}

impl JsBridge {
    fn new_internal(url: &str) -> Self {
        Self {
            write_buffer: String::new(),
            nodes: vec![],
            body_id: None,
            current_url: url.to_string(),
            pending_navigation: None,
            next_timer_id: 1,
            timers: vec![],
            event_listeners: vec![],
        }
    }

    pub fn new() -> Self {
        Self::new_internal("https://localhost")
    }

    pub fn document_write(&mut self, text: &str) {
        self.write_buffer.push_str(text);
    }

    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.write_buffer)
    }

    // ── Load DOM tree from crate DOM ────────────────────────────────

    fn flatten(node: &Node, nodes: &mut Vec<FlatNode>) -> u32 {
        let id = nodes.len() as u32;
        match &node.node_type {
            NodeType::Document => {
                nodes.push(FlatNode::document());
                for child in &node.children {
                    let child_id = Self::flatten(child, nodes);
                    nodes[id as usize].children.push(child_id);
                    nodes[child_id as usize].parent = Some(id);
                }
            }
            NodeType::Text(text) => {
                nodes.push(FlatNode::text(text));
            }
            NodeType::Comment(_) => {}
            NodeType::Element(elem) => {
                let mut fn_ = FlatNode::element(&elem.tag_name);
                fn_.attrs = elem.attributes.clone();
                nodes.push(fn_);
                for child in &node.children {
                    let child_id = Self::flatten(child, nodes);
                    nodes[id as usize].children.push(child_id);
                    nodes[child_id as usize].parent = Some(id);
                }
            }
        }
        id
    }

    pub fn load_dom(root: &Node, url: &str) -> Self {
        let nodes = {
            let mut n = vec![];
            Self::flatten(root, &mut n);
            n
        };
        let mut bridge = Self { nodes, body_id: None, write_buffer: String::new(), current_url: url.to_string(), pending_navigation: None, next_timer_id: 1, timers: vec![], event_listeners: vec![] };
        bridge.body_id = bridge.find_body();
        bridge
    }

    fn find_body(&self) -> Option<u32> {
        self.find_tag(0, "body")
    }

    fn find_tag(&self, start: u32, tag: &str) -> Option<u32> {
        let tag_lower = tag.to_lowercase();
        let mut stack = vec![start];
        while let Some(id) = stack.pop() {
            if let Some(node) = self.nodes.get(id as usize) {
                if !node.is_text && !node.is_document && node.tag == tag_lower {
                    return Some(id);
                }
                for &child in &node.children {
                    stack.push(child);
                }
            }
        }
        None
    }

    // ── Convert back to crate DOM tree ──────────────────────────────

    fn to_dom_node(&self, id: u32) -> Node {
        let node = &self.nodes[id as usize];
        if node.is_document {
            let children: Vec<Node> = node.children.iter().map(|&c| self.to_dom_node(c)).collect();
            Node { children, node_type: NodeType::Document }
        } else if node.is_text {
            Node { children: vec![], node_type: NodeType::Text(node.text.clone()) }
        } else {
            let children: Vec<Node> = node.children.iter().map(|&c| self.to_dom_node(c)).collect();
            Node { children, node_type: NodeType::Element(ElementData { tag_name: node.tag.clone(), attributes: node.attrs.clone() }) }
        }
    }

    pub fn to_dom(&self) -> Node {
        if self.nodes.is_empty() {
            return Node::new_document();
        }
        self.to_dom_node(0)
    }

    // ── DOM manipulation methods ────────────────────────────────────

    pub fn create_element(&mut self, tag: &str) -> u32 {
        let id = self.nodes.len() as u32;
        self.nodes.push(FlatNode::element(tag));
        id
    }

    pub fn create_text_node(&mut self, text: &str) -> u32 {
        let id = self.nodes.len() as u32;
        self.nodes.push(FlatNode::text(text));
        id
    }

    pub fn append_child(&mut self, parent_id: u32, child_id: u32) {
        if parent_id == child_id { return; }
        if let Some(parent) = self.nodes.get_mut(parent_id as usize) {
            parent.children.push(child_id);
        }
        if let Some(child) = self.nodes.get_mut(child_id as usize) {
            child.parent = Some(parent_id);
        }
    }

    pub fn set_attribute(&mut self, node_id: u32, name: &str, value: &str) {
        if let Some(node) = self.nodes.get_mut(node_id as usize) {
            if !node.is_text && !node.is_document {
                node.attrs.insert(name.to_string(), value.to_string());
            }
        }
    }

    pub fn get_attribute(&self, node_id: u32, name: &str) -> Option<String> {
        self.nodes.get(node_id as usize).and_then(|n| n.attrs.get(name).cloned())
    }

    pub fn get_text_content(&self, node_id: u32) -> String {
        let mut out = String::new();
        self.collect_text(node_id, &mut out);
        out
    }

    fn collect_text(&self, id: u32, out: &mut String) {
        if let Some(node) = self.nodes.get(id as usize) {
            if node.is_text {
                out.push_str(&node.text);
            } else if !node.is_document {
                for &child in &node.children {
                    self.collect_text(child, out);
                }
            }
        }
    }

    pub fn set_text_content(&mut self, node_id: u32, text: &str) {
        {
            let node = self.nodes.get_mut(node_id as usize);
            if let Some(node) = node {
                if node.is_text || node.is_document { return; }
                node.children.clear();
            }
        }
        let text_id = self.nodes.len() as u32;
        self.nodes.push(FlatNode::text(text));
        if let Some(n) = self.nodes.get_mut(text_id as usize) {
            n.parent = Some(node_id);
        }
        if let Some(node) = self.nodes.get_mut(node_id as usize) {
            node.children.push(text_id);
        }
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<u32> {
        self.find_attr(0, "id", id)
    }

    fn find_attr(&self, start: u32, attr: &str, value: &str) -> Option<u32> {
        let mut stack = vec![start];
        while let Some(id) = stack.pop() {
            if let Some(node) = self.nodes.get(id as usize) {
                if !node.is_text && !node.is_document {
                    if node.attrs.get(attr).map_or(false, |v| v == value) {
                        return Some(id);
                    }
                }
                for &child in &node.children {
                    stack.push(child);
                }
            }
        }
        None
    }

    pub fn get_tag_name(&self, node_id: u32) -> Option<String> {
        self.nodes.get(node_id as usize).map(|n| {
            if n.is_document { "document".to_string() }
            else if n.is_text { "text".to_string() }
            else { n.tag.to_uppercase() }
        })
    }

    // ── innerHTML set: parse simple HTML fragment ───────────────────

    pub fn set_inner_html(&mut self, node_id: u32, html: &str) {
        if let Some(node) = self.nodes.get_mut(node_id as usize) {
            if node.is_text || node.is_document { return; }
            node.children.clear();
        }
        let children = self.parse_html_fragment(html);
        for child_id in children {
            if let Some(child) = self.nodes.get_mut(child_id as usize) {
                child.parent = Some(node_id);
            }
            if let Some(parent) = self.nodes.get_mut(node_id as usize) {
                parent.children.push(child_id);
            }
        }
    }

    fn parse_html_fragment(&mut self, html: &str) -> Vec<u32> {
        let mut result = vec![];
        let html = html.trim();
        if html.is_empty() { return result; }

        let mut pos = 0;
        let chars: Vec<char> = html.chars().collect();

        while pos < chars.len() {
            if chars[pos] == '<' {
                if pos + 1 < chars.len() && chars[pos + 1] == '/' {
                    let end = html[pos..].find('>');
                    if let Some(end) = end {
                        pos += end + 1;
                        continue;
                    }
                    break;
                }
                if pos + 1 < chars.len() && chars[pos + 1] == '!' {
                    if html[pos..].starts_with("<!--") {
                        let end = html[pos..].find("-->");
                        if let Some(end) = end {
                            pos += end + 3;
                            continue;
                        }
                        break;
                    }
                    pos += 1;
                    continue;
                }
                let tag_end = html[pos..].find(|c: char| c == '>' || c == ' ' || c == '\t' || c == '\n');
                if let Some(tag_end) = tag_end {
                    let tag_name = html[pos+1..pos+tag_end].to_lowercase();
                    let is_self_closing = ["br", "hr", "img", "input", "meta", "link"];
                    let self_closing = is_self_closing.contains(&tag_name.as_str());

                    let mut attr_end = tag_end;
                    loop {
                        if pos + attr_end >= chars.len() || chars[pos + attr_end] == '>' {
                            break;
                        }
                        attr_end += 1;
                    }
                    if pos + attr_end >= chars.len() { break; }

                    let attrs_part = &html[pos+tag_end..pos+attr_end];
                    let attrs = self.parse_attributes(attrs_part);

                    let el_id = self.nodes.len() as u32;
                    self.nodes.push(FlatNode::element(&tag_name));
                    self.nodes[el_id as usize].attrs = attrs;
                    result.push(el_id);

                    pos += attr_end + 1;

                    if !self_closing && tag_name != "script" && tag_name != "style" {
                        let closing = format!("</{}>", tag_name);
                        if let Some(closing_pos) = html[pos..].find(&closing) {
                            let inner = &html[pos..pos + closing_pos];
                            let inner_children = self.parse_html_fragment(inner);
                            for child_id in inner_children {
                                if let Some(child) = self.nodes.get_mut(child_id as usize) {
                                    child.parent = Some(el_id);
                                }
                                self.nodes[el_id as usize].children.push(child_id);
                            }
                            pos += closing_pos + closing.len();
                        } else {
                            let inner = &html[pos..];
                            let inner_children = self.parse_html_fragment(inner);
                            for child_id in inner_children {
                                if let Some(child) = self.nodes.get_mut(child_id as usize) {
                                    child.parent = Some(el_id);
                                }
                                self.nodes[el_id as usize].children.push(child_id);
                            }
                            pos = chars.len();
                        }
                    }
                } else {
                    pos += 1;
                }
            } else {
                let text_end = html[pos..].find('<').unwrap_or(html.len() - pos);
                let text = &html[pos..pos + text_end];
                if !text.trim().is_empty() || text_end > 0 {
                    let text_id = self.nodes.len() as u32;
                    self.nodes.push(FlatNode::text(text));
                    result.push(text_id);
                }
                pos += text_end;
            }
        }
        result
    }

    fn parse_attributes(&self, s: &str) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        let s = s.trim();
        if s.is_empty() { return attrs; }
        let mut key = String::new();
        let mut value = String::new();
        let mut in_quote = false;
        let mut quote_char = '"';
        let mut after_eq = false;

        for c in s.chars() {
            if in_quote {
                if c == quote_char {
                    in_quote = false;
                    attrs.insert(key.trim().to_string(), std::mem::take(&mut value));
                    key.clear();
                    after_eq = false;
                } else {
                    value.push(c);
                }
            } else if c == '"' || c == '\'' {
                if after_eq {
                    in_quote = true;
                    quote_char = c;
                }
            } else if c == '=' {
                after_eq = true;
            } else if c.is_whitespace() {
                if !key.is_empty() && !after_eq {
                    attrs.insert(std::mem::take(&mut key).trim().to_string(), String::new());
                }
                if after_eq && !in_quote {
                    after_eq = false;
                    key.clear();
                }
            } else if after_eq {
                value.push(c);
            } else {
                key.push(c);
            }
        }
        if !key.is_empty() {
            if after_eq && !in_quote {
                attrs.insert(key.trim().to_string(), std::mem::take(&mut value));
            } else {
                attrs.insert(key.trim().to_string(), String::new());
            }
        }
        attrs
    }

    // ── querySelector / querySelectorAll ────────────────────────────

    pub fn query_selector(&self, node_id: u32, selector: &str) -> Option<u32> {
        let sel = parse_complex(selector)?;
        self.query_sel(node_id, &sel, false).into_iter().next()
    }

    pub fn query_selector_all(&self, node_id: u32, selector: &str) -> Vec<u32> {
        if let Some(sel) = parse_complex(selector) {
            self.query_sel(node_id, &sel, true).into_iter().collect()
        } else { vec![] }
    }

    fn query_sel(&self, start: u32, sel: &ComplexSel, all: bool) -> Vec<u32> {
        let mut results = vec![];
        let mut stack: Vec<u32> = self.nodes.get(start as usize).map(|n| n.children.clone()).unwrap_or_default();
        while let Some(id) = stack.pop() {
            if matches_complex(&self.nodes, id, sel) {
                results.push(id);
                if !all { return results; }
            }
            if let Some(node) = self.nodes.get(id as usize) {
                for &child in node.children.iter().rev() {
                    stack.push(child);
                }
            }
        }
        results
    }

    // ── DOM traversal ───────────────────────────────────────────────

    pub fn get_parent(&self, node_id: u32) -> Option<u32> {
        self.nodes.get(node_id as usize).and_then(|n| n.parent)
    }

    pub fn get_children(&self, node_id: u32) -> Vec<u32> {
        self.nodes.get(node_id as usize).map(|n|
            n.children.iter().filter(|&&id| self.nodes.get(id as usize).map_or(false, |c| !c.is_text)).copied().collect()
        ).unwrap_or_default()
    }

    pub fn get_child_nodes(&self, node_id: u32) -> Vec<u32> {
        self.nodes.get(node_id as usize).map(|n| n.children.clone()).unwrap_or_default()
    }

    pub fn get_first_child(&self, node_id: u32) -> Option<u32> {
        self.nodes.get(node_id as usize).and_then(|n| n.children.first().copied())
    }

    pub fn get_last_child(&self, node_id: u32) -> Option<u32> {
        self.nodes.get(node_id as usize).and_then(|n| n.children.last().copied())
    }

    pub fn get_next_sibling(&self, node_id: u32) -> Option<u32> {
        let parent_id = self.nodes.get(node_id as usize)?.parent?;
        let siblings = &self.nodes.get(parent_id as usize)?.children;
        let idx = siblings.iter().position(|&id| id == node_id)?;
        siblings.get(idx + 1).copied()
    }

    pub fn get_previous_sibling(&self, node_id: u32) -> Option<u32> {
        let parent_id = self.nodes.get(node_id as usize)?.parent?;
        let siblings = &self.nodes.get(parent_id as usize)?.children;
        let idx = siblings.iter().position(|&id| id == node_id)?;
        if idx > 0 { siblings.get(idx - 1).copied() } else { None }
    }

    pub fn get_child_element_count(&self, node_id: u32) -> u32 {
        self.get_children(node_id).len() as u32
    }

    // ── Timer methods ───────────────────────────────────────────────

    pub fn set_timeout(&mut self, source: String, delay_ms: u64) -> u32 {
        let id = self.next_timer_id;
        self.next_timer_id += 1;
        let fire_at = std::time::Instant::now() + std::time::Duration::from_millis(delay_ms);
        self.timers.push(TimerEntry { id, source, delay_ms, is_interval: false, fire_at });
        id
    }

    pub fn set_interval(&mut self, source: String, delay_ms: u64) -> u32 {
        let id = self.next_timer_id;
        self.next_timer_id += 1;
        let fire_at = std::time::Instant::now() + std::time::Duration::from_millis(delay_ms);
        self.timers.push(TimerEntry { id, source, delay_ms, is_interval: true, fire_at });
        id
    }

    pub fn clear_timer(&mut self, id: u32) {
        self.timers.retain(|t| t.id != id);
    }

    pub fn has_pending_timers(&self) -> bool {
        !self.timers.is_empty()
    }

    /// Returns (timer_id, source_code) pairs for expired timers.
    /// Re-registers interval timers for their next fire.
    pub fn poll_timers(&mut self) -> Vec<(u32, String)> {
        let now = std::time::Instant::now();
        let mut ready = vec![];
        let mut i = 0;
        while i < self.timers.len() {
            if self.timers[i].fire_at <= now {
                let entry = self.timers.remove(i);
                ready.push((entry.id, entry.source.clone()));
                if entry.is_interval {
                    let new_id = self.next_timer_id;
                    self.next_timer_id += 1;
                    let fire_at = now + std::time::Duration::from_millis(entry.delay_ms);
                    self.timers.push(TimerEntry { id: new_id, source: entry.source, delay_ms: entry.delay_ms, is_interval: true, fire_at });
                }
            } else {
                i += 1;
            }
        }
        ready
    }

    // ── Event listener methods ──────────────────────────────────────

    pub fn add_event_listener(&mut self, node_id: u32, event_type: String, source: String) {
        self.event_listeners.push(EventListenerEntry { node_id, event_type, source });
    }

    pub fn remove_event_listener(&mut self, node_id: u32, event_type: String, source: String) {
        self.event_listeners.retain(|e| !(e.node_id == node_id && e.event_type == event_type && e.source == source));
    }

    pub fn get_event_listeners(&self, node_id: u32, event_type: &str) -> Vec<String> {
        self.event_listeners.iter()
            .filter(|e| e.node_id == node_id && e.event_type == event_type)
            .map(|e| e.source.clone())
            .collect()
    }

    /// Returns (source, node_id) for all matching event listeners, including on ancestor nodes.
    pub fn get_event_listeners_bubbling(&self, node_id: u32, event_type: &str) -> Vec<(String, u32)> {
        let mut results = vec![];
        let mut current = Some(node_id);
        while let Some(nid) = current {
            for e in &self.event_listeners {
                if e.node_id == nid && e.event_type == event_type {
                    results.push((e.source.clone(), nid));
                }
            }
            current = self.nodes.get(nid as usize).and_then(|n| n.parent);
        }
        results
    }

    // ── Style methods ───────────────────────────────────────────────

    pub fn set_style_property(&mut self, node_id: u32, property: &str, value: &str) {
        if let Some(node) = self.nodes.get_mut(node_id as usize) {
            if !node.is_text && !node.is_document {
                node.inline_styles.insert(property.to_string(), value.to_string());
            }
        }
    }

    pub fn get_style_property(&self, node_id: u32, property: &str) -> String {
        self.nodes.get(node_id as usize)
            .and_then(|n| n.inline_styles.get(property))
            .cloned()
            .unwrap_or_default()
    }

    // ── Location methods ────────────────────────────────────────────

    pub fn get_location_href(&self) -> String {
        self.current_url.clone()
    }

    pub fn set_location_href(&mut self, href: String) {
        self.pending_navigation = Some(href);
    }

    pub fn get_location_hostname(&self) -> String {
        parse_url(&self.current_url).hostname
    }

    pub fn get_location_pathname(&self) -> String {
        parse_url(&self.current_url).pathname
    }

    pub fn get_location_protocol(&self) -> String {
        parse_url(&self.current_url).protocol
    }

    pub fn get_location_port(&self) -> String {
        parse_url(&self.current_url).port
    }

    pub fn get_location_search(&self) -> String {
        parse_url(&self.current_url).search
    }

    pub fn get_location_hash(&self) -> String {
        parse_url(&self.current_url).hash
    }

    pub fn location_reload(&mut self) {
        self.pending_navigation = Some(self.current_url.clone());
    }

    pub fn location_assign(&mut self, url: String) {
        self.pending_navigation = Some(url);
    }

    pub fn location_replace(&mut self, url: String) {
        self.pending_navigation = Some(url);
    }

    // ── Fetch (blocking, called from JS) ────────────────────────────

    pub fn fetch_url(&self, url: &str) -> String {
        let resolved = crate::engine::net::resolve_url(url, &self.current_url);
        crate::engine::net::fetch(&resolved)
    }

    // ── Get elements at a point (for click dispatch) ────────────────

    pub fn element_at_point(&self, x: f32, y: f32, elements: &[crate::ui::screens::browser::StyledElement]) -> Option<u32> {
        let mut best_id = None;
        let mut best_area = std::f32::MAX;
        for el in elements.iter() {
            let ex = el.x.max(0.0);
            let ey = el.y.max(0.0);
            let ew = el.width.max(1.0);
            let eh = if el.height > 0.0 && el.height.is_finite() { el.height } else { 30.0 };
            if x >= ex && x <= ex + ew && y >= ey && y <= ey + eh {
                let area = ew * eh;
                if area < best_area {
                    best_area = area;
                    best_id = Some(self.find_node_by_tag_position(&el.tag, ex, ey));
                }
            }
        }
        best_id
    }

    fn find_node_by_tag_position(&self, tag: &str, _x: f32, _y: f32) -> u32 {
        if let Some(body) = self.body_id {
            let candidates = self.query_selector_all(body, tag);
            candidates.into_iter().next().unwrap_or(body)
        } else { 0 }
    }
}

// ── JS Shim ─────────────────────────────────────────────────────────

const SHIM_JS: &str = r#"
(function() {
    if (window.__domShimLoaded) return;
    window.__domShimLoaded = true;
    window.location = {
        href: "",
        reload: function() { __window_reload(); }
    };
    window.navigator = {
        userAgent: "Aether/1.0 (Spatial; Proprietary)",
        platform: "AetherOS"
    };
    window.fetch = function(url) {
        return Promise.resolve({
            json: function() { return Promise.resolve(JSON.parse(__window_fetch(url))); },
            text: function() { return Promise.resolve(__window_fetch(url)); }
        });
    };


    function makeStyleObject(id) {
        var style = {};
        var props = ['color', 'backgroundColor', 'fontSize', 'fontWeight', 'marginTop', 'marginBottom', 'padding', 'border', 'width', 'height', 'display', 'textAlign'];
        for (var i = 0; i < props.length; i++) {
            (function(prop) {
                Object.defineProperty(style, prop, {
                    get: function() { return __getStyleProperty(id, prop); },
                    set: function(val) { __setStyleProperty(id, prop, String(val)); },
                    enumerable: true,
                    configurable: true
                });
            })(props[i]);
        }
        return style;
    }

    function makeElement(id, tagName) {
        return {
            __id: id,
            tagName: tagName || "UNKNOWN",
            nodeType: 1,
            style: makeStyleObject(id),
            appendChild: function(child) {
                if (child && typeof child.__id === 'number') {
                    __dom_appendChild(id, child.__id);
                }
                return child;
            },
            setAttribute: function(name, value) {
                __dom_setAttribute(id, String(name), String(value));
            },
            getAttribute: function(name) {
                return __dom_getAttribute(id, String(name));
            },
            get textContent() {
                return __dom_getTextContent(id);
            },
            set textContent(val) {
                __dom_setTextContent(id, String(val));
            },
            get innerHTML() {
                return __dom_getTextContent(id);
            },
            set innerHTML(val) {
                __dom_setInnerHTML(id, String(val));
            },
            addEventListener: function(type, listener) {
                __dom_addEventListener(id, type, listener);
            },
            removeEventListener: function(type, listener) {
                __dom_removeEventListener(id, type, listener);
            },
            getBoundingClientRect: function() {
                var rect = __dom_getBoundingClientRect(id);
                return {
                    x: rect.x, y: rect.y,
                    width: rect.width, height: rect.height,
                    top: rect.y, left: rect.x,
                    right: rect.x + rect.width, bottom: rect.y + rect.height
                };
            },
            get children() {
                var ids = __dom_getChildren(id);
                return ids.map(childId => makeElement(childId));
            },
            get parentNode() {
                var pid = __dom_getParent(id);
                return pid !== null ? makeElement(pid) : null;
            },
            click: function() {
                __dom_dispatch_click(id);
            }

            get id() {
                return __dom_getAttribute(id, "id") || "";
            },
            set id(val) {
                __dom_setAttribute(id, "id", String(val));
            },
            get className() {
                return __dom_getAttribute(id, "class") || "";
            },
            set className(val) {
                __dom_setAttribute(id, "class", String(val));
            },
            getAttributeNode: function(name) {
                var val = __dom_getAttribute(id, String(name));
                return val !== null ? { value: val } : null;
            },
            hasAttribute: function(name) {
                return __dom_getAttribute(id, String(name)) !== null;
            },
            removeAttribute: function(name) {
                __dom_setAttribute(id, String(name), "");
            },
            addEventListener: function(type, handler) {
                __addEventListener(id, String(type), handler.toString());
            },
            removeEventListener: function(type, handler) {
                __removeEventListener(id, String(type), handler.toString());
            }
        };
    }

    var origDoc = document;
    document.body = makeElement(__dom_bodyId(), "BODY");

    document.createElement = function(tag) {
        var id = __dom_createElement(String(tag));
        return makeElement(id, String(tag).toUpperCase());
    };

    document.createTextNode = function(text) {
        return { __id: __dom_createTextNode(String(text)), nodeType: 3, textContent: text };
    };

    document.getElementById = function(id) {
        var nodeId = __dom_getElementById(String(id));
        if (nodeId < 0) return null;
        var tag = __dom_getTagName(nodeId);
        return makeElement(nodeId, tag);
    };

    document.querySelector = function(sel) {
        var id = __dom_querySelector(document.body.__id, String(sel));
        if (id < 0) return null;
        return makeElement(id, __dom_getTagName(id));
    };

    document.querySelectorAll = function(sel) {
        var ids = __dom_querySelectorAll(document.body.__id, String(sel));
        var result = [];
        for (var i = 0; i < ids.length; i++) {
            result.push(makeElement(ids[i], __dom_getTagName(ids[i])));
        }
        return result;
    };

    document.getElementsByTagName = function(tag) {
        return document.querySelectorAll(tag);
    };

    document.documentElement = document.body;
    document.head = document.body;

    var EProto = {
        querySelector: function(sel) {
            var id = __dom_querySelector(this.__id, String(sel));
            if (id < 0) return null;
            return makeElement(id, __dom_getTagName(id));
        },
        querySelectorAll: function(sel) {
            var ids = __dom_querySelectorAll(this.__id, String(sel));
            var result = [];
            for (var i = 0; i < ids.length; i++) {
                result.push(makeElement(ids[i], __dom_getTagName(ids[i])));
            }
            return result;
        },
        get parentNode() {
            var pid = __dom_getParent(this.__id);
            return pid >= 0 ? makeElement(pid, __dom_getTagName(pid)) : null;
        },
        get children() {
            var ids = __dom_getChildren(this.__id);
            var result = [];
            for (var i = 0; i < ids.length; i++) {
                result.push(makeElement(ids[i], __dom_getTagName(ids[i])));
            }
            return result;
        },
        get childNodes() {
            var ids = __dom_getChildNodes(this.__id);
            var result = [];
            for (var i = 0; i < ids.length; i++) {
                var tag = __dom_getTagName(ids[i]);
                if (tag === "text") {
                    result.push({ nodeType: 3, textContent: __dom_getTextContent(ids[i]) });
                } else {
                    result.push(makeElement(ids[i], tag));
                }
            }
            return result;
        },
        get firstChild() {
            var id = __dom_getFirstChild(this.__id);
            if (id < 0) return null;
            var tag = __dom_getTagName(id);
            if (tag === "text") return { nodeType: 3, textContent: __dom_getTextContent(id) };
            return makeElement(id, tag);
        },
        get lastChild() {
            var id = __dom_getLastChild(this.__id);
            if (id < 0) return null;
            var tag = __dom_getTagName(id);
            if (tag === "text") return { nodeType: 3, textContent: __dom_getTextContent(id) };
            return makeElement(id, tag);
        },
        get nextSibling() {
            var id = __dom_getNextSibling(this.__id);
            if (id < 0) return null;
            var tag = __dom_getTagName(id);
            if (tag === "text") return { nodeType: 3, textContent: __dom_getTextContent(id) };
            return makeElement(id, tag);
        },
        get previousSibling() {
            var id = __dom_getPreviousSibling(this.__id);
            if (id < 0) return null;
            var tag = __dom_getTagName(id);
            if (tag === "text") return { nodeType: 3, textContent: __dom_getTextContent(id) };
            return makeElement(id, tag);
        },
        get childElementCount() {
            return __dom_getChildElementCount(this.__id);
        }
    };

    var _origMake = makeElement;
    makeElement = function(id, tagName) {
        var el = _origMake(id, tagName);
        for (var key in EProto) {
            if (EProto.hasOwnProperty(key)) {
                Object.defineProperty(el, key, Object.getOwnPropertyDescriptor(EProto, key));
            }
        }
        return el;
    };

    var origBodyId = document.body.__id;
    document.body = makeElement(origBodyId, "BODY");

    var _origCreateElement = document.createElement;
    document.createElement = function(tag) {
        var id = __dom_createElement(String(tag));
        return makeElement(id, String(tag).toUpperCase());
    };

    document.getElementById = function(id) {
        var nodeId = __dom_getElementById(String(id));
        if (nodeId < 0) return null;
        var tag = __dom_getTagName(nodeId);
        return makeElement(nodeId, tag);
    };

    // ── window.location ─────────────────────────────────────────────

    window.location = {};
    (function() {
        var locProps = ['href', 'hostname', 'pathname', 'protocol', 'port', 'search', 'hash'];
        var getters = {
            href: __getLocationHref,
            hostname: __getLocationHostname,
            pathname: __getLocationPathname,
            protocol: __getLocationProtocol,
            port: __getLocationPort,
            search: __getLocationSearch,
            hash: __getLocationHash
        };
        for (var i = 0; i < locProps.length; i++) {
            (function(prop) {
                Object.defineProperty(window.location, prop, {
                    get: function() { return getters[prop](); },
                    enumerable: true,
                    configurable: true
                });
            })(locProps[i]);
        }
        Object.defineProperty(window.location, 'href', {
            get: function() { return __getLocationHref(); },
            set: function(val) { __setLocationHref(String(val)); },
            enumerable: true,
            configurable: true
        });
        window.location.reload = function() { __locationReload(); };
        window.location.assign = function(url) { __locationAssign(String(url)); };
        window.location.replace = function(url) { __locationReplace(String(url)); };
    })();

    // ── window.setTimeout / setInterval ─────────────────────────────

    window.setTimeout = function(fn, ms) {
        var src = fn.toString();
        return __setTimeout(src, ms || 0);
    };

    window.setInterval = function(fn, ms) {
        var src = fn.toString();
        return __setInterval(src, ms || 0);
    };

    window.clearTimeout = function(id) {
        __clearTimer(id);
    };

    window.clearInterval = function(id) {
        __clearTimer(id);
    };

    // ── window.fetch ────────────────────────────────────────────────

    window.fetch = function(url) {
        var text = __fetch(String(url));
        return {
            ok: true,
            status: 200,
            statusText: "OK",
            url: url,
            text: function() { return Promise.resolve(text); },
            json: function() { return Promise.resolve(JSON.parse(text)); }
        };
    };

    // ── console improvements ────────────────────────────────────────

    var _origLog = console.log;
    var _origWarn = console.warn;
    var _origError = console.error;

    console.assert = function(condition, msg) {
        if (!condition) {
            _origError("Assertion failed" + (msg ? ": " + msg : ""));
        }
    };

    console.count = (function() {
        var counts = {};
        return function(label) {
            label = label || "default";
            counts[label] = (counts[label] || 0) + 1;
            _origLog(label + ": " + counts[label]);
        };
    })();

    console.time = (function() {
        var timers = {};
        return function(label) {
            label = label || "default";
            timers[label] = Date.now();
        };
    })();

    console.timeEnd = (function() {
        var timers = {};
        return function(label) {
            label = label || "default";
            var start = timers[label];
            if (start !== undefined) {
                _origLog(label + ": " + (Date.now() - start) + "ms");
                delete timers[label];
            }
        };
    })();

    console.group = function(label) {
        _origLog("> " + (label || "group"));
    };

    console.groupEnd = function() {};

    console.table = function(data) {
        if (Array.isArray(data)) {
            for (var i = 0; i < data.length; i++) {
                _origLog("[" + i + "] " + JSON.stringify(data[i]));
            }
        } else if (typeof data === 'object' && data !== null) {
            _origLog(JSON.stringify(data, null, 2));
        } else {
            _origLog(String(data));
        }
    };

    // ── window.XMLHttpRequest ───────────────────────────────────────

    window.XMLHttpRequest = function() {
        this.readyState = 0;
        this.status = 0;
        this.statusText = "";
        this.responseText = "";
        this._method = "GET";
        this._url = "";
        this._async = true;
    };

    window.XMLHttpRequest.prototype.open = function(method, url, async) {
        this._method = method || "GET";
        this._url = String(url);
        this._async = async !== false;
        this.readyState = 1;
    };

    window.XMLHttpRequest.prototype.send = function() {
        this.readyState = 2;
        this.responseText = __fetch(this._url);
        this.status = 200;
        this.statusText = "OK";
        this.readyState = 4;
        if (typeof this.onreadystatechange === "function") {
            this.onreadystatechange();
        }
    };

    window.XMLHttpRequest.prototype.setRequestHeader = function() {};
    window.XMLHttpRequest.prototype.getResponseHeader = function() { return null; };

    // ── window.navigator ────────────────────────────────────────────

    window.navigator = {
        userAgent: "AetherBrowser/0.1",
        platform: "Rust",
        language: "en-US",
        cookieEnabled: false
    };

})();
"#;

// ── Register browser APIs in JS context ────────────────────────────

/// Register all browser APIs in the given JS context.
/// Must be called for each script execution if bridging is desired.
pub fn register_browser_api(
    ctx: &rquickjs::Ctx<'_>,
    bridge: &Arc<Mutex<JsBridge>>,
) -> Result<(), rquickjs::Error> {
    use rquickjs::function::{Func, Rest};
    use rquickjs::{Object, Function};

    let globals = ctx.globals();

    // ── console ─────────────────────────────────────────────────────
    let console = Object::new(ctx.clone())?;
    console.set("log", Func::new(|args: Rest<String>| {
        println!("[JS] {}", args.into_inner().join(" "));
    }))?;
    console.set("warn", Func::new(|args: Rest<String>| {
        eprintln!("[JS WARN] {}", args.into_inner().join(" "));
    }))?;
    console.set("error", Func::new(|args: Rest<String>| {
        eprintln!("[JS ERROR] {}", args.into_inner().join(" "));
    }))?;
    globals.set("console", console)?;

    // ── document (base) + document.write ────────────────────────────
    let document = Object::new(ctx.clone())?;
    {
        let bridge_clone = Arc::clone(bridge);
        document.set("write", Func::new(move |text: String| {
            if let Ok(mut b) = bridge_clone.lock() {
                b.document_write(&text);
            }
        }))?;
    }
    globals.set("document", document)?;

    // ── Low-level DOM functions ─────────────────────────────────────

    let b1 = Arc::clone(bridge);
    let fn_create = Function::new(ctx.clone(), move |tag: String| -> i32 {
        if let Ok(mut b) = b1.lock() {
            b.create_element(&tag) as i32
        } else { -1 }
    })?;
    fn_create.set_name("__dom_createElement")?;
    globals.set("__dom_createElement", fn_create)?;

    let b1 = Arc::clone(bridge);
    let fn_create_text = Function::new(ctx.clone(), move |text: String| -> i32 {
        if let Ok(mut b) = b1.lock() {
            b.create_text_node(&text) as i32
        } else { -1 }
    })?;
    fn_create_text.set_name("__dom_createTextNode")?;
    globals.set("__dom_createTextNode", fn_create_text)?;

    let b1 = Arc::clone(bridge);
    let fn_append = Function::new(ctx.clone(), move |parent_id: i32, child_id: i32| {
        if let Ok(mut b) = b1.lock() {
            b.append_child(parent_id as u32, child_id as u32);
        }
    })?;
    fn_append.set_name("__dom_appendChild")?;
    globals.set("__dom_appendChild", fn_append)?;

    let b1 = Arc::clone(bridge);
    let fn_set_attr = Function::new(ctx.clone(), move |node_id: i32, name: String, value: String| {
        if let Ok(mut b) = b1.lock() {
            b.set_attribute(node_id as u32, &name, &value);
        }
    })?;
    fn_set_attr.set_name("__dom_setAttribute")?;
    globals.set("__dom_setAttribute", fn_set_attr)?;

    let b1 = Arc::clone(bridge);
    let fn_get_attr = Function::new(ctx.clone(), move |node_id: i32, name: String| -> Option<String> {
        if let Ok(b) = b1.lock() {
            b.get_attribute(node_id as u32, &name)
        } else { None }
    })?;
    fn_get_attr.set_name("__dom_getAttribute")?;
    globals.set("__dom_getAttribute", fn_get_attr)?;

    let b1 = Arc::clone(bridge);
    let fn_get_text = Function::new(ctx.clone(), move |node_id: i32| -> String {
        if let Ok(b) = b1.lock() {
            b.get_text_content(node_id as u32)
        } else { String::new() }
    })?;
    fn_get_text.set_name("__dom_getTextContent")?;
    globals.set("__dom_getTextContent", fn_get_text)?;

    let b1 = Arc::clone(bridge);
    let fn_set_text = Function::new(ctx.clone(), move |node_id: i32, text: String| {
        if let Ok(mut b) = b1.lock() {
            b.set_text_content(node_id as u32, &text);
        }
    })?;
    fn_set_text.set_name("__dom_setTextContent")?;
    globals.set("__dom_setTextContent", fn_set_text)?;

    let b1 = Arc::clone(bridge);
    let fn_set_html = Function::new(ctx.clone(), move |node_id: i32, html: String| {
        if let Ok(mut b) = b1.lock() {
            b.set_inner_html(node_id as u32, &html);
        }
    })?;
    fn_set_html.set_name("__dom_setInnerHTML")?;
    globals.set("__dom_setInnerHTML", fn_set_html)?;

    let b1 = Arc::clone(bridge);
    let fn_get_by_id = Function::new(ctx.clone(), move |id: String| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_element_by_id(&id).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_get_by_id.set_name("__dom_getElementById")?;
    globals.set("__dom_getElementById", fn_get_by_id)?;

    let b1 = Arc::clone(bridge);
    let fn_body_id = Function::new(ctx.clone(), move || -> i32 {
        if let Ok(b) = b1.lock() {
            b.body_id.map(|v| v as i32).unwrap_or(0)
        } else { 0 }
    })?;
    fn_body_id.set_name("__dom_bodyId")?;
    globals.set("__dom_bodyId", fn_body_id)?;

    let b1 = Arc::clone(bridge);
    let fn_tag_name = Function::new(ctx.clone(), move |node_id: i32| -> String {
        if let Ok(b) = b1.lock() {
            b.get_tag_name(node_id as u32).unwrap_or_default()
        } else { String::new() }
    })?;
    fn_tag_name.set_name("__dom_getTagName")?;
    globals.set("__dom_getTagName", fn_tag_name)?;

    // ── querySelector / querySelectorAll ────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_qs = Function::new(ctx.clone(), move |node_id: i32, sel: String| -> i32 {
        if let Ok(b) = b1.lock() {
            b.query_selector(node_id as u32, &sel).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_qs.set_name("__dom_querySelector")?;
    globals.set("__dom_querySelector", fn_qs)?;

    let b1 = Arc::clone(bridge);
    let fn_qsa = Function::new(ctx.clone(), move |node_id: i32, sel: String| -> Vec<i32> {
        if let Ok(b) = b1.lock() {
            b.query_selector_all(node_id as u32, &sel).into_iter().map(|v| v as i32).collect()
        } else { vec![] }
    })?;
    fn_qsa.set_name("__dom_querySelectorAll")?;
    globals.set("__dom_querySelectorAll", fn_qsa)?;

    // ── DOM traversal ───────────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_parent = Function::new(ctx.clone(), move |node_id: i32| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_parent(node_id as u32).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_parent.set_name("__dom_getParent")?;
    globals.set("__dom_getParent", fn_parent)?;

    let b1 = Arc::clone(bridge);
    let fn_children = Function::new(ctx.clone(), move |node_id: i32| -> Vec<i32> {
        if let Ok(b) = b1.lock() {
            b.get_children(node_id as u32).into_iter().map(|v| v as i32).collect()
        } else { vec![] }
    })?;
    fn_children.set_name("__dom_getChildren")?;
    globals.set("__dom_getChildren", fn_children)?;

    let b1 = Arc::clone(bridge);
    let fn_child_nodes = Function::new(ctx.clone(), move |node_id: i32| -> Vec<i32> {
        if let Ok(b) = b1.lock() {
            b.get_child_nodes(node_id as u32).into_iter().map(|v| v as i32).collect()
        } else { vec![] }
    })?;
    fn_child_nodes.set_name("__dom_getChildNodes")?;
    globals.set("__dom_getChildNodes", fn_child_nodes)?;

    let b1 = Arc::clone(bridge);
    let fn_first = Function::new(ctx.clone(), move |node_id: i32| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_first_child(node_id as u32).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_first.set_name("__dom_getFirstChild")?;
    globals.set("__dom_getFirstChild", fn_first)?;

    let b1 = Arc::clone(bridge);
    let fn_last = Function::new(ctx.clone(), move |node_id: i32| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_last_child(node_id as u32).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_last.set_name("__dom_getLastChild")?;
    globals.set("__dom_getLastChild", fn_last)?;

    let b1 = Arc::clone(bridge);
    let fn_next = Function::new(ctx.clone(), move |node_id: i32| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_next_sibling(node_id as u32).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_next.set_name("__dom_getNextSibling")?;
    globals.set("__dom_getNextSibling", fn_next)?;

    let b1 = Arc::clone(bridge);
    let fn_prev = Function::new(ctx.clone(), move |node_id: i32| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_previous_sibling(node_id as u32).map(|v| v as i32).unwrap_or(-1)
        } else { -1 }
    })?;
    fn_prev.set_name("__dom_getPreviousSibling")?;
    globals.set("__dom_getPreviousSibling", fn_prev)?;

    let b1 = Arc::clone(bridge);
    let fn_cec = Function::new(ctx.clone(), move |node_id: i32| -> i32 {
        if let Ok(b) = b1.lock() {
            b.get_child_element_count(node_id as u32) as i32
        } else { 0 }
    })?;
    fn_cec.set_name("__dom_getChildElementCount")?;
    globals.set("__dom_getChildElementCount", fn_cec)?;

    // ── Timer functions ─────────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_set_timeout = Function::new(ctx.clone(), move |source: String, delay: i32| -> i32 {
        if let Ok(mut b) = b1.lock() {
            b.set_timeout(source, delay.max(0) as u64) as i32
        } else { -1 }
    })?;
    fn_set_timeout.set_name("__setTimeout")?;
    globals.set("__setTimeout", fn_set_timeout)?;

    let b1 = Arc::clone(bridge);
    let fn_set_interval = Function::new(ctx.clone(), move |source: String, delay: i32| -> i32 {
        if let Ok(mut b) = b1.lock() {
            b.set_interval(source, delay.max(0) as u64) as i32
        } else { -1 }
    })?;
    fn_set_interval.set_name("__setInterval")?;
    globals.set("__setInterval", fn_set_interval)?;

    let b1 = Arc::clone(bridge);
    let fn_clear_timer = Function::new(ctx.clone(), move |id: i32| {
        if let Ok(mut b) = b1.lock() {
            b.clear_timer(id as u32);
        }
    })?;
    fn_clear_timer.set_name("__clearTimer")?;
    globals.set("__clearTimer", fn_clear_timer)?;

    // ── Event listener functions ────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_add_el = Function::new(ctx.clone(), move |node_id: i32, event_type: String, source: String| {
        if let Ok(mut b) = b1.lock() {
            b.add_event_listener(node_id as u32, event_type, source);
        }
    })?;
    fn_add_el.set_name("__addEventListener")?;
    globals.set("__addEventListener", fn_add_el)?;

    let b1 = Arc::clone(bridge);
    let fn_remove_el = Function::new(ctx.clone(), move |node_id: i32, event_type: String, source: String| {
        if let Ok(mut b) = b1.lock() {
            b.remove_event_listener(node_id as u32, event_type, source);
        }
    })?;
    fn_remove_el.set_name("__removeEventListener")?;
    globals.set("__removeEventListener", fn_remove_el)?;

    // ── Style functions ─────────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_get_style = Function::new(ctx.clone(), move |node_id: i32, prop: String| -> String {
        if let Ok(b) = b1.lock() {
            b.get_style_property(node_id as u32, &prop)
        } else { String::new() }
    })?;
    fn_get_style.set_name("__getStyleProperty")?;
    globals.set("__getStyleProperty", fn_get_style)?;

    let b1 = Arc::clone(bridge);
    let fn_set_style = Function::new(ctx.clone(), move |node_id: i32, prop: String, value: String| {
        if let Ok(mut b) = b1.lock() {
            b.set_style_property(node_id as u32, &prop, &value);
        }
    })?;
    fn_set_style.set_name("__setStyleProperty")?;
    globals.set("__setStyleProperty", fn_set_style)?;

    // ── Location functions ──────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_loc_href = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_href() } else { String::new() }
    })?;
    fn_loc_href.set_name("__getLocationHref")?;
    globals.set("__getLocationHref", fn_loc_href)?;

    let b1 = Arc::clone(bridge);
    let fn_set_loc_href = Function::new(ctx.clone(), move |href: String| {
        if let Ok(mut b) = b1.lock() { b.set_location_href(href); }
    })?;
    fn_set_loc_href.set_name("__setLocationHref")?;
    globals.set("__setLocationHref", fn_set_loc_href)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_hostname = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_hostname() } else { String::new() }
    })?;
    fn_loc_hostname.set_name("__getLocationHostname")?;
    globals.set("__getLocationHostname", fn_loc_hostname)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_pathname = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_pathname() } else { String::new() }
    })?;
    fn_loc_pathname.set_name("__getLocationPathname")?;
    globals.set("__getLocationPathname", fn_loc_pathname)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_protocol = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_protocol() } else { String::new() }
    })?;
    fn_loc_protocol.set_name("__getLocationProtocol")?;
    globals.set("__getLocationProtocol", fn_loc_protocol)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_port = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_port() } else { String::new() }
    })?;
    fn_loc_port.set_name("__getLocationPort")?;
    globals.set("__getLocationPort", fn_loc_port)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_search = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_search() } else { String::new() }
    })?;
    fn_loc_search.set_name("__getLocationSearch")?;
    globals.set("__getLocationSearch", fn_loc_search)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_hash = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.get_location_hash() } else { String::new() }
    })?;
    fn_loc_hash.set_name("__getLocationHash")?;
    globals.set("__getLocationHash", fn_loc_hash)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_reload = Function::new(ctx.clone(), move || {
        if let Ok(mut b) = b1.lock() { b.location_reload(); }
    })?;
    fn_loc_reload.set_name("__locationReload")?;
    globals.set("__locationReload", fn_loc_reload)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_assign = Function::new(ctx.clone(), move |url: String| {
        if let Ok(mut b) = b1.lock() { b.location_assign(url); }
    })?;
    fn_loc_assign.set_name("__locationAssign")?;
    globals.set("__locationAssign", fn_loc_assign)?;

    let b1 = Arc::clone(bridge);
    let fn_loc_replace = Function::new(ctx.clone(), move |url: String| {
        if let Ok(mut b) = b1.lock() { b.location_replace(url); }
    })?;
    fn_loc_replace.set_name("__locationReplace")?;
    globals.set("__locationReplace", fn_loc_replace)?;

    // ── Fetch function ──────────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_fetch = Function::new(ctx.clone(), move |url: String| -> String {
        if let Ok(b) = b1.lock() {
            b.fetch_url(&url)
        } else { String::new() }
    })?;
    fn_fetch.set_name("__fetch")?;
    globals.set("__fetch", fn_fetch)?;

    // ── Inject JS shim ──────────────────────────────────────────────
    let _ = ctx.eval::<(), _>(SHIM_JS);


    let b_v1 = Arc::clone(bridge);
    let fn_save_pw = Function::new(ctx.clone(), move |url: String, user: String, pass: String| {
        // We'd need a vault instance in JsBridge or similar
        // For now, JsBridge is ephemeral per page, but vault is global.
        // Let's just log it or use a placeholder for now since vault isn't in JsBridge yet.
        println!("[VAULT] Saving password for {} / {}", url, user);
    })?;
    globals.set("__vault_savePassword", fn_save_pw)?;
Ok(())
}
