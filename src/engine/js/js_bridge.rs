use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

use crate::plog;
use crate::engine::dom::{ElementData, Node, NodeType};

// ── Flat DOM node representation ────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct FlatNode {
    pub(crate) parent: Option<u32>,
    pub(crate) children: Vec<u32>,
    pub(crate) tag: String,
    pub(crate) attrs: HashMap<String, String>,
    pub(crate) text: String,
    pub(crate) is_text: bool,
    pub(crate) is_document: bool,
    pub(crate) inline_styles: HashMap<String, String>,
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
pub(crate) enum SimpleSel {
    Universal,
    Tag(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone)]
pub(crate) enum Combinator {
    Descendant,
    Child,
}

#[derive(Debug, Clone)]
pub(crate) struct CompoundSel {
    pub(crate) simples: Vec<SimpleSel>,
}

#[derive(Debug, Clone)]
pub(crate) struct ComplexSel {
    pub(crate) compound: CompoundSel,
    pub(crate) combinator: Option<(Combinator, Box<ComplexSel>)>,
}

pub(crate) fn simple_sel_to_stratus(sel: &SimpleSel) -> crate::engine::stratus::SimpleSelector {
    use crate::engine::stratus::SimpleSelector;
    match sel {
        SimpleSel::Universal => SimpleSelector { tag_name: None, id: None, class: vec![], attribute: None, pseudo_class: None },
        SimpleSel::Tag(t) => SimpleSelector { tag_name: Some(t.clone()), id: None, class: vec![], attribute: None, pseudo_class: None },
        SimpleSel::Class(c) => SimpleSelector { tag_name: None, id: None, class: vec![c.clone()], attribute: None, pseudo_class: None },
        SimpleSel::Id(id) => SimpleSelector { tag_name: None, id: Some(id.clone()), class: vec![], attribute: None, pseudo_class: None },
    }
}

pub(crate) fn matches_simple(node: &FlatNode, sel: &SimpleSel) -> bool {
    if node.is_text || node.is_document { return false; }
    let element = crate::engine::stratus::ElementData::with_attributes(node.tag.clone(), node.attrs.clone());
    let ss = simple_sel_to_stratus(sel);
    ss.matches(&element)
}
pub(crate) fn matches_compound(node: &FlatNode, sel: &CompoundSel) -> bool {
    sel.simples.iter().all(|s| matches_simple(node, s))
}


pub(crate) fn parse_simple_selector(s: &str, pos: &mut usize) -> Option<SimpleSel> {
    let chars: Vec<char> = s.chars().collect();
    if *pos >= chars.len() { return None; }
    match chars[*pos] {
        '*' => { *pos += 1; Some(SimpleSel::Universal) }
        '.' => {
            *pos += 1;
            let start = *pos;
            while *pos < chars.len() && chars[*pos] != '.' && chars[*pos] != '#' && chars[*pos] != '[' && !chars[*pos].is_whitespace() && chars[*pos] != '>' { *pos += 1; }
            if *pos > start { Some(SimpleSel::Class(chars[start..*pos].iter().collect())) } else { None }
        }
        '#' => {
            *pos += 1;
            let start = *pos;
            while *pos < chars.len() && chars[*pos] != '.' && chars[*pos] != '#' && chars[*pos] != '[' && !chars[*pos].is_whitespace() && chars[*pos] != '>' { *pos += 1; }
            if *pos > start { Some(SimpleSel::Id(chars[start..*pos].iter().collect())) } else { None }
        }
        c if c.is_alphanumeric() || c == '-' => {
            let start = *pos;
            while *pos < chars.len() && (chars[*pos].is_alphanumeric() || chars[*pos] == '-') { *pos += 1; }
            Some(SimpleSel::Tag(chars[start..*pos].iter().collect()))
        }
        _ => None,
    }
}

pub(crate) fn parse_compound(s: &str, pos: &mut usize) -> CompoundSel {
    let chars: Vec<char> = s.chars().collect();
    let mut simples = vec![];
    skip_ws(s, pos);
    loop {
        if *pos >= chars.len() { break; }
        if chars[*pos] == '>' || chars[*pos] == '+' || chars[*pos] == '~' || chars[*pos] == ',' { break; }
        if let Some(simple) = parse_simple_selector(s, pos) {
            simples.push(simple);
        } else { break; }
        if *pos < chars.len() && chars[*pos].is_whitespace() { break; }
    }
    if simples.is_empty() { simples.push(SimpleSel::Universal); }
    CompoundSel { simples }
}

