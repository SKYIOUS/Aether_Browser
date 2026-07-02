//! ============================================================
//! Aether Browser — CSS3 Lexer & Parser (from scratch)
//! src/engine/css/parser.rs
//!
//! A full CSS3 tokenizer and parser producing StyleSheet ASTs.
//! Zero external browser dependencies.
//! ============================================================

use std::collections::HashMap;

// ── CSS Tokens ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    Ident(String),
    AtKeyword(String),
    Hash(String, HashFlag),
    String(String),
    Url(String),
    Delim(char),
    Number(f64, NumberType),
    Percentage(f64),
    Dimension(f64, String, NumberType),
    Whitespace,
    Cdo,         // <!--
    Cdc,         // -->
    Colon,
    Semicolon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function(String),
    Eof,
    BadString,
    BadUrl,
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HashFlag { Unrestricted, Id }

#[derive(Debug, Clone, PartialEq)]
pub enum NumberType { Integer, Number }

// ── CSS AST Types ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    Qualified(QualifiedRule),
    AtRule(AtRule),
}

#[derive(Debug, Clone)]
pub struct QualifiedRule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub struct AtRule {
    pub name: String,
    pub prelude: Vec<ComponentValue>,
    pub block: Option<Vec<Rule>>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: Vec<ComponentValue>,
    pub important: bool,
}

#[derive(Debug, Clone)]
pub enum ComponentValue {
    Token(CssToken),
    Block(Vec<ComponentValue>),
    Function(String, Vec<ComponentValue>),
}

// ── Selector Types ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    Type(String),                     // div, p, span
    Universal,                        // *
    Class(String),                    // .class
    Id(String),                       // #id
    Attribute(AttributeSelector),    // [attr=value]
    PseudoClass(String),              // :hover
    PseudoElement(String),            // ::before
    Combinator(Combinator),          // >, +, ~, ' '
}

#[derive(Debug, Clone, PartialEq)]
pub enum Combinator {
    Descendant,      // space
    Child,           // >
    AdjacentSibling, // +
    GeneralSibling,  // ~
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSelector {
    pub name: String,
    pub matcher: Option<AttributeMatcher>,
    pub value: Option<String>,
    pub case_insensitive: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeMatcher {
    Exact,        // =
    Includes,     // ~=
    DashMatch,    // |=
    Prefix,       // ^=
    Suffix,       // $=
    Substring,    // *=
}

// ── Specificity ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Specificity(pub u32, pub u32, pub u32);

impl Specificity {
    pub fn as_number(&self) -> u64 {
        ((self.0 as u64) << 16) | ((self.1 as u64) << 8) | (self.2 as u64)
    }
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let mut a = 0u32; // ID selectors
        let mut b = 0u32; // class, attribute, pseudo-class
        let mut c = 0u32; // type, pseudo-element

        for part in &self.parts {
            match part {
                SelectorPart::Id(_) => a += 1,
                SelectorPart::Class(_) | SelectorPart::Attribute(_) | SelectorPart::PseudoClass(_) => b += 1,
                SelectorPart::Type(_) | SelectorPart::PseudoElement(_) => c += 1,
                SelectorPart::Universal => {}
                SelectorPart::Combinator(_) => {}
            }
        }

        Specificity(a, b, c)
    }
}

// ── CSS Lexer ─────────────────────────────────────────────────

pub struct CssLexer {
    input: Vec<char>,
    pos: usize,
}

impl CssLexer {
    pub fn new(source: &str) -> Self {
        Self { input: source.chars().collect(), pos: 0 }
    }

    fn current(&self) -> Option<char> { self.input.get(self.pos).copied() }
    fn peek(&self, offset: usize) -> Option<char> { self.input.get(self.pos + offset).copied() }

    fn consume(&mut self) -> Option<char> {
        let c = self.input.get(self.pos).copied();
        self.pos += 1;
        c
    }

    fn reconsume(&mut self) { if self.pos > 0 { self.pos -= 1; } }

    fn next_non_whitespace(&mut self) {
        while matches!(self.current(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn consume_whitespace(&mut self) -> CssToken {
        while matches!(self.current(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
        CssToken::Whitespace
    }

    fn consume_comment(&mut self) {
        // already consumed '/*'
        loop {
            match self.consume() {
                Some('*') if self.current() == Some('/') => { self.pos += 1; break; }
                None => break,
                _ => {}
            }
        }
    }

    fn consume_string(&mut self, end_char: char) -> CssToken {
        let mut s = String::new();
        loop {
            match self.consume() {
                Some(c) if c == end_char => return CssToken::String(s),
                Some('\\') => {
                    match self.consume() {
                        Some('\n') => {} // escaped newline = nothing
                        Some(c) => s.push(c),
                        None => return CssToken::String(s),
                    }
                }
                Some('\n') => return CssToken::BadString,
                None => return CssToken::String(s),
                Some(c) => s.push(c),
            }
        }
    }

    fn consume_url(&mut self) -> CssToken {
        self.next_non_whitespace();
        let mut url = String::new();
        loop {
            match self.consume() {
                Some(')') => return CssToken::Url(url),
                Some('"') | Some('\'') | Some('(') => return CssToken::BadUrl,
                Some(c) if c.is_whitespace() => {
                    self.next_non_whitespace();
                    if self.current() == Some(')') {
                        self.pos += 1;
                        return CssToken::Url(url);
                    }
                    return CssToken::BadUrl;
                }
                None => return CssToken::Url(url),
                Some(c) => url.push(c),
            }
        }
    }

    fn consume_ident(&mut self, first: char) -> String {
        let mut s = first.to_string();
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '-' || c == '_' || c > '\x7F' {
                s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        s
    }

    fn consume_number(&mut self, first: char) -> (f64, NumberType) {
        let mut s = first.to_string();
        let mut is_float = false;

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                s.push(c);
                self.pos += 1;
            } else if c == '.' && !is_float {
                if matches!(self.peek(1), Some(d) if d.is_ascii_digit()) {
                    is_float = true;
                    s.push(c);
                    self.pos += 1;
                } else {
                    break;
                }
            } else if (c == 'e' || c == 'E') && !s.is_empty() {
                if matches!(self.peek(1), Some(d) if d.is_ascii_digit() || d == '+' || d == '-') {
                    is_float = true;
                    s.push(c);
                    self.pos += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let val = s.parse::<f64>().unwrap_or(0.0);
        let num_type = if is_float { NumberType::Number } else { NumberType::Integer };
        (val, num_type)
    }

    pub fn next_token(&mut self) -> CssToken {
        // Skip comments inline
        loop {
            if self.current() == Some('/') && self.peek(1) == Some('*') {
                self.pos += 2;
                self.consume_comment();
            } else {
                break;
            }
        }

        match self.consume() {
            None => CssToken::Eof,
            Some(c) if c.is_whitespace() => {
                self.reconsume();
                self.consume_whitespace()
            }
            Some('"') => self.consume_string('"'),
            Some('\'') => self.consume_string('\''),
            Some('#') => {
                let mut name = String::new();
                while let Some(c) = self.current() {
                    if c.is_alphanumeric() || c == '-' || c == '_' || c > '\x7F' {
                        name.push(c);
                        self.pos += 1;
                    } else { break; }
                }
                // Check if it's a valid id
                let is_id = name.chars().next().map(|c| c.is_alphabetic() || c == '_' || c > '\x7F').unwrap_or(false);
                let flag = if is_id { HashFlag::Id } else { HashFlag::Unrestricted };
                CssToken::Hash(name, flag)
            }
            Some('@') => {
                if matches!(self.current(), Some(c) if c.is_alphabetic() || c == '_' || c > '\x7F') {
                    let first = self.consume().unwrap();
                    let name = self.consume_ident(first);
                    CssToken::AtKeyword(name)
                } else {
                    CssToken::Delim('@')
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' || c > '\x7F' || (c == '-' && matches!(self.current(), Some(n) if n.is_alphabetic() || n == '_' || n == '-')) => {
                let ident = self.consume_ident(c);
                // Check if followed by (
                if self.current() == Some('(') {
                    self.pos += 1;
                    if ident.eq_ignore_ascii_case("url") {
                        return self.consume_url();
                    }
                    return CssToken::Function(ident);
                }
                CssToken::Ident(ident)
            }
            Some(c) if c.is_ascii_digit() || (c == '.' && matches!(self.current(), Some(d) if d.is_ascii_digit())) || (c == '+' && matches!(self.current(), Some(d) if d.is_ascii_digit() || d == '.')) || (c == '-' && matches!(self.current(), Some(d) if d.is_ascii_digit() || d == '.')) => {
                let (val, num_type) = self.consume_number(c);
                // Check for dimension or percentage
                match self.current() {
                    Some('%') => { self.pos += 1; CssToken::Percentage(val) }
                    Some(c) if c.is_alphabetic() || c == '_' || c > '\x7F' => {
                        let unit_first = self.consume().unwrap();
                        let unit = self.consume_ident(unit_first);
                        CssToken::Dimension(val, unit, num_type)
                    }
                    _ => CssToken::Number(val, num_type),
                }
            }
            Some('<') if self.peek(0) == Some('!') && self.peek(1) == Some('-') && self.peek(2) == Some('-') => {
                self.pos += 3;
                CssToken::Cdo
            }
            Some('-') if self.peek(0) == Some('-') && self.peek(1) == Some('>') => {
                self.pos += 2;
                CssToken::Cdc
            }
            Some(':') => CssToken::Colon,
            Some(';') => CssToken::Semicolon,
            Some(',') => CssToken::Comma,
            Some('[') => CssToken::LeftBracket,
            Some(']') => CssToken::RightBracket,
            Some('(') => CssToken::LeftParen,
            Some(')') => CssToken::RightParen,
            Some('{') => CssToken::LeftBrace,
            Some('}') => CssToken::RightBrace,
            Some(c) => CssToken::Delim(c),
        }
    }

    pub fn tokenize_all(&mut self) -> Vec<CssToken> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok == CssToken::Eof;
            tokens.push(tok);
            if is_eof { break; }
        }
        tokens
    }
}

// ── CSS Parser ────────────────────────────────────────────────

pub struct CssParser {
    tokens: Vec<CssToken>,
    pos: usize,
}

impl CssParser {
    pub fn new(source: &str) -> Self {
        let mut lexer = CssLexer::new(source);
        let tokens = lexer.tokenize_all();
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&CssToken> { self.tokens.get(self.pos) }
    fn consume(&mut self) -> Option<CssToken> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }
    fn reconsume(&mut self) { if self.pos > 0 { self.pos -= 1; } }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(CssToken::Whitespace)) {
            self.pos += 1;
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let rules = self.consume_rules_list(true);
        Stylesheet { rules }
    }

    fn consume_rules_list(&mut self, top_level: bool) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            match self.current() {
                None | Some(CssToken::Eof) => break,
                Some(CssToken::Whitespace) => { self.pos += 1; }
                Some(CssToken::Cdo) | Some(CssToken::Cdc) => {
                    if top_level { self.pos += 1; } else {
                        if let Some(rule) = self.consume_qualified_rule() {
                            rules.push(Rule::Qualified(rule));
                        }
                    }
                }
                Some(CssToken::AtKeyword(_)) => {
                    if let Some(at_rule) = self.consume_at_rule() {
                        rules.push(Rule::AtRule(at_rule));
                    }
                }
                _ => {
                    if let Some(rule) = self.consume_qualified_rule() {
                        rules.push(Rule::Qualified(rule));
                    }
                }
            }
        }
        rules
    }

    fn consume_at_rule(&mut self) -> Option<AtRule> {
        let name = if let Some(CssToken::AtKeyword(n)) = self.consume() { n } else { return None; };
        let mut prelude = Vec::new();
        let mut block = None;
        let mut declarations = Vec::new();

        loop {
            match self.current() {
                Some(CssToken::Semicolon) => { self.pos += 1; break; }
                None | Some(CssToken::Eof) => break,
                Some(CssToken::LeftBrace) => {
                    self.pos += 1;
                    // For @media, @supports, etc. — parse nested rules
                    match name.as_str() {
                        "media" | "supports" | "layer" => {
                            block = Some(self.consume_rules_list(false));
                        }
                        _ => {
                            // Parse declarations (e.g. @font-face, @keyframes)
                            declarations = self.consume_declarations();
                        }
                    }
                    if matches!(self.current(), Some(CssToken::RightBrace)) { self.pos += 1; }
                    break;
                }
                _ => {
                    if let Some(cv) = self.consume_component_value() {
                        prelude.push(cv);
                    }
                }
            }
        }

        Some(AtRule { name, prelude, block, declarations })
    }

    fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
        let mut selector_tokens = Vec::new();

        loop {
            match self.current() {
                None | Some(CssToken::Eof) => return None,
                Some(CssToken::LeftBrace) => {
                    self.pos += 1;
                    let declarations = self.consume_declarations();
                    if matches!(self.current(), Some(CssToken::RightBrace)) { self.pos += 1; }
                    
                    // Parse selector from collected tokens
                    let selector_str: String = tokens_to_string(&selector_tokens);
                    let selectors = parse_selector_list(&selector_str);
                    
                    return Some(QualifiedRule { selectors, declarations });
                }
                _ => {
                    let tok = self.consume()?;
                    selector_tokens.push(tok);
                }
            }
        }
    }

    fn consume_declarations(&mut self) -> Vec<Declaration> {
        let mut decls = Vec::new();
        loop {
            match self.current() {
                None | Some(CssToken::Eof) | Some(CssToken::RightBrace) => break,
                Some(CssToken::Whitespace) | Some(CssToken::Semicolon) => { self.pos += 1; }
                Some(CssToken::AtKeyword(_)) => {
                    // skip at-rule inside declarations
                    self.consume_at_rule();
                }
                Some(CssToken::Ident(_)) => {
                    // Collect declaration
                    let mut tokens = Vec::new();
                    while !matches!(self.current(), None | Some(CssToken::Semicolon) | Some(CssToken::RightBrace) | Some(CssToken::Eof)) {
                        let tok = self.consume().unwrap();
                        tokens.push(tok);
                    }
                    if let Some(CssToken::Semicolon) = self.current() { self.pos += 1; }
                    if let Some(decl) = parse_declaration(tokens) {
                        decls.push(decl);
                    }
                }
                _ => { self.pos += 1; }
            }
        }
        decls
    }

    fn consume_component_value(&mut self) -> Option<ComponentValue> {
        match self.current()? {
            CssToken::LeftBrace | CssToken::LeftBracket | CssToken::LeftParen => {
                let open = self.consume()?;
                let mut values = Vec::new();
                let close = match open {
                    CssToken::LeftBrace => CssToken::RightBrace,
                    CssToken::LeftBracket => CssToken::RightBracket,
                    _ => CssToken::RightParen,
                };
                loop {
                    if matches!(self.current(), None | Some(CssToken::Eof)) { break; }
                    if self.current() == Some(&close) { self.pos += 1; break; }
                    if let Some(cv) = self.consume_component_value() { values.push(cv); }
                }
                Some(ComponentValue::Block(values))
            }
            CssToken::Function(_) => {
                let name = if let Some(CssToken::Function(n)) = self.consume() { n } else { return None; };
                let mut values = Vec::new();
                loop {
                    if matches!(self.current(), None | Some(CssToken::Eof) | Some(CssToken::RightParen)) { break; }
                    if let Some(cv) = self.consume_component_value() { values.push(cv); }
                }
                if matches!(self.current(), Some(CssToken::RightParen)) { self.pos += 1; }
                Some(ComponentValue::Function(name, values))
            }
            _ => {
                let tok = self.consume()?;
                Some(ComponentValue::Token(tok))
            }
        }
    }
}

// ── Selector Parser ───────────────────────────────────────────

pub fn parse_selector_list(input: &str) -> Vec<Selector> {
    input.split(',').filter_map(|s| parse_selector(s.trim())).collect()
}

pub fn parse_selector(input: &str) -> Option<Selector> {
    let input = input.trim();
    if input.is_empty() { return None; }

    let mut parts = Vec::new();
    let mut chars = input.chars().peekable();
    let mut last_was_combinator = true;

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
                // Check if next non-whitespace is a combinator
                if !last_was_combinator {
                    // Could be descendant combinator
                    let next = chars.peek().copied();
                    match next {
                        Some('>') | Some('+') | Some('~') => {}
                        _ => {
                            parts.push(SelectorPart::Combinator(Combinator::Descendant));
                            last_was_combinator = true;
                        }
                    }
                }
            }
            '>' => {
                chars.next();
                parts.push(SelectorPart::Combinator(Combinator::Child));
                last_was_combinator = true;
            }
            '+' => {
                chars.next();
                parts.push(SelectorPart::Combinator(Combinator::AdjacentSibling));
                last_was_combinator = true;
            }
            '~' => {
                chars.next();
                parts.push(SelectorPart::Combinator(Combinator::GeneralSibling));
                last_was_combinator = true;
            }
            '*' => {
                chars.next();
                parts.push(SelectorPart::Universal);
                last_was_combinator = false;
            }
            '#' => {
                chars.next();
                let mut id = String::new();
                while chars.peek().map(|c| c.is_alphanumeric() || *c == '-' || *c == '_').unwrap_or(false) {
                    id.push(chars.next().unwrap());
                }
                parts.push(SelectorPart::Id(id));
                last_was_combinator = false;
            }
            '.' => {
                chars.next();
                let mut cls = String::new();
                while chars.peek().map(|c| c.is_alphanumeric() || *c == '-' || *c == '_').unwrap_or(false) {
                    cls.push(chars.next().unwrap());
                }
                parts.push(SelectorPart::Class(cls));
                last_was_combinator = false;
            }
            ':' => {
                chars.next();
                let is_pseudo_el = chars.peek() == Some(&':');
                if is_pseudo_el { chars.next(); }
                let mut name = String::new();
                while chars.peek().map(|c| c.is_alphanumeric() || *c == '-' || *c == '_').unwrap_or(false) {
                    name.push(chars.next().unwrap());
                }
                // Handle pseudo-class functions like :not(), :nth-child()
                if chars.peek() == Some(&'(') {
                    chars.next();
                    let mut inner = String::new();
                    let mut depth = 1;
                    while let Some(c) = chars.next() {
                        if c == '(' { depth += 1; inner.push(c); }
                        else if c == ')' { depth -= 1; if depth == 0 { break; } else { inner.push(c); } }
                        else { inner.push(c); }
                    }
                    name.push('(');
                    name.push_str(&inner);
                    name.push(')');
                }
                if is_pseudo_el {
                    parts.push(SelectorPart::PseudoElement(name));
                } else {
                    parts.push(SelectorPart::PseudoClass(name));
                }
                last_was_combinator = false;
            }
            '[' => {
                chars.next();
                let mut attr_name = String::new();
                while chars.peek().map(|c| *c != ']' && *c != '=' && *c != '~' && *c != '|' && *c != '^' && *c != '$' && *c != '*').unwrap_or(false) {
                    attr_name.push(chars.next().unwrap());
                }
                let attr_name = attr_name.trim().to_string();

                // Parse matcher
                let matcher = match chars.peek() {
                    Some(&'=') => { chars.next(); Some(AttributeMatcher::Exact) }
                    Some(&'~') => { chars.next(); chars.next(); Some(AttributeMatcher::Includes) }
                    Some(&'|') => { chars.next(); chars.next(); Some(AttributeMatcher::DashMatch) }
                    Some(&'^') => { chars.next(); chars.next(); Some(AttributeMatcher::Prefix) }
                    Some(&'$') => { chars.next(); chars.next(); Some(AttributeMatcher::Suffix) }
                    Some(&'*') => { chars.next(); chars.next(); Some(AttributeMatcher::Substring) }
                    _ => None,
                };

                let mut value = None;
                if matcher.is_some() {
                    // Parse value
                    while chars.peek() == Some(&' ') { chars.next(); }
                    let quote = if chars.peek() == Some(&'"') || chars.peek() == Some(&'\'') { chars.next() } else { None };
                    let mut val = String::new();
                    loop {
                        match chars.peek() {
                            Some(&']') | None => break,
                            Some(&q) if Some(q) == quote => { chars.next(); break; }
                            Some(&' ') if quote.is_none() => break,
                            _ => val.push(chars.next().unwrap()),
                        }
                    }
                    value = Some(val);
                }

                while chars.peek() != Some(&']') && chars.peek().is_some() { chars.next(); }
                chars.next(); // consume ']'

                parts.push(SelectorPart::Attribute(AttributeSelector {
                    name: attr_name,
                    matcher,
                    value,
                    case_insensitive: false,
                }));
                last_was_combinator = false;
            }
            c if c.is_alphabetic() || c == '_' || c == '-' => {
                let mut tag = String::new();
                while chars.peek().map(|c| c.is_alphanumeric() || *c == '-' || *c == '_').unwrap_or(false) {
                    tag.push(chars.next().unwrap());
                }
                parts.push(SelectorPart::Type(tag));
                last_was_combinator = false;
            }
            _ => { chars.next(); }
        }
    }

    if parts.is_empty() { None } else { Some(Selector { parts }) }
}

// ── Declaration Parser ────────────────────────────────────────

fn parse_declaration(tokens: Vec<CssToken>) -> Option<Declaration> {
    let mut iter = tokens.into_iter().peekable();

    // Skip whitespace
    while matches!(iter.peek(), Some(CssToken::Whitespace)) { iter.next(); }

    let property = if let Some(CssToken::Ident(name)) = iter.next() { name.to_ascii_lowercase() } else { return None; };

    while matches!(iter.peek(), Some(CssToken::Whitespace)) { iter.next(); }

    // Expect colon
    match iter.peek() {
        Some(CssToken::Colon) => { iter.next(); }
        _ => return None,
    }

    while matches!(iter.peek(), Some(CssToken::Whitespace)) { iter.next(); }

    let mut value: Vec<ComponentValue> = Vec::new();
    let mut important = false;
    let mut remaining: Vec<CssToken> = iter.collect();

    // Check !important at end
    let mut end_idx = remaining.len();
    for i in (0..remaining.len()).rev() {
        match &remaining[i] {
            CssToken::Whitespace => continue,
            CssToken::Ident(s) if s.eq_ignore_ascii_case("important") => {
                if i > 0 {
                    if let CssToken::Delim('!') = remaining[i - 1] {
                        important = true;
                        end_idx = i - 1;
                        break;
                    }
                }
            }
            _ => break,
        }
    }

    for tok in remaining.into_iter().take(end_idx) {
        value.push(ComponentValue::Token(tok));
    }

    Some(Declaration { property, value, important })
}

// ── Token Helper ──────────────────────────────────────────────

fn tokens_to_string(tokens: &[CssToken]) -> String {
    tokens.iter().map(|t| match t {
        CssToken::Ident(s) | CssToken::AtKeyword(s) | CssToken::Hash(s, _) => s.clone(),
        CssToken::Whitespace => " ".to_string(),
        CssToken::Colon => ":".to_string(),
        CssToken::Semicolon => ";".to_string(),
        CssToken::Comma => ",".to_string(),
        CssToken::LeftBracket => "[".to_string(),
        CssToken::RightBracket => "]".to_string(),
        CssToken::LeftParen => "(".to_string(),
        CssToken::RightParen => ")".to_string(),
        CssToken::LeftBrace => "{".to_string(),
        CssToken::RightBrace => "}".to_string(),
        CssToken::Delim(c) => c.to_string(),
        CssToken::Number(n, _) => n.to_string(),
        CssToken::Percentage(n) => format!("{}%", n),
        CssToken::Dimension(n, u, _) => format!("{}{}", n, u),
        CssToken::String(s) => format!("\"{}\"", s),
        CssToken::Url(s) => format!("url({})", s),
        CssToken::Function(n) => format!("{}(", n),
        _ => String::new(),
    }).collect()
}

// ── Computed Style Values ─────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub color: Option<CssColor>,
    pub background_color: Option<CssColor>,
    pub font_size: Option<f32>,
    pub font_weight: Option<String>,
    pub font_family: Option<String>,
    pub font_style: Option<String>,
    pub text_align: Option<String>,
    pub text_decoration: Option<String>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,

    pub margin_top: Option<f32>,
    pub margin_right: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,

    pub padding_top: Option<f32>,
    pub padding_right: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,

    pub border_top_width: Option<f32>,
    pub border_right_width: Option<f32>,
    pub border_bottom_width: Option<f32>,
    pub border_left_width: Option<f32>,
    pub border_top_color: Option<CssColor>,
    pub border_right_color: Option<CssColor>,
    pub border_bottom_color: Option<CssColor>,
    pub border_left_color: Option<CssColor>,
    pub border_radius: Option<f32>,

    pub width: Option<CssLength>,
    pub height: Option<CssLength>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,

    pub display: Option<String>,
    pub position: Option<String>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub z_index: Option<i32>,

    pub flex_direction: Option<String>,
    pub flex_wrap: Option<String>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<CssLength>,
    pub justify_content: Option<String>,
    pub align_items: Option<String>,
    pub align_self: Option<String>,
    pub align_content: Option<String>,
    pub gap: Option<f32>,

    pub overflow: Option<String>,
    pub overflow_x: Option<String>,
    pub overflow_y: Option<String>,
    pub visibility: Option<String>,
    pub opacity: Option<f32>,
    pub cursor: Option<String>,
    pub pointer_events: Option<String>,

    pub box_shadow: Option<String>,
    pub text_shadow: Option<String>,
    pub transform: Option<String>,
    pub transition: Option<String>,
    pub animation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CssLength {
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    Auto,
    None,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CssColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl CssColor {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    pub fn transparent() -> Self { Self { r: 0, g: 0, b: 0, a: 0 } }
    pub fn black() -> Self { Self::rgba(0, 0, 0, 255) }
    pub fn white() -> Self { Self::rgba(255, 255, 255, 255) }
}

/// Parse a CSS length value like "16px", "1.5em", "50%", "auto"
pub fn parse_length(s: &str) -> Option<CssLength> {
    let s = s.trim();
    if s == "auto" { return Some(CssLength::Auto); }
    if s == "none" { return Some(CssLength::None); }

    if let Some(px) = s.strip_suffix("px") {
        return px.parse::<f32>().ok().map(CssLength::Px);
    }
    if let Some(em) = s.strip_suffix("em") {
        if !em.ends_with('r') {
            return em.parse::<f32>().ok().map(CssLength::Em);
        }
    }
    if let Some(rem) = s.strip_suffix("rem") {
        return rem.parse::<f32>().ok().map(CssLength::Rem);
    }
    if let Some(pct) = s.strip_suffix('%') {
        return pct.parse::<f32>().ok().map(CssLength::Percent);
    }
    if let Some(vw) = s.strip_suffix("vw") {
        return vw.parse::<f32>().ok().map(CssLength::Vw);
    }
    if let Some(vh) = s.strip_suffix("vh") {
        return vh.parse::<f32>().ok().map(CssLength::Vh);
    }

    None
}

/// Parse a CSS color: #hex, rgb(), rgba(), named colors
pub fn parse_color(s: &str) -> Option<CssColor> {
    let s = s.trim();
    if s.starts_with('#') {
        let hex = &s[1..];
        return match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(CssColor::rgba(r, g, b, 255))
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
                Some(CssColor::rgba(r, g, b, a))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(CssColor::rgba(r, g, b, 255))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(CssColor::rgba(r, g, b, a))
            }
            _ => None,
        };
    }

    if let Some(inner) = s.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 4 {
            let r = parts[0].trim().parse::<f32>().ok()? as u8;
            let g = parts[1].trim().parse::<f32>().ok()? as u8;
            let b = parts[2].trim().parse::<f32>().ok()? as u8;
            let a = (parts[3].trim().parse::<f32>().ok()? * 255.0) as u8;
            return Some(CssColor::rgba(r, g, b, a));
        }
    }

    if let Some(inner) = s.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 3 {
            let r = parts[0].trim().parse::<f32>().ok()? as u8;
            let g = parts[1].trim().parse::<f32>().ok()? as u8;
            let b = parts[2].trim().parse::<f32>().ok()? as u8;
            return Some(CssColor::rgba(r, g, b, 255));
        }
    }

    // Named colors
    named_color(s)
}

fn named_color(name: &str) -> Option<CssColor> {
    Some(match name.to_ascii_lowercase().as_str() {
        "transparent" => CssColor::transparent(),
        "black" => CssColor::rgba(0, 0, 0, 255),
        "white" => CssColor::rgba(255, 255, 255, 255),
        "red" => CssColor::rgba(255, 0, 0, 255),
        "green" => CssColor::rgba(0, 128, 0, 255),
        "lime" => CssColor::rgba(0, 255, 0, 255),
        "blue" => CssColor::rgba(0, 0, 255, 255),
        "yellow" => CssColor::rgba(255, 255, 0, 255),
        "cyan" | "aqua" => CssColor::rgba(0, 255, 255, 255),
        "magenta" | "fuchsia" => CssColor::rgba(255, 0, 255, 255),
        "gray" | "grey" => CssColor::rgba(128, 128, 128, 255),
        "silver" => CssColor::rgba(192, 192, 192, 255),
        "darkgray" | "darkgrey" => CssColor::rgba(169, 169, 169, 255),
        "lightgray" | "lightgrey" => CssColor::rgba(211, 211, 211, 255),
        "orange" => CssColor::rgba(255, 165, 0, 255),
        "purple" => CssColor::rgba(128, 0, 128, 255),
        "brown" => CssColor::rgba(165, 42, 42, 255),
        "pink" => CssColor::rgba(255, 192, 203, 255),
        "navy" => CssColor::rgba(0, 0, 128, 255),
        "teal" => CssColor::rgba(0, 128, 128, 255),
        "maroon" => CssColor::rgba(128, 0, 0, 255),
        "olive" => CssColor::rgba(128, 128, 0, 255),
        "coral" => CssColor::rgba(255, 127, 80, 255),
        "salmon" => CssColor::rgba(250, 128, 114, 255),
        "gold" => CssColor::rgba(255, 215, 0, 255),
        "indigo" => CssColor::rgba(75, 0, 130, 255),
        "violet" => CssColor::rgba(238, 130, 238, 255),
        "turquoise" => CssColor::rgba(64, 224, 208, 255),
        "skyblue" => CssColor::rgba(135, 206, 235, 255),
        "crimson" => CssColor::rgba(220, 20, 60, 255),
        "khaki" => CssColor::rgba(240, 230, 140, 255),
        "lavender" => CssColor::rgba(230, 230, 250, 255),
        "beige" => CssColor::rgba(245, 245, 220, 255),
        "ivory" => CssColor::rgba(255, 255, 240, 255),
        "wheat" => CssColor::rgba(245, 222, 179, 255),
        "linen" => CssColor::rgba(250, 240, 230, 255),
        "chocolate" => CssColor::rgba(210, 105, 30, 255),
        "tomato" => CssColor::rgba(255, 99, 71, 255),
        "snow" => CssColor::rgba(255, 250, 250, 255),
        "azure" => CssColor::rgba(240, 255, 255, 255),
        "mintcream" => CssColor::rgba(245, 255, 250, 255),
        _ => return None,
    })
}

// ── Public API ────────────────────────────────────────────────

pub fn parse_css(source: &str) -> Stylesheet {
    CssParser::new(source).parse_stylesheet()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_rule() {
        let mut lexer = CssLexer::new("p { color: red; }");
        let tokens = lexer.tokenize_all();
        assert!(tokens.iter().any(|t| matches!(t, CssToken::Ident(s) if s == "p")));
        assert!(tokens.iter().any(|t| matches!(t, CssToken::LeftBrace)));
    }

    #[test]
    fn test_parse_simple_rule() {
        let ss = parse_css("p { color: red; font-size: 16px; }");
        assert_eq!(ss.rules.len(), 1);
        if let Rule::Qualified(qr) = &ss.rules[0] {
            assert_eq!(qr.declarations.len(), 2);
            assert_eq!(qr.declarations[0].property, "color");
        }
    }

    #[test]
    fn test_parse_selector() {
        let sel = parse_selector("div.container > p.text#main").unwrap();
        assert!(sel.parts.iter().any(|p| matches!(p, SelectorPart::Id(id) if id == "main")));
        assert!(sel.parts.iter().any(|p| matches!(p, SelectorPart::Class(c) if c == "container")));
    }

    #[test]
    fn test_parse_color_hex() {
        let c = parse_color("#FF8800").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 136);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_parse_color_named() {
        let c = parse_color("cornsilk");
        // May or may not be in list, just check it doesn't panic
        let _ = c;
        let c = parse_color("red").unwrap();
        assert_eq!(c.r, 255);
    }

    #[test]
    fn test_specificity() {
        let sel = parse_selector("#id .class div").unwrap();
        let spec = sel.specificity();
        assert_eq!(spec.0, 1); // 1 ID
        assert_eq!(spec.1, 1); // 1 class
        assert_eq!(spec.2, 1); // 1 type
    }

    #[test]
    fn test_at_media() {
        let ss = parse_css("@media (max-width: 600px) { p { color: blue; } }");
        assert!(ss.rules.iter().any(|r| matches!(r, Rule::AtRule(a) if a.name == "media")));
    }
}
