//! Stratus CSS Parser
//! Zero-copy string parser for CSS stylesheets

use super::style_value::{Color, LengthValue};

const MAX_INPUT_LENGTH: usize = 100_000;
const MAX_ITERATIONS: usize = 200_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
    Composite(Vec<SimpleSelector>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
    pub attribute: Option<(String, String)>,
    pub pseudo_class: Option<String>,
}

impl SimpleSelector {
    pub fn new() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
            attribute: None,
            pseudo_class: None,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Keyword(String),
    Length(LengthValue),
    Color(Color),
    Shorthand(Vec<PropertyValue>),
}

pub struct Parser {
    pos: usize,
    input: String,
    iterations: usize,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let input = if input.len() > MAX_INPUT_LENGTH {
            eprintln!("[Stratus] CSS input truncated from {} to {} chars", input.len(), MAX_INPUT_LENGTH);
            input[..MAX_INPUT_LENGTH].to_string()
        } else {
            input
        };
        Parser { pos: 0, input, iterations: 0 }
    }

    pub fn parse(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::new();
        let mut rules = Vec::new();

        loop {
            self.iterations += 1;
            if self.iterations > MAX_ITERATIONS {
                eprintln!("[Stratus] Parser hit iteration limit at {} rules", rules.len());
                break;
            }

            self.skip_whitespace_and_comments();
            if self.eof() { break; }

            let c = self.next_char();
            if c == '}' {
                self.consume_char();
                continue;
            }

            if c == '@' {
                self.consume_char();
                self.skip_at_rule(&mut rules);
                continue;
            }

            match self.parse_rule() {
                Some(rule) => rules.push(rule),
                None => {
                    self.consume_while(|c| c != '{' && c != '}');
                    if self.next_char() == '{' {
                        self.skip_to_matching_brace();
                    }
                }
            }
        }

        stylesheet.rules = rules;
        stylesheet
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        let mut selectors = Vec::new();
        let mut paren_depth = 0usize;

        loop {
            if self.eof() { return None; }
            let c = self.next_char();

            if c == '(' { paren_depth += 1; }
            if c == ')' { paren_depth = paren_depth.saturating_sub(1); }

            if paren_depth == 0 && c == '{' { break; }
            if c == ',' && paren_depth == 0 {
                self.consume_char();
                self.skip_whitespace();
                continue;
            }
            if c == ';' || c == '}' { return None; }

            self.skip_whitespace();
            let selector = self.parse_simple_selector()?;
            if !selector.tag_name.is_none() || selector.id.is_some() || !selector.class.is_empty() {
                selectors.push(Selector::Simple(selector));
            } else {
                self.consume_while(|c| c != ',' && c != '{' && c != '}' && c != ';');
            }
        }

        if selectors.is_empty() { return None; }

        self.consume_char();
        let declarations = self.parse_declarations();

        Some(Rule { selectors, declarations })
    }

    fn parse_simple_selector(&mut self) -> Option<SimpleSelector> {
        let mut selector = SimpleSelector::new();

        loop {
            if self.eof() { break; }
            let c = self.next_char();

            if c == ',' || c == '{' || c == '}' || c == ';' { break; }
            if c.is_whitespace() { break; }

            match c {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '[' => {
                    self.consume_char();
                    let attr_name = self.parse_identifier();
                    self.skip_whitespace();
                    let op = self.next_char();
                    let attr_value = if op == '=' {
                        self.consume_char();
                        Some(self.parse_identifier())
                    } else {
                        None
                    };
                    if self.next_char() == ']' { self.consume_char(); }
                    if let Some(value) = attr_value {
                        selector.attribute = Some((attr_name, value));
                    }
                }
                ':' => {
                    self.consume_char();
                    selector.pseudo_class = Some(self.parse_identifier());
                }
                _ => {
                    if c.is_alphanumeric() || c == '-' {
                        selector.tag_name = Some(self.parse_identifier());
                    } else {
                        break;
                    }
                }
            }
        }

        Some(selector)
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        loop {
            self.skip_whitespace_and_comments();
            if self.eof() { break; }

            let c = self.next_char();
            if c == '}' {
                self.consume_char();
                break;
            }
            if c == ';' {
                self.consume_char();
                continue;
            }

            let name = self.parse_identifier();
            if name.is_empty() {
                self.consume_while(|c| c != ';' && c != '}');
                if self.next_char() == ';' { self.consume_char(); }
                continue;
            }

            self.skip_whitespace();
            if self.next_char() == ':' { self.consume_char(); }
            self.skip_whitespace();

            let value_str = self.parse_value_string();
            let value = self.parse_property_value(&value_str);

            if self.next_char() == ';' { self.consume_char(); }

            if !name.is_empty() {
                declarations.push(Declaration { name, value });
            }
        }

        declarations
    }

    fn parse_value_string(&mut self) -> String {
        let mut value = String::new();
        let mut paren_depth = 0usize;

        loop {
            if self.eof() { break; }
            let c = self.next_char();

            if c == '(' { paren_depth += 1; }
            if c == ')' { paren_depth = paren_depth.saturating_sub(1); }

            if paren_depth == 0 && (c == ';' || c == '}') { break; }
            value.push(c);
            self.consume_char();
        }

        value.trim().to_string()
    }

    fn parse_property_value(&self, s: &str) -> PropertyValue {
        let s = s.trim();

        if s.starts_with('#') {
            if let Some(color) = Color::from_hex(s) {
                return PropertyValue::Color(color);
            }
        }

        if let Some(lv) = LengthValue::from_str(s) {
            return PropertyValue::Length(lv);
        }

        if let Some(color) = Color::from_named(s) {
            return PropertyValue::Color(color);
        }

        PropertyValue::Keyword(s.to_string())
    }

    fn parse_identifier(&mut self) -> String {
        let mut result = String::new();
        while !self.eof() {
            let c = self.next_char();
            if c.is_alphanumeric() || c == '-' || c == '_' {
                result.push(c);
                self.consume_char();
            } else { break; }
        }
        result
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.consume_while(char::is_whitespace);
            if self.input[self.pos..].starts_with("/*") {
                self.skip_comment();
            } else { break; }
        }
    }

    fn skip_comment(&mut self) {
        self.consume_char();
        self.consume_char();
        while !self.eof() {
            if self.input[self.pos..].starts_with("*/") {
                self.consume_char();
                self.consume_char();
                break;
            }
            self.consume_char();
        }
    }

    fn skip_at_rule(&mut self, rules: &mut Vec<Rule>) {
        let name = self.consume_while(|c| c.is_alphabetic() || c == '-');

        if name == "media" {
            // Skip past the media query to the opening brace
            loop {
                self.skip_whitespace_and_comments();
                if self.eof() { break; }
                if self.next_char() == '{' { break; }
                self.consume_char();
            }
            if !self.eof() {
                self.consume_char(); // consume '{'
                // Parse rules inside @media block until matching '}'
                let mut brace_depth = 1;
                while !self.eof() && brace_depth > 0 {
                    self.skip_whitespace_and_comments();
                    if self.eof() { break; }

                    self.iterations += 1;
                    if self.iterations > MAX_ITERATIONS { break; }

                    let c = self.next_char();
                    if c == '}' {
                        self.consume_char();
                        brace_depth -= 1;
                        continue;
                    }
                    if c == '@' {
                        self.consume_char();
                        self.skip_at_rule(rules);
                        continue;
                    }

                    match self.parse_rule() {
                        Some(rule) => rules.push(rule),
                        None => {
                            self.consume_while(|c| c != '{' && c != '}');
                            if self.next_char() == '{' {
                                self.skip_to_matching_brace();
                            }
                        }
                    }
                }
            }
        } else {
            while !self.eof() {
                let c = self.next_char();
                if c == '{' {
                    self.skip_to_matching_brace();
                    break;
                }
                if c == ';' {
                    self.consume_char();
                    break;
                }
                self.consume_char();
            }
        }
    }

    fn skip_to_matching_brace(&mut self) {
        let mut depth = 1usize;
        while !self.eof() && depth > 0 {
            let c = self.next_char();
            if c == '{' { depth += 1; }
            else if c == '}' { depth -= 1; }
            self.consume_char();
        }
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let start = self.pos;
        while !self.eof() && test(self.next_char()) {
            self.consume_char();
        }
        self.input[start..self.pos].to_string()
    }
}