pub(crate) fn skip_ws(s: &str, pos: &mut usize) {
    let chars: Vec<char> = s.chars().collect();
    while *pos < chars.len() && chars[*pos].is_whitespace() { *pos += 1; }
}

pub(crate) fn parse_combinator(s: &str, pos: &mut usize) -> Option<Combinator> {
    let chars: Vec<char> = s.chars().collect();
    skip_ws(s, pos);
    if *pos >= chars.len() { return None; }
    if chars[*pos] == '>' {
        *pos += 1;
        skip_ws(s, pos);
        Some(Combinator::Child)
    } else {
        Some(Combinator::Descendant)
    }
}

pub(crate) fn parse_complex(s: &str) -> Option<ComplexSel> {
    let s = s.trim();
    if s.is_empty() { return None; }
    let mut pos = 0;
    let mut compounds = vec![];
    let mut combinators = vec![];

    compounds.push(parse_compound(s, &mut pos));
    while pos < s.len() {
        let saved_pos = pos;
        if let Some(c) = parse_combinator(s, &mut pos) {
            combinators.push(c);
            compounds.push(parse_compound(s, &mut pos));
        } else if pos == saved_pos {
            break;
        }
    }

    let mut res = ComplexSel { compound: compounds.pop()?, combinator: None };
    while !compounds.is_empty() {
        let comp = compounds.pop()?;
        let comb = combinators.pop()?;
        let mut last = &mut res;
        while let Some((_, ref mut next)) = last.combinator {
            last = next;
        }
        last.combinator = Some((comb, Box::new(ComplexSel { compound: comp, combinator: None })));
    }
    Some(res)
}
pub(crate) fn matches_complex(nodes: &[FlatNode], node_id: u32, sel: &ComplexSel) -> bool {
    if !matches_compound(&nodes[node_id as usize], &sel.compound) { return false; }
    match &sel.combinator {
        Some((Combinator::Child, parent_sel)) => {
            if let Some(pid) = nodes[node_id as usize].parent {
                matches_complex(nodes, pid, &*parent_sel)
            } else { false }
        }
        Some((Combinator::Descendant, parent_sel)) => {
            let mut current = nodes[node_id as usize].parent;
            while let Some(pid) = current {
                if matches_complex(nodes, pid, &*parent_sel) { return true; }
                current = nodes[pid as usize].parent;
            }
            false
        }
        None => true,
    }
}
#[derive(Debug)]
pub(crate) struct TimerEntry {
    pub(crate) id: u32,
    pub(crate) source: String,
    pub(crate) delay_ms: u64,
    pub(crate) is_interval: bool,
    pub(crate) fire_at: std::time::Instant,
}

// ── Event listener entry ────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct EventListenerEntry {
    pub(crate) node_id: u32,
    pub(crate) event_type: String,
    pub(crate) source: String,
}

// ── URL parts helper ────────────────────────────────────────────────

pub(crate) struct UrlParts {
    protocol: String,
    hostname: String,
    port: String,
    pathname: String,
    search: String,
    hash: String,
}

#[derive(Debug, Clone)]
pub(crate) struct CookieEntry {
    pub(crate) value: String,
    pub(crate) expires: Option<Instant>,
}

pub(crate) type CookieOriginStore = HashMap<String, HashMap<String, CookieEntry>>;
pub(crate) type OriginStore = HashMap<String, HashMap<String, String>>;

pub(crate) fn is_expired(entry: &CookieEntry) -> bool {
    entry.expires.map_or(false, |exp| Instant::now() >= exp)
}

pub(crate) fn parse_cookie_expiry(cookie_str: &str) -> Option<Instant> {
    // ponytail: only Max-Age and RFC 1123 Expires; no obs-date variants
    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("Max-Age=").or_else(|| part.strip_prefix("max-age=")).or_else(|| part.strip_prefix("MAX-AGE=")) {
            if let Ok(secs) = val.trim().parse::<u64>() {
                return Some(Instant::now() + Duration::from_secs(secs));
            }
        }
        if let Some(val) = part.strip_prefix("Expires=").or_else(|| part.strip_prefix("expires=")) {
            if let Some(instant) = parse_rfc1123_date(val.trim()) {
                return Some(instant);
            }
        }
    }
    None
}

