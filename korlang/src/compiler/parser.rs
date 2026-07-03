use super::lexer::Token;
#[derive(Debug, Clone)]
pub enum Expr { Literal(String), Number(f64), Identifier(String), Binding(String), Interpolated { parts: Vec<String>, vars: Vec<String> } }
#[derive(Debug, Clone)]
pub struct Property { pub name: String, pub value: Expr }
#[derive(Debug, Clone)]
pub enum Node { Element(Element), IfElse { condition: Expr, then_branch: Vec<Node>, else_branch: Vec<Node> }, ForLoop { var: String, count: Expr, body: Vec<Node> } }
#[derive(Debug, Clone)]
pub struct Element { pub name: String, pub properties: Vec<Property>, pub children: Vec<Node>, pub on_click: Option<String> }
#[derive(Debug, Clone)]
pub struct State { pub name: String, pub type_name: String, pub default_value: Expr }
#[derive(Debug, Clone)]
pub struct Component { pub name: String, pub states: Vec<State>, pub root: Node }
pub struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    pub fn parse_component(&mut self) -> Option<Component> {
        if self.consume(Token::Component) {
            let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
            self.expect(Token::OpenBrace);
            let mut states = Vec::new();
            while self.peek() == Some(&Token::State) { if let Some(s) = self.parse_state() { states.push(s); } else { break; } }
            let root = self.parse_node()?;
            self.expect(Token::CloseBrace);
            Some(Component { name, states, root })
        } else { None }
    }
    fn parse_state(&mut self) -> Option<State> {
        self.advance_opt();
        let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        self.expect(Token::Colon);
        let type_name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        self.expect(Token::Equals);
        let default_value = self.parse_expr()?;
        Some(State { name, type_name, default_value })
    }
    fn parse_node(&mut self) -> Option<Node> {
        match self.peek() {
            Some(Token::If) => self.parse_if(),
            Some(Token::For) => self.parse_for(),
            _ => self.parse_element().map(Node::Element),
        }
    }
    fn parse_if(&mut self) -> Option<Node> {
        self.advance_opt();
        let condition = self.parse_expr()?;
        self.expect(Token::OpenBrace);
        let mut then_branch = Vec::new();
        while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
            if let Some(n) = self.parse_node() { then_branch.push(n); } else { break; }
        }
        self.expect(Token::CloseBrace);
        let mut else_branch = Vec::new();
        if self.consume(Token::Else) {
            self.expect(Token::OpenBrace);
            while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
                if let Some(n) = self.parse_node() { else_branch.push(n); } else { break; }
            }
            self.expect(Token::CloseBrace);
        }
        Some(Node::IfElse { condition, then_branch, else_branch })
    }
    fn parse_for(&mut self) -> Option<Node> {
        self.advance_opt();
        let var = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        let count = self.parse_expr()?;
        self.expect(Token::OpenBrace);
        let mut body = Vec::new();
        while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
            if let Some(n) = self.parse_node() { body.push(n); } else { break; }
        }
        self.expect(Token::CloseBrace);
        Some(Node::ForLoop { var, count, body })
    }
    fn parse_element(&mut self) -> Option<Element> {
        let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        let mut properties = Vec::new();
        let mut on_click = None;
        if self.consume(Token::OpenParen) {
            while self.peek() != Some(&Token::CloseParen) && !self.is_eof() {
                let prop_name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { break; };
                self.expect(Token::Colon);
                if let Some(val) = self.parse_expr() { properties.push(Property { name: prop_name, value: val }); }
                if self.peek() == Some(&Token::Comma) { self.advance_opt(); }
            }
            self.expect(Token::CloseParen);
        }
        let mut children = Vec::new();
        if self.consume(Token::OpenBrace) {
            while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
                if let Some(Token::Identifier(id)) = self.peek() {
                    if id == "on_click" {
                        self.advance_opt(); self.expect(Token::Colon);
                        if let Some(Token::StringLiteral(h)) = self.advance_opt() { on_click = Some(h); }
                        continue;
                    }
                }
                if let Some(child) = self.parse_node() { children.push(child); } else { break; }
            }
            self.expect(Token::CloseBrace);
        }
        Some(Element { name, properties, children, on_click })
    }
    fn parse_expr(&mut self) -> Option<Expr> {
        match self.advance_opt() {
            Some(Token::StringLiteral(s)) => Some(Expr::Literal(s)),
            Some(Token::InterpolatedString { parts, vars }) => Some(Expr::Interpolated { parts, vars }),
            Some(Token::Number(n)) => Some(Expr::Number(n)),
            Some(Token::Identifier(id)) => Some(Expr::Identifier(id)),
            Some(Token::Bind) => { let id = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; }; Some(Expr::Binding(id)) }
            _ => None,
        }
    }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn is_eof(&self) -> bool { self.pos >= self.tokens.len() }
    fn advance_opt(&mut self) -> Option<Token> { if self.is_eof() { None } else { let t = self.tokens[self.pos].clone(); self.pos += 1; Some(t) } }
    fn consume(&mut self, token: Token) -> bool { if self.peek() == Some(&token) { self.pos += 1; true } else { false } }
    fn expect(&mut self, _token: Token) { self.consume(_token); }
}
