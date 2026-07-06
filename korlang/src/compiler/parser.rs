use super::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Number(f64),
    Identifier(String),
    Binding(String),
    Call { name: String, args: Vec<Expr> },
    List(Vec<Expr>),
    Interpolated { parts: Vec<String>, vars: Vec<String> },
    BinaryOp { left: Box<Expr>, op: Token, right: Box<Expr> },
    UnaryOp { op: Token, expr: Box<Expr> },
}

#[derive(Debug, Clone)]
pub struct Property { pub name: String, pub value: Expr }

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    IfElse { condition: Expr, then_branch: Vec<Node>, else_branch: Vec<Node> },
    ForLoop { var: String, collection: Expr, body: Vec<Node> },
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub properties: Vec<Property>,
    pub children: Vec<Node>,
    pub on_click: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    pub type_name: String,
    pub default_value: Expr,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub states: Vec<State>,
    pub functions: Vec<FunctionDef>,
    pub root: Node,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_component(&mut self) -> Option<Component> {
        if self.consume(&Token::Component) {
            let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
            self.expect(Token::OpenBrace);
            let mut states = Vec::new();
            let mut functions = Vec::new();
            while !self.is_eof() && self.peek() != Some(&Token::CloseBrace) {
                match self.peek() {
                    Some(Token::State) => { if let Some(s) = self.parse_state() { states.push(s); } }
                    Some(Token::Fn) => { if let Some(f) = self.parse_function_def() { functions.push(f); } }
                    _ => break,
                }
            }
            let root = self.parse_node()?;
            self.expect(Token::CloseBrace);
            Some(Component { name, states, functions, root })
        } else {
            None
        }
    }

    fn parse_state(&mut self) -> Option<State> {
        self.consume(&Token::State);
        let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        self.expect(Token::Colon);
        let type_name = match self.advance_opt() {
            Some(Token::Identifier(id)) => id,
            Some(Token::Int) => "Int".to_string(),
            Some(Token::Float) => "Float".to_string(),
            Some(Token::String) => "String".to_string(),
            Some(Token::Bool) => "Bool".to_string(),
            _ => "Any".to_string(),
        };
        self.expect(Token::Equals);
        let default_value = self.parse_expr()?;
        Some(State { name, type_name, default_value })
    }

    fn parse_function_def(&mut self) -> Option<FunctionDef> {
        self.consume(&Token::Fn);
        let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        self.expect(Token::OpenParen);
        let mut params = Vec::new();
        while self.peek() != Some(&Token::CloseParen) && !self.is_eof() {
            if let Some(Token::Identifier(id)) = self.advance_opt() { params.push(id); }
            if self.peek() == Some(&Token::Comma) { self.advance_opt(); }
        }
        self.expect(Token::CloseParen);
        self.expect(Token::OpenBrace);
        let mut body = Vec::new();
        while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
            if let Some(n) = self.parse_node() { body.push(n); } else { break; }
        }
        self.expect(Token::CloseBrace);
        Some(FunctionDef { name, params, body })
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
        if self.consume(&Token::Else) {
            self.expect(Token::OpenBrace);
            while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
                if let Some(n) = self.parse_node() { else_branch.push(n); } else { break; }
            }
            self.expect(Token::CloseBrace);
        }
        Some(Node::IfElse { condition, then_branch, else_branch })
    }

    fn parse_for(&mut self) -> Option<Node> {
        self.consume(&Token::For);
        let var = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        self.expect(Token::In);
        let collection = self.parse_expr()?;
        self.expect(Token::OpenBrace);
        let mut body = Vec::new();
        while self.peek() != Some(&Token::CloseBrace) && !self.is_eof() {
            if let Some(n) = self.parse_node() { body.push(n); } else { break; }
        }
        self.expect(Token::CloseBrace);
        Some(Node::ForLoop { var, collection, body })
    }

    fn parse_element(&mut self) -> Option<Element> {
        let name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
        let mut properties = Vec::new();
        let mut on_click = None;
        if self.consume(&Token::OpenParen) {
            while self.peek() != Some(&Token::CloseParen) && !self.is_eof() {
                let prop_name = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { break; };
                self.expect(Token::Colon);
                if let Some(val) = self.parse_expr() { properties.push(Property { name: prop_name, value: val }); }
                if self.peek() == Some(&Token::Comma) { self.advance_opt(); }
            }
            self.expect(Token::CloseParen);
        }
        let mut children = Vec::new();
        if self.consume(&Token::OpenBrace) {
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
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Option<Expr> {
        let mut left = self.parse_logical_and()?;
        while self.consume(&Token::Or) {
            let right = self.parse_logical_and()?;
            left = Expr::BinaryOp { left: Box::new(left), op: Token::Or, right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_logical_and(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison()?;
        while self.consume(&Token::And) {
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp { left: Box::new(left), op: Token::And, right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut left = self.parse_additive()?;
        if let Some(tok) = self.peek().cloned() {
            if matches!(tok, Token::Eq | Token::Neq | Token::Lt | Token::Gt | Token::Le | Token::Ge) {
                self.advance_opt();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp { left: Box::new(left), op: tok, right: Box::new(right) };
            }
        }
        Some(left)
    }

    fn parse_additive(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplicative()?;
        while let Some(tok) = self.peek().cloned() {
            if matches!(tok, Token::Plus | Token::Minus) {
                self.advance_opt();
                let right = self.parse_multiplicative()?;
                left = Expr::BinaryOp { left: Box::new(left), op: tok, right: Box::new(right) };
            } else { break; }
        }
        Some(left)
    }

    fn parse_multiplicative(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary()?;
        while let Some(tok) = self.peek().cloned() {
            if matches!(tok, Token::Star | Token::Slash) {
                self.advance_opt();
                let right = self.parse_unary()?;
                left = Expr::BinaryOp { left: Box::new(left), op: tok, right: Box::new(right) };
            } else { break; }
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if let Some(tok) = self.peek().cloned() {
            if matches!(tok, Token::Not | Token::Minus) {
                self.advance_opt();
                let expr = self.parse_unary()?;
                return Some(Expr::UnaryOp { op: tok, expr: Box::new(expr) });
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let tok = self.advance_opt()?;
        match tok {
            Token::StringLiteral(s) => Some(Expr::Literal(s)),
            Token::InterpolatedString { parts, vars } => Some(Expr::Interpolated { parts, vars }),
            Token::Number(n) => Some(Expr::Number(n)),
            Token::Identifier(id) => {
                if self.consume(&Token::OpenParen) {
                    let mut args = Vec::new();
                    while self.peek() != Some(&Token::CloseParen) && !self.is_eof() {
                        if let Some(arg) = self.parse_expr() { args.push(arg); }
                        if self.peek() == Some(&Token::Comma) { self.advance_opt(); }
                    }
                    self.expect(Token::CloseParen);
                    Some(Expr::Call { name: id, args })
                } else {
                    Some(Expr::Identifier(id))
                }
            }
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen);
                Some(expr)
            }
            Token::OpenBracket => {
                let mut items = Vec::new();
                while self.peek() != Some(&Token::CloseBracket) && !self.is_eof() {
                    if let Some(expr) = self.parse_expr() { items.push(expr); }
                    if self.peek() == Some(&Token::Comma) { self.advance_opt(); }
                }
                self.expect(Token::CloseBracket);
                Some(Expr::List(items))
            }
            Token::Bind => {
                let id = if let Some(Token::Identifier(id)) = self.advance_opt() { id } else { return None; };
                Some(Expr::Binding(id))
            }
            _ => None,
        }
    }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn is_eof(&self) -> bool { self.pos >= self.tokens.len() }
    fn advance_opt(&mut self) -> Option<Token> { if self.is_eof() { None } else { let t = self.tokens[self.pos].clone(); self.pos += 1; Some(t) } }
    fn consume(&mut self, token: &Token) -> bool { if self.peek() == Some(token) { self.pos += 1; true } else { false } }
    fn expect(&mut self, token: Token) {
        if !self.consume(&token) {
            eprintln!("parser: expected {:?}, got {:?}", token, self.peek());
        }
    }
}