// ponytail: only RFC 1123/850 "Thu, 01 Jan 1970 00:00:00 GMT", not obs-date
pub(crate) fn parse_rfc1123_date(s: &str) -> Option<Instant> {
    let s = s.strip_suffix(" GMT")?;
    let (_wkday, rest) = s.split_once(", ")?;
    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() != 4 { return None; }
    let day: u32 = parts[0].parse().ok()?;
    let month_idx = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]
        .iter().position(|m| m.eq_ignore_ascii_case(parts[1]))? as u32;
    let year: i64 = parts[2].parse().ok()?;
    let time: Vec<&str> = parts[3].split(':').collect();
    if time.len() != 3 { return None; }
    let hour: u32 = time[0].parse().ok()?;
    let min: u32 = time[1].parse().ok()?;
    let sec: u32 = time[2].parse().ok()?;
    // ponytail: naive date→duration, ignores leap seconds
    if year < 1970 {
        return Some(Instant::now()); // pre-1970 = already expired
    }
    let days = (year - 1970) * 365 + (year - 1969) / 4
        + [0,31,59,90,120,151,181,212,243,273,304,334][month_idx as usize] as i64
        + if month_idx > 1 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 1 } else { 0 }
        + day as i64 - 1;
    Some(Instant::now() + Duration::from_secs((days * 86400 + hour as i64 * 3600 + min as i64 * 60 + sec as i64).max(0) as u64))
}

pub(crate) fn sweep_expired_cookies(store: &mut CookieOriginStore) {
    for cookies in store.values_mut() {
        cookies.retain(|_, v| !is_expired(v));
    }
}

pub(crate) fn cookie_store() -> &'static RwLock<CookieOriginStore> {
    static COOKIE_STORE: OnceLock<RwLock<CookieOriginStore>> = OnceLock::new();
    COOKIE_STORE.get_or_init(|| RwLock::new(HashMap::new()))
}

pub(crate) fn local_storage_store() -> &'static RwLock<OriginStore> {
    static LOCAL_STORAGE: OnceLock<RwLock<OriginStore>> = OnceLock::new();
    LOCAL_STORAGE.get_or_init(|| RwLock::new(HashMap::new()))
}

impl Default for UrlParts {
    fn default() -> Self {
        Self { protocol: "https:".into(), hostname: String::new(), port: String::new(), pathname: "/".into(), search: String::new(), hash: String::new() }
    }
}

pub(crate) fn parse_url(url: &str) -> UrlParts {
    let mut parts = UrlParts::default();
    let s = url.trim();
    if s.is_empty() { return parts; }

    // Protocol
    let rest = if let Some(pos) = s.find("://") {
        parts.protocol = s[..pos+1].to_string();
        &s[pos+3..]
    } else if let Some(rest) = s.strip_prefix("//") {
        parts.protocol = "https:".into();
        rest
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

    // Hostname:port (handle IPv6 [::1])
    if rest.starts_with('[') {
        if let Some(bracket_end) = rest.find(']') {
            parts.hostname = rest[..bracket_end + 1].to_string();
            if bracket_end + 1 < rest.len() && rest.as_bytes()[bracket_end + 1] == b':' {
                parts.port = rest[bracket_end + 2..].to_string();
            }
        } else {
            parts.hostname = rest.to_string();
        }
    } else if let Some(pos) = rest.find(':') {
        parts.hostname = rest[..pos].to_string();
        parts.port = rest[pos+1..].to_string();
    } else {
        parts.hostname = rest.to_string();
    }

    parts
}

pub(crate) fn origin_key(url: &str) -> String {
    let parts = parse_url(url);
    if parts.hostname.is_empty() {
        "null".to_string()
    } else if parts.port.is_empty() {
        format!("{}//{}", parts.protocol, parts.hostname)
    } else {
        format!("{}//{}:{}", parts.protocol, parts.hostname, parts.port)
    }
}

// ── JsBridge ────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct JsBridge {
    pub write_buffer: String,
    pub(crate) nodes: Vec<FlatNode>,
    pub body_id: Option<u32>,
    pub current_url: String,
    pub pending_navigation: Option<String>,
    pub doc_title: String,
    pub history_state: String,
    pub pending_history_delta: Option<i32>,
    pub(crate) next_timer_id: u32,
    pub(crate) timers: Vec<TimerEntry>,
    pub(crate) event_listeners: Vec<EventListenerEntry>,
    pub(crate) js_errors: Vec<String>,
}

impl JsBridge {
    pub(crate) fn current_origin(&self) -> String {
        origin_key(&self.current_url)
    }

    fn new_internal(url: &str) -> Self {
        Self {
            write_buffer: String::new(),
            nodes: vec![],
            body_id: None,
            current_url: url.to_string(),
            pending_navigation: None,
            doc_title: String::new(),
            history_state: String::new(),
            pending_history_delta: None,
            next_timer_id: 1,
            timers: vec![],
            event_listeners: vec![],
            js_errors: vec![],
        }
    }

    pub fn new() -> Self {
        Self::new_internal("https://localhost")
    }

    pub fn report_js_error(&mut self, msg: String) {
        self.js_errors.push(msg);
        if self.js_errors.len() > 50 {
            self.js_errors.remove(0);
        }
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
        let mut bridge = Self { nodes, body_id: None, write_buffer: String::new(), current_url: url.to_string(), pending_navigation: None, doc_title: String::new(), history_state: String::new(), pending_history_delta: None, next_timer_id: 1, timers: vec![], event_listeners: vec![], js_errors: vec![] };
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

    fn is_event_handler(name: &str) -> bool {
        let name = name.to_lowercase();
        name.starts_with("on")
            && matches!(&name[2..], "load" | "click" | "dblclick" | "mousedown" | "mouseup"
                | "mouseover" | "mousemove" | "mouseout" | "mouseenter" | "mouseleave"
                | "focus" | "blur" | "keydown" | "keyup" | "keypress"
                | "submit" | "reset" | "change" | "select" | "input" | "invalid"
                | "error" | "abort" | "contextmenu" | "resize" | "scroll" | "wheel"
                | "drag" | "dragend" | "dragenter" | "dragexit" | "dragleave" | "dragover" | "dragstart" | "drop"
                | "pointerdown" | "pointerup" | "pointermove" | "pointerover" | "pointerout"
                | "pointerenter" | "pointerleave" | "pointercancel"
                | "touchstart" | "touchend" | "touchmove" | "touchcancel"
                | "play" | "pause" | "playing" | "ended" | "volumechange" | "waiting"
                | "canplay" | "canplaythrough" | "seeked" | "seeking" | "stalled"
                | "suspend" | "emptied" | "ratechange" | "durationchange"
                | "animationstart" | "animationend" | "animationiteration"
                | "transitionstart" | "transitionend" | "transitionrun" | "transitioncancel"
                | "visibilitychange" | "fullscreenchange" | "fullscreenerror")
    }

    fn is_dangerous_url(value: &str) -> bool {
        let trimmed = value.trim().to_lowercase();
        trimmed.starts_with("javascript:") || trimmed.starts_with("data:")
    }

    pub fn set_attribute(&mut self, node_id: u32, name: &str, value: &str) {
        if let Some(node) = self.nodes.get_mut(node_id as usize) {
            if !node.is_text && !node.is_document {
                if Self::is_event_handler(name) { return; }
                if name.to_lowercase() == "href" || name.to_lowercase() == "src" {
                    if Self::is_dangerous_url(value) { return; }
                }
                if name.to_lowercase() == "srcdoc" { return; }
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

    // ponytail: simple HTML serialization — skips void elements, no attr escaping for special chars
    fn serialize_node(&self, id: u32) -> String {
        let node = match self.nodes.get(id as usize) { Some(n) => n, None => return String::new() };
        if node.is_text { return node.text.clone(); }
        let mut html = String::new();
        html.push('<');
        html.push_str(&node.tag);
        for (k, v) in &node.attrs {
            html.push(' ');
            html.push_str(k);
            html.push_str("=\"");
            html.push_str(v);
            html.push('"');
        }
        html.push('>');
        for &child in &node.children {
            html.push_str(&self.serialize_node(child));
        }
        html.push_str("</");
        html.push_str(&node.tag);
        html.push('>');
        html
    }

    pub fn get_inner_html(&self, node_id: u32) -> String {
        let node = match self.nodes.get(node_id as usize) { Some(n) => n, None => return String::new() };
        if node.is_text || node.is_document { return String::new(); }
        let mut html = String::new();
        for &child in &node.children {
            html.push_str(&self.serialize_node(child));
        }
        html
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
                if !node.is_text && !node.is_document && node.attrs.get(attr).is_some_and(|v| v == value) {
                    return Some(id);
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

    fn sanitize_attrs(attrs: &mut HashMap<String, String>) {
        attrs.retain(|k, v| {
            if Self::is_event_handler(k) { return false; }
            if k.to_lowercase() == "srcdoc" { return false; }
            if (k.to_lowercase() == "href" || k.to_lowercase() == "src") && Self::is_dangerous_url(v) { return false; }
            true
        });
    }

    pub(crate) fn parse_html_fragment(&mut self, html: &str) -> Vec<u32> {
        self.parse_html_fragment_depth(html, 0)
    }

    fn parse_html_fragment_depth(&mut self, html: &str, depth: usize) -> Vec<u32> {
        if depth > 100 { return vec![]; }
        let mut result = vec![];
        let html = html.trim();
        if html.is_empty() { return result; }

        let mut pos = 0;
        let bytes = html.as_bytes();
        let byte_len = html.len();
        while pos < byte_len {
            if bytes[pos] == b'<' {
                if pos + 1 < byte_len && bytes[pos + 1] == b'/' {
                    let end = html[pos..].find('>');
                    if let Some(end) = end {
                        pos += end + 1;
                        continue;
                    }
                    break;
                }
                if pos + 1 < byte_len && bytes[pos + 1] == b'!' {
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
                let tag_end = html[pos..].find(['>', ' ', '\t', '\n']);
                if let Some(tag_end) = tag_end {
                    let tag_name = html[pos+1..pos+tag_end].to_lowercase();
                    if tag_name == "script" || tag_name == "style" || tag_name == "iframe" || tag_name == "object" || tag_name == "embed" {
                        let closing = format!("</{}>", tag_name);
                        if let Some(closing_pos) = html[pos..].to_lowercase().find(&closing.to_lowercase()) {
                            pos += closing_pos + closing.len();
                        } else { pos = byte_len; }
                        continue;
                    }
                    let is_self_closing = ["br", "hr", "img", "input", "meta", "link"];
                    let self_closing = is_self_closing.contains(&tag_name.as_str());

                    let mut attr_end = tag_end;
                    loop {
                        if pos + attr_end >= byte_len || bytes[pos + attr_end] == b'>' {
                            break;
                        }
                        attr_end += 1;
                    }
                    if pos + attr_end >= byte_len { break; }

                    let attrs_part = &html[pos+tag_end..pos+attr_end];
                    let mut attrs = self.parse_attributes(attrs_part);
                    Self::sanitize_attrs(&mut attrs);

                    let el_id = self.nodes.len() as u32;
                    self.nodes.push(FlatNode::element(&tag_name));
                    self.nodes[el_id as usize].attrs = attrs;
                    result.push(el_id);

                    pos += attr_end + 1;

                    if !self_closing {
                        let closing = format!("</{}>", tag_name);
                        if let Some(closing_pos) = html[pos..].find(&closing) {
                            let inner = &html[pos..pos + closing_pos];
                            if tag_name == "style" || tag_name == "script" {
                                if !inner.is_empty() {
                                    let text_id = self.nodes.len() as u32;
                                    self.nodes.push(FlatNode::text(inner));
                                    self.nodes[el_id as usize].children.push(text_id);
                                    if let Some(child) = self.nodes.get_mut(text_id as usize) {
                                        child.parent = Some(el_id);
                                    }
                                }
                            } else {
                                let inner_children = self.parse_html_fragment_depth(inner, depth + 1);
                                for child_id in inner_children {
                                    if let Some(child) = self.nodes.get_mut(child_id as usize) {
                                        child.parent = Some(el_id);
                                    }
                                    self.nodes[el_id as usize].children.push(child_id);
                                }
                            }
                            pos += closing_pos + closing.len();
                        } else {
                            let inner = &html[pos..];
                            if tag_name == "style" || tag_name == "script" {
                                if !inner.is_empty() {
                                    let text_id = self.nodes.len() as u32;
                                    self.nodes.push(FlatNode::text(inner));
                                    self.nodes[el_id as usize].children.push(text_id);
                                    if let Some(child) = self.nodes.get_mut(text_id as usize) {
                                        child.parent = Some(el_id);
                                    }
                                }
                            } else {
                                let inner_children = self.parse_html_fragment_depth(inner, depth + 1);
                                for child_id in inner_children {
                                    if let Some(child) = self.nodes.get_mut(child_id as usize) {
                                        child.parent = Some(el_id);
                                    }
                                    self.nodes[el_id as usize].children.push(child_id);
                                }
                            }
                            pos = byte_len;
                        }
                    }
                } else {
                    pos += 1;
                }
            } else {
                let text_end = html[pos..].find('<').unwrap_or(byte_len - pos);
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

    // ── DOM traversal ───────────────────────────────────────────────

    pub fn get_parent(&self, node_id: u32) -> Option<u32> {
        self.nodes.get(node_id as usize).and_then(|n| n.parent)
    }

    pub fn get_children(&self, node_id: u32) -> Vec<u32> {
        self.nodes.get(node_id as usize).map(|n|
            n.children.iter().filter(|&&id| self.nodes.get(id as usize).is_some_and(|c| !c.is_text)).copied().collect()
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

    // ── Get elements at a point (for click dispatch) ────────────────

    pub fn element_at_point(&self, x: f32, y: f32, elements: &[crate::engine::pipeline::StyledElement]) -> Option<u32> {
        let mut best_id = None;
        let mut best_area = f32::MAX;
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

    pub fn find_node_by_path(&self, path: &[usize]) -> Option<u32> {
        let mut current = 0u32;
        for &child_index in path {
            let node = self.nodes.get(current as usize)?;
            current = *node.children.get(child_index)?;
        }
        Some(current)
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

    // ── Closure registry (preserves captured variables across timer/event ticks) ──
    var __closureRegistry = {};
    var __nextClosureId = 0;
    var __handlerRegistry = {};
    var __nextHandlerId = 0;

    window.__executeClosure = function(id) {
        var fn = __closureRegistry[id];
        if (fn) {
            try { fn(); } catch(e) { __reportError(String(e), e.lineNumber || 0); }
        }
    };
    window.__executeHandler = function(id, nodeId) {
        var fn = __handlerRegistry[id];
        if (fn) {
            try { fn({ target: { __id: nodeId } }); } catch(e) { __reportError(String(e), e.lineNumber || 0); }
        }
    };

    // ── window.onerror ────────────────────────────────────────────────
    window.onerror = null;
    window.__triggerOnError = function(msg, url, line) {
        if (typeof window.onerror === 'function') {
            try { window.onerror(msg, url, line); } catch(e) {}
        }
    };

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
            json: function() { return Promise.resolve(JSON.parse(__fetch(url))); },
            text: function() { return Promise.resolve(__fetch(url)); }
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
                return __dom_getInnerHTML(id);
            },
            set innerHTML(val) {
                __dom_setInnerHTML(id, String(val));
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
                return pid !== -1 ? makeElement(pid) : null;
            },
            click: function() {
                __dom_dispatch_click(id);
            },

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
                if (typeof handler === 'function') {
                    var hid = __nextHandlerId++;
                    __handlerRegistry[hid] = handler;
                    __addEventListener(id, String(type), '__executeHandler(' + hid + ',' + id + ')');
                } else {
                    __addEventListener(id, String(type), String(handler));
                }
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
        if (typeof fn === 'function') {
            var cid = __nextClosureId++;
            __closureRegistry[cid] = fn;
            return __setTimeout('__executeClosure(' + cid + ')', ms || 0);
        }
        return __setTimeout(String(fn), ms || 0);
    };

    window.setInterval = function(fn, ms) {
        if (typeof fn === 'function') {
            var cid = __nextClosureId++;
            __closureRegistry[cid] = fn;
            return __setInterval('__executeClosure(' + cid + ')', ms || 0);
        }
        return __setInterval(String(fn), ms || 0);
    };

    window.clearTimeout = function(id) {
        __clearTimer(id);
    };

    window.clearInterval = function(id) {
        __clearTimer(id);
    };

    // ── window.fetch ────────────────────────────────────────────────

    window.fetch = function(url) {
        var raw = __fetch(String(url));
        var m = raw.match(/^__STATUS_(\d+)__/);
        var status = m ? parseInt(m[1], 10) : 0;
        var body = m ? raw.slice(m[0].length) : raw;
        return {
            ok: status >= 200 && status < 300,
            status: status,
            statusText: status >= 200 && status < 300 ? "OK" : body,
            url: url,
            text: function() { return Promise.resolve(body); },
            json: function() { return Promise.resolve(JSON.parse(body)); }
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

    (function() {
        var timers = {};
        console.time = function(label) {
            label = label || "default";
            timers[label] = Date.now();
        };
        console.timeEnd = function(label) {
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

    // ponytail: sync XHR — blocks JS thread, 3s timeout via __fetchXhr
    window.XMLHttpRequest.prototype.send = function() {
        this.readyState = 2;
        var raw = __fetchXhr(this._url);
        var m = raw.match(/^__STATUS_(\d+)__/);
        this.status = m ? parseInt(m[1], 10) : 0;
        this.responseText = m ? raw.slice(m[0].length) : raw;
        this.statusText = this.status >= 200 && this.status < 300 ? "OK" : this.responseText;
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
        cookieEnabled: true
    };

    // ── document.title ─────────────────────────────────────────────
    Object.defineProperty(document, 'title', {
        get: function() { return _getTitle(); },
        set: function(v) { _setTitle(String(v)); },
        enumerable: true,
        configurable: true
    });

    // ── window.history ─────────────────────────────────────────────
    window.history = {
        length: 0,
        state: null,
        pushState: function(state, title, url) {
            _pushState(state ? JSON.stringify(state) : null, title || "", url || "");
        },
        replaceState: function(state, title, url) {
            _replaceState(state ? JSON.stringify(state) : null, title || "", url || "");
        },
        back: function() { _historyBack(); },
        forward: function() { _historyForward(); },
        go: function(delta) { _historyGo(delta || 0); }
    };

    // ── document.cookie ─────────────────────────────────────────────
    Object.defineProperty(document, 'cookie', {
        get: function() { return _getCookie(); },
        set: function(v) { _setCookie(String(v)); },
        enumerable: true,
        configurable: true
    });

    // ── window.localStorage ─────────────────────────────────────────
    (function() {
        var storage = {};
        Object.defineProperty(storage, 'length', {
            get: function() { return _localStorageLength(); },
            enumerable: true,
            configurable: true
        });
        storage.getItem = function(k) { return _localStorageGetItem(String(k)); };
        storage.setItem = function(k, v) { _localStorageSetItem(String(k), String(v)); };
        storage.removeItem = function(k) { _localStorageRemoveItem(String(k)); };
        storage.clear = function() { _localStorageClear(); };
        storage.key = function(i) { return _localStorageKey(Number(i)); };
        window.localStorage = storage;
    })();

    // ── Korlang ────────────────────────────────────────────────────
    window.evalKorlang = function(code) {
        return __evalKorlang(String(code));
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
        let msg = args.into_inner().join(" ");
        plog!("JS", "{}", msg);
    }))?;
    console.set("warn", Func::new(|args: Rest<String>| {
        let msg = args.into_inner().join(" ");
        plog!("JS", "WARN: {}", msg);
    }))?;
    console.set("error", Func::new(|args: Rest<String>| {
        let msg = args.into_inner().join(" ");
        plog!("JS", "ERROR: {}", msg);
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
    let fn_get_html = Function::new(ctx.clone(), move |node_id: i32| -> String {
        if let Ok(b) = b1.lock() {
            b.get_inner_html(node_id as u32)
        } else { String::new() }
    })?;
    fn_get_html.set_name("__dom_getInnerHTML")?;
    globals.set("__dom_getInnerHTML", fn_get_html)?;

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

    // ── XHR fetch (shorter timeout) ─────────────────────────────────
    let bx = Arc::clone(bridge);
    let fn_fetch_xhr = Function::new(ctx.clone(), move |url: String| -> String {
        if let Ok(b) = bx.lock() {
            b.fetch_url_xhr(&url)
        } else { String::new() }
    })?;
    fn_fetch_xhr.set_name("__fetchXhr")?;
    globals.set("__fetchXhr", fn_fetch_xhr)?;

    // ── Cookie functions ───────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_get_cookie = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() {
            b.get_cookie()
        } else {
            String::new()
        }
    })?;
    fn_get_cookie.set_name("_getCookie")?;
    globals.set("_getCookie", fn_get_cookie)?;

    let b1 = Arc::clone(bridge);
    let fn_set_cookie = Function::new(ctx.clone(), move |v: String| {
        if let Ok(mut b) = b1.lock() {
            b.set_cookie(&v);
        }
    })?;
    fn_set_cookie.set_name("_setCookie")?;
    globals.set("_setCookie", fn_set_cookie)?;

    // ── LocalStorage functions ─────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_ls_get = Function::new(ctx.clone(), move |k: String| -> Option<String> {
        if let Ok(b) = b1.lock() {
            b.local_storage_get_item(&k)
        } else {
            None
        }
    })?;
    fn_ls_get.set_name("_localStorageGetItem")?;
    globals.set("_localStorageGetItem", fn_ls_get)?;

    let b1 = Arc::clone(bridge);
    let fn_ls_set = Function::new(ctx.clone(), move |k: String, v: String| {
        if let Ok(mut b) = b1.lock() {
            b.local_storage_set_item(k, v);
        }
    })?;
    fn_ls_set.set_name("_localStorageSetItem")?;
    globals.set("_localStorageSetItem", fn_ls_set)?;

    let b1 = Arc::clone(bridge);
    let fn_ls_remove = Function::new(ctx.clone(), move |k: String| {
        if let Ok(mut b) = b1.lock() {
            b.local_storage_remove_item(&k);
        }
    })?;
    fn_ls_remove.set_name("_localStorageRemoveItem")?;
    globals.set("_localStorageRemoveItem", fn_ls_remove)?;

    let b1 = Arc::clone(bridge);
    let fn_ls_clear = Function::new(ctx.clone(), move || {
        if let Ok(mut b) = b1.lock() {
            b.local_storage_clear();
        }
    })?;
    fn_ls_clear.set_name("_localStorageClear")?;
    globals.set("_localStorageClear", fn_ls_clear)?;

    let b1 = Arc::clone(bridge);
    let fn_ls_key = Function::new(ctx.clone(), move |i: i32| -> Option<String> {
        if let Ok(b) = b1.lock() {
            b.local_storage_key(i)
        } else {
            None
        }
    })?;
    fn_ls_key.set_name("_localStorageKey")?;
    globals.set("_localStorageKey", fn_ls_key)?;

    let b1 = Arc::clone(bridge);
    let fn_ls_len = Function::new(ctx.clone(), move || -> i32 {
        if let Ok(b) = b1.lock() {
            b.local_storage_length()
        } else {
            0
        }
    })?;
    fn_ls_len.set_name("_localStorageLength")?;
    globals.set("_localStorageLength", fn_ls_len)?;

    // ── document.title ──────────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_get_title = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.doc_title.clone() } else { String::new() }
    })?;
    fn_get_title.set_name("_getTitle")?;
    globals.set("_getTitle", fn_get_title)?;

    let b1 = Arc::clone(bridge);
    let fn_set_title = Function::new(ctx.clone(), move |title: String| {
        if let Ok(mut b) = b1.lock() { b.doc_title = title; }
    })?;
    fn_set_title.set_name("_setTitle")?;
    globals.set("_setTitle", fn_set_title)?;

    // ── window.history ─────────────────────────────────────────────
    let b1 = Arc::clone(bridge);
    let fn_get_history_state = Function::new(ctx.clone(), move || -> String {
        if let Ok(b) = b1.lock() { b.history_state.clone() } else { String::new() }
    })?;
    fn_get_history_state.set_name("_getHistoryState")?;
    globals.set("_getHistoryState", fn_get_history_state)?;

    let b1 = Arc::clone(bridge);
    let fn_push_state = Function::new(ctx.clone(), move |state: String, _title: String, url: String| {
        if let Ok(mut b) = b1.lock() {
            b.history_state = state;
            if !url.is_empty() { b.pending_navigation = Some(url); }
        }
    })?;
    fn_push_state.set_name("_pushState")?;
    globals.set("_pushState", fn_push_state)?;

    let b1 = Arc::clone(bridge);
    let fn_replace_state = Function::new(ctx.clone(), move |state: String, _title: String, url: String| {
        if let Ok(mut b) = b1.lock() {
            b.history_state = state;
            if !url.is_empty() { b.current_url = url; }
        }
    })?;
    fn_replace_state.set_name("_replaceState")?;
    globals.set("_replaceState", fn_replace_state)?;

    let b1 = Arc::clone(bridge);
    let fn_history_back = Function::new(ctx.clone(), move || {
        if let Ok(mut b) = b1.lock() { b.pending_history_delta = Some(-1); }
    })?;
    fn_history_back.set_name("_historyBack")?;
    globals.set("_historyBack", fn_history_back)?;

    let b1 = Arc::clone(bridge);
    let fn_history_forward = Function::new(ctx.clone(), move || {
        if let Ok(mut b) = b1.lock() { b.pending_history_delta = Some(1); }
    })?;
    fn_history_forward.set_name("_historyForward")?;
    globals.set("_historyForward", fn_history_forward)?;

    let b1 = Arc::clone(bridge);
    let fn_history_go = Function::new(ctx.clone(), move |delta: i32| {
        if let Ok(mut b) = b1.lock() { b.pending_history_delta = Some(delta); }
    })?;
    fn_history_go.set_name("_historyGo")?;
    globals.set("_historyGo", fn_history_go)?;

    // ── Error reporting ────────────────────────────────────────────
    {
        let bridge_clone = Arc::clone(bridge);
        let fn_report = Function::new(ctx.clone(), move |msg: String, line: i32| {
            if let Ok(mut b) = bridge_clone.lock() {
                let entry = if line > 0 { format!("Error: {} (line {})", msg, line) } else { format!("Error: {}", msg) };
                b.report_js_error(entry);
            }
        })?;
        fn_report.set_name("__reportError")?;
        globals.set("__reportError", fn_report)?;
    }

    // ── Inject JS shim ──────────────────────────────────────────────
    if let Err(e) = ctx.eval::<(), _>(SHIM_JS) {
        plog!("JS", "SHIM_JS eval failed: {:?}", e);
    }


    // ── Korlang integration ──────────────────────────────────────────
    let fn_eval_korlang = Function::new(ctx.clone(), move |code: String| -> String {
        crate::engine::korlang::eval_korlang(&code).unwrap_or_else(|e| e)
    })?;
    fn_eval_korlang.set_name("__evalKorlang")?;
    globals.set("__evalKorlang", fn_eval_korlang)?;

    // ponytail: __vault_savePassword removed — would need encrypted storage, add when login forms are supported
Ok(())
}
