#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Component, State, If, Else, For,
    Identifier(String), StringLiteral(String), InterpolatedString { parts: Vec<String>, vars: Vec<String> },
    Number(f64),
    Equals, Colon, OpenBrace, CloseBrace, OpenParen, CloseParen, Comma, Dot, Bind,
}
pub struct Lexer { input: Vec<char>, pos: usize }
impl Lexer {
    pub fn new(input: &str) -> Self { Self { input: input.chars().collect(), pos: 0 } }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            self.skip_whitespace();
            if self.is_eof() { break; }
            let c = self.peek();
            match c {
                '{' => { self.advance(); tokens.push(Token::OpenBrace); }
                '}' => { self.advance(); tokens.push(Token::CloseBrace); }
                '(' => { self.advance(); tokens.push(Token::OpenParen); }
                ')' => { self.advance(); tokens.push(Token::CloseParen); }
                ':' => { self.advance(); tokens.push(Token::Colon); }
                '=' => { self.advance(); tokens.push(Token::Equals); }
                ',' => { self.advance(); tokens.push(Token::Comma); }
                '.' => { self.advance(); tokens.push(Token::Dot); }
                '\"' => { tokens.push(self.read_string()); }
                _ if c.is_alphabetic() => { tokens.push(self.read_identifier()); }
                _ if c.is_numeric() => { tokens.push(self.read_number()); }
                _ => { eprintln!("lexer: skipping unknown character '{}'", self.advance()); }
            }
        }
        tokens
    }
    fn is_eof(&self) -> bool { self.pos >= self.input.len() }
    fn peek(&self) -> char { self.input[self.pos] }
    fn advance(&mut self) -> char { let c = self.input[self.pos]; self.pos += 1; c }
    fn skip_whitespace(&mut self) { while !self.is_eof() && self.peek().is_whitespace() { self.advance(); } }
    fn read_identifier(&mut self) -> Token {
        let mut id = String::new();
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_' || self.peek() == '⬡' || self.peek() == '←' || self.peek() == '⟳' || self.peek() == '⚙') { id.push(self.advance()); }
        match id.as_str() { "Component" => Token::Component, "state" => Token::State, "bind" => Token::Bind, "if" => Token::If, "else" => Token::Else, "for" => Token::For, _ => Token::Identifier(id) }
    }
    fn read_string(&mut self) -> Token {
        self.advance();
        let mut parts = Vec::new();
        let mut vars = Vec::new();
        let mut current = String::new();
        while !self.is_eof() && self.peek() != '\"' {
            if self.peek() == '$' && self.pos + 1 < self.input.len() && self.input[self.pos + 1].is_alphabetic() {
                parts.push(current.clone());
                current.clear();
                self.advance();
                let mut var = String::new();
                while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_') {
                    var.push(self.advance());
                }
                vars.push(var);
            } else {
                current.push(self.advance());
            }
        }
        parts.push(current);
        if !self.is_eof() { self.advance(); }
        if vars.is_empty() {
            Token::StringLiteral(parts.into_iter().next().unwrap_or_default())
        } else {
            Token::InterpolatedString { parts, vars }
        }
    }
    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while !self.is_eof() && (self.peek().is_numeric() || self.peek() == '.') { s.push(self.advance()); }
        Token::Number(s.parse().unwrap_or(0.0))
    }
}