pub fn parse(css: &str) -> Stylesheet {
    Parser::new(css.to_string()).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_rule() {
        let css = "div { color: red; }";
        let stylesheet = parse(css);
        assert_eq!(stylesheet.rules.len(), 1);
        assert_eq!(stylesheet.rules[0].selectors.len(), 1);
    }

    #[test]
    fn test_parse_multiple_rules() {
        let css = "body { margin: 0; } h1 { font-size: 1.5em; }";
        let stylesheet = parse(css);
        assert_eq!(stylesheet.rules.len(), 2);
    }

    #[test]
    fn test_parse_class_selector() {
        let css = ".card { background: #eee; }";
        let stylesheet = parse(css);
        assert!(matches!(
            &stylesheet.rules[0].selectors[0],
            Selector::Simple(s) if s.class.contains(&"card".to_string())
        ));
    }

    #[test]
    fn test_parse_id_selector() {
        let css = "#nav { display: flex; }";
        let stylesheet = parse(css);
        assert!(matches!(
            &stylesheet.rules[0].selectors[0],
            Selector::Simple(s) if s.id == Some("nav".to_string())
        ));
    }

    #[test]
    fn test_parse_multiple_with_id() {
        let css = "div { color: red; } #id { color: blue; }";
        let stylesheet = parse(css);
        assert_eq!(stylesheet.rules.len(), 2, "Should parse 2 rules");
    }

    #[test]
    fn test_parse_empty_input() {
        let stylesheet = parse("");
        assert!(stylesheet.rules.is_empty());
    }
}