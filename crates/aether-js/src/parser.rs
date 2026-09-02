use crate::ast::*;
use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub msg: String,
    pub token_pos: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error at token {}: {}", self.token_pos, self.msg)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(tokens: &[Token]) -> Result<Vec<Stmt>, ParseError> {
    let mut p = Parser { tokens, pos: 0 };
    let mut stmts = Vec::new();
    while !p.at_end() {
        stmts.push(p.parse_stmt()?);
    }
    Ok(stmts)
}

impl<'a> Parser<'a> {
    fn at_end(&self) -> bool {
        matches!(self.current(), Token::Eof)
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        if !self.at_end() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        let t = self.advance();
        if std::mem::discriminant(&t) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(ParseError {
                msg: format!("expected {expected:?}, got {t:?}"),
                token_pos: self.pos,
            })
        }
    }

    fn eat_semicolon(&mut self) {
        if matches!(self.current(), Token::Semicolon) {
            self.advance();
        }
    }

    fn at_stmt_start(&self) -> bool {
        matches!(
            self.current(),
            Token::Var
                | Token::Let
                | Token::Const
                | Token::Function
                | Token::If
                | Token::While
                | Token::For
                | Token::Return
                | Token::Throw
                | Token::Try
                | Token::LBrace
                | Token::Do
                | Token::Switch
                | Token::Semicolon
        )
    }

    // ─── Statements ───

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.current().clone() {
            Token::Var => {
                self.advance();
                let name = self.expect_ident()?;
                let init = if matches!(self.current(), Token::Assign) {
                    self.advance();
                    Some(self.parse_expr(0)?)
                } else {
                    None
                };
                self.eat_semicolon();
                Ok(Stmt::VarDecl { name, init })
            }
            Token::Let => {
                self.advance();
                let name = self.expect_ident()?;
                let init = if matches!(self.current(), Token::Assign) {
                    self.advance();
                    Some(self.parse_expr(0)?)
                } else {
                    None
                };
                self.eat_semicolon();
                Ok(Stmt::LetDecl { name, init })
            }
            Token::Const => {
                self.advance();
                let name = self.expect_ident()?;
                self.expect(&Token::Assign)?;
                let init = self.parse_expr(0)?;
                self.eat_semicolon();
                Ok(Stmt::ConstDecl { name, init })
            }
            Token::Function => self.parse_function_decl(),
            Token::If => {
                self.advance();
                self.expect(&Token::LParen)?;
                let test = self.parse_expr(0)?;
                self.expect(&Token::RParen)?;
                let consequent = self.parse_stmt()?;
                let alternate = if matches!(self.current(), Token::Else) {
                    self.advance();
                    Some(Box::new(self.parse_stmt()?))
                } else {
                    None
                };
                Ok(Stmt::If {
                    test,
                    consequent: Box::new(consequent),
                    alternate,
                })
            }
            Token::While => {
                self.advance();
                self.expect(&Token::LParen)?;
                let test = self.parse_expr(0)?;
                self.expect(&Token::RParen)?;
                let body = self.parse_stmt()?;
                Ok(Stmt::While {
                    test,
                    body: Box::new(body),
                })
            }
            Token::For => {
                self.advance();
                self.expect(&Token::LParen)?;
                let init = if matches!(self.current(), Token::Semicolon) {
                    self.advance();
                    None
                } else if matches!(self.current(), Token::Var | Token::Let | Token::Const) {
                    let decl = self.parse_var_decl_in_for()?;
                    Some(Box::new(decl))
                } else {
                    let expr = self.parse_expr(0)?;
                    self.eat_semicolon();
                    Some(Box::new(Stmt::Expr(expr)))
                };

                if matches!(self.current(), Token::In) {
                    // for-in
                    let left = match init.as_ref().unwrap().as_ref() {
                        Stmt::VarDecl { name, .. } | Stmt::LetDecl { name, .. } => name.clone(),
                        Stmt::Expr(Expr::Identifier(name)) => name.clone(),
                        _ => {
                            return Err(ParseError {
                                msg: "invalid for-in left-hand side".to_string(),
                                token_pos: self.pos,
                            });
                        }
                    };
                    self.advance(); // skip 'in'
                    let right = self.parse_expr(0)?;
                    self.expect(&Token::RParen)?;
                    let body = self.parse_stmt()?;
                    return Ok(Stmt::ForIn {
                        left,
                        right,
                        body: Box::new(body),
                    });
                }

                let test = if matches!(self.current(), Token::Semicolon) {
                    self.advance();
                    None
                } else {
                    let e = self.parse_expr(0)?;
                    self.eat_semicolon();
                    Some(e)
                };

                let update = if matches!(self.current(), Token::RParen) {
                    None
                } else {
                    let e = self.parse_expr(0)?;
                    Some(e)
                };

                self.expect(&Token::RParen)?;
                let body = self.parse_stmt()?;
                Ok(Stmt::For {
                    init,
                    test,
                    update,
                    body: Box::new(body),
                })
            }
            Token::Return => {
                self.advance();
                let val = if self.at_stmt_start() || matches!(self.current(), Token::Semicolon) {
                    None
                } else {
                    Some(self.parse_expr(0)?)
                };
                self.eat_semicolon();
                Ok(Stmt::Return(val))
            }
            Token::Throw => {
                self.advance();
                let val = self.parse_expr(0)?;
                self.eat_semicolon();
                Ok(Stmt::Throw(val))
            }
            Token::Break => {
                self.advance();
                self.eat_semicolon();
                Ok(Stmt::Empty)
            }
            Token::Continue => {
                self.advance();
                self.eat_semicolon();
                Ok(Stmt::Empty)
            }
            Token::Try => {
                self.advance();
                let body = Box::new(self.parse_stmt()?);
                let catch = if matches!(self.current(), Token::Catch) {
                    self.advance();
                    let param = if matches!(self.current(), Token::LParen) {
                        self.advance();
                        let p = self.expect_ident()?;
                        self.expect(&Token::RParen)?;
                        p
                    } else {
                        "e".to_string()
                    };
                    let block = Box::new(self.parse_stmt()?);
                    Some((param, block))
                } else {
                    None
                };
                let finally = if matches!(self.current(), Token::Finally) {
                    self.advance();
                    Some(Box::new(self.parse_stmt()?))
                } else {
                    None
                };
                Ok(Stmt::Try {
                    body,
                    catch,
                    finally,
                })
            }
            Token::LBrace => {
                self.advance();
                let mut stmts = Vec::new();
                while !matches!(self.current(), Token::RBrace) {
                    stmts.push(self.parse_stmt()?);
                }
                self.expect(&Token::RBrace)?;
                Ok(Stmt::Block(stmts))
            }
            Token::Semicolon => {
                self.advance();
                Ok(Stmt::Empty)
            }
            _ => {
                let expr = self.parse_expr(0)?;
                self.eat_semicolon();
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_function_decl(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&Token::Function)?;
        let name = self.expect_ident()?;
        self.expect(&Token::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(&Token::RParen)?;
        let body = self.parse_function_body()?;
        Ok(Stmt::FunctionDecl { name, params, body })
    }

    fn parse_function_body(&mut self) -> Result<Box<Stmt>, ParseError> {
        self.expect(&Token::LBrace)?;
        let mut stmts = Vec::new();
        while !matches!(self.current(), Token::RBrace) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&Token::RBrace)?;
        Ok(Box::new(Stmt::Block(stmts)))
    }

    fn parse_param_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();
        if !matches!(self.current(), Token::RParen) {
            params.push(self.expect_ident()?);
            while matches!(self.current(), Token::Comma) {
                self.advance();
                params.push(self.expect_ident()?);
            }
        }
        Ok(params)
    }

    fn parse_var_decl_in_for(&mut self) -> Result<Stmt, ParseError> {
        let kw = self.advance();
        let name = self.expect_ident()?;
        let init = if matches!(self.current(), Token::Assign) {
            self.advance();
            Some(self.parse_expr(0)?)
        } else {
            None
        };
        self.eat_semicolon();
        match kw {
            Token::Var => Ok(Stmt::VarDecl { name, init }),
            Token::Let => Ok(Stmt::LetDecl { name, init }),
            Token::Const => Ok(Stmt::ConstDecl {
                name,
                init: init.unwrap_or(Expr::Literal(Lit::Undefined)),
            }),
            _ => unreachable!(),
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Token::Ident(s) => Ok(s),
            t => Err(ParseError {
                msg: format!("expected identifier, got {t:?}"),
                token_pos: self.pos,
            }),
        }
    }

    // ─── Expressions ───

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary()?;

        loop {
            let op = match self.current() {
                Token::Plus => Some(BinOp::Add),
                Token::Minus => Some(BinOp::Sub),
                Token::Star => Some(BinOp::Mul),
                Token::Slash => Some(BinOp::Div),
                Token::Percent => Some(BinOp::Mod),
                Token::StarStar => Some(BinOp::Pow),
                Token::EqEq => Some(BinOp::Eq),
                Token::EqEqEq => Some(BinOp::StrictEq),
                Token::NotEq => Some(BinOp::Ne),
                Token::NotEqEq => Some(BinOp::StrictNe),
                Token::Lt => Some(BinOp::Lt),
                Token::Gt => Some(BinOp::Gt),
                Token::LtEq => Some(BinOp::LtEq),
                Token::GtEq => Some(BinOp::GtEq),
                Token::AmpAmp => Some(BinOp::And),
                Token::PipePipe => Some(BinOp::Or),
                Token::Amp => Some(BinOp::BitAnd),
                Token::Pipe => Some(BinOp::BitOr),
                Token::Caret => Some(BinOp::BitXor),
                Token::LtLt => Some(BinOp::Shl),
                Token::GtGt => Some(BinOp::Shr),
                Token::GtGtGt => Some(BinOp::UShr),
                Token::In => Some(BinOp::In),
                Token::Instanceof => Some(BinOp::Instanceof),
                _ => None,
            };

            if let Some(op) = op {
                let bp = self.infix_bp(op);
                if bp <= min_bp {
                    break;
                }
                self.advance();
                let right = self.parse_expr(bp)?;
                left = Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
                continue;
            }

            // Assignment operators
            if self.is_assign_op() {
                let aop = self.assign_op();
                self.advance();
                let right = self.parse_expr(0)?;
                left = Expr::Assignment {
                    op: aop,
                    target: Box::new(left),
                    value: Box::new(right),
                };
                continue;
            }

            // Call
            if matches!(self.current(), Token::LParen) {
                self.advance();
                let args = self.parse_args()?;
                self.expect(&Token::RParen)?;
                left = Expr::Call {
                    callee: Box::new(left),
                    args,
                };
                continue;
            }

            // Member (dot)
            if matches!(self.current(), Token::Dot) {
                self.advance();
                let prop = match self.advance() {
                    Token::Ident(s) => Expr::Identifier(s),
                    t => {
                        return Err(ParseError {
                            msg: format!("expected property name, got {t:?}"),
                            token_pos: self.pos,
                        });
                    }
                };
                left = Expr::Member {
                    object: Box::new(left),
                    property: Box::new(prop),
                    computed: false,
                };
                continue;
            }

            // Member (computed)
            if matches!(self.current(), Token::LBracket) {
                self.advance();
                let prop = self.parse_expr(0)?;
                self.expect(&Token::RBracket)?;
                left = Expr::Member {
                    object: Box::new(left),
                    property: Box::new(prop),
                    computed: true,
                };
                continue;
            }

            // Ternary
            if matches!(self.current(), Token::Question) {
                self.advance();
                let consequent = self.parse_expr(0)?;
                self.expect(&Token::Colon)?;
                let alternate = self.parse_expr(0)?;
                left = Expr::Conditional {
                    test: Box::new(left),
                    consequent: Box::new(consequent),
                    alternate: Box::new(alternate),
                };
                continue;
            }

            break;
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Literal(Lit::Number(n)))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::Literal(Lit::String(s)))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Expr::Literal(Lit::Bool(b)))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Literal(Lit::Null))
            }
            Token::Undefined => {
                self.advance();
                Ok(Expr::Literal(Lit::Undefined))
            }
            Token::This => {
                self.advance();
                Ok(Expr::This)
            }
            Token::Ident(s) => {
                self.advance();
                Ok(Expr::Identifier(s))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Token::Function => self.parse_function_expr(),
            Token::New => {
                self.advance();
                let callee = self.parse_expr(0)?;
                let args = if matches!(self.current(), Token::LParen) {
                    self.advance();
                    let a = self.parse_args()?;
                    self.expect(&Token::RParen)?;
                    a
                } else {
                    Vec::new()
                };
                Ok(Expr::New {
                    callee: Box::new(callee),
                    args,
                })
            }
            // Unary prefix operators
            Token::Minus
            | Token::Bang
            | Token::Tilde
            | Token::Typeof
            | Token::Void
            | Token::Delete => {
                let op = match self.current() {
                    Token::Minus => UnaryOp::Neg,
                    Token::Bang => UnaryOp::Not,
                    Token::Tilde => UnaryOp::BitNot,
                    Token::Typeof => UnaryOp::TypeOf,
                    Token::Void => UnaryOp::Void,
                    Token::Delete => UnaryOp::Delete,
                    _ => unreachable!(),
                };
                self.advance();
                let operand = self.parse_primary()?;
                Ok(Expr::Unary {
                    op,
                    operand: Box::new(operand),
                    prefix: true,
                })
            }
            Token::PlusPlus | Token::MinusMinus => {
                self.advance();
                let operand = self.parse_primary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    operand: Box::new(operand),
                    prefix: true,
                })
            }
            t => Err(ParseError {
                msg: format!("unexpected token {t:?}"),
                token_pos: self.pos,
            }),
        }
    }

    fn parse_function_expr(&mut self) -> Result<Expr, ParseError> {
        self.expect(&Token::Function)?;
        let name = if matches!(self.current(), Token::Ident(_)) {
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&Token::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(&Token::RParen)?;
        let body = self.parse_function_body()?;
        Ok(Expr::FunctionExpr { name, params, body })
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();
        if !matches!(self.current(), Token::RParen) {
            args.push(self.parse_expr(0)?);
            while matches!(self.current(), Token::Comma) {
                self.advance();
                args.push(self.parse_expr(0)?);
            }
        }
        Ok(args)
    }

    fn is_assign_op(&self) -> bool {
        matches!(
            self.current(),
            Token::Assign
                | Token::PlusAssign
                | Token::MinusAssign
                | Token::StarAssign
                | Token::SlashAssign
                | Token::PercentAssign
                | Token::StarStarAssign
                | Token::AmpEq
                | Token::PipeEq
                | Token::CaretEq
                | Token::LtLtEq
                | Token::GtGtEq
                | Token::GtGtGtEq
        )
    }

    fn assign_op(&self) -> AssignOp {
        match self.current() {
            Token::Assign => AssignOp::Assign,
            Token::PlusAssign => AssignOp::AddAssign,
            Token::MinusAssign => AssignOp::SubAssign,
            Token::StarAssign => AssignOp::MulAssign,
            Token::SlashAssign => AssignOp::DivAssign,
            Token::PercentAssign => AssignOp::ModAssign,
            Token::StarStarAssign => AssignOp::PowAssign,
            Token::AmpEq => AssignOp::AndAssign,
            Token::PipeEq => AssignOp::OrAssign,
            Token::CaretEq => AssignOp::XorAssign,
            Token::LtLtEq => AssignOp::ShlAssign,
            Token::GtGtEq => AssignOp::ShrAssign,
            Token::GtGtGtEq => AssignOp::UShrAssign,
            _ => AssignOp::Assign,
        }
    }

    fn infix_bp(&self, op: BinOp) -> u8 {
        match op {
            BinOp::Or => 1,
            BinOp::And => 2,
            BinOp::BitOr => 3,
            BinOp::BitXor => 4,
            BinOp::BitAnd => 5,
            BinOp::Eq | BinOp::StrictEq | BinOp::Ne | BinOp::StrictNe => 6,
            BinOp::Lt | BinOp::Gt | BinOp::LtEq | BinOp::GtEq | BinOp::In | BinOp::Instanceof => 7,
            BinOp::Shl | BinOp::Shr | BinOp::UShr => 8,
            BinOp::Add | BinOp::Sub => 9,
            BinOp::Mul | BinOp::Div | BinOp::Mod => 10,
            BinOp::Pow => 11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    fn parse_str(s: &str) -> Vec<Stmt> {
        let toks = tokenize(s).unwrap();
        parse(&toks).unwrap()
    }

    #[test]
    fn number_literal() {
        let stmts = parse_str("42;");
        assert!(matches!(
            &stmts[0],
            Stmt::Expr(Expr::Literal(Lit::Number(42.0)))
        ));
    }

    #[test]
    fn binary_expr() {
        let stmts = parse_str("1 + 2 * 3;");
        assert!(matches!(&stmts[0], Stmt::Expr(Expr::Binary { .. })));
    }

    #[test]
    fn var_decl() {
        let stmts = parse_str("var x = 10;");
        assert!(matches!(
            &stmts[0],
            Stmt::VarDecl { name, init: Some(_) } if name == "x"
        ));
    }

    #[test]
    fn function_decl() {
        let stmts = parse_str("function add(a, b) { return a + b; }");
        assert!(matches!(
            &stmts[0],
            Stmt::FunctionDecl { name, params, .. } if name == "add" && params.len() == 2
        ));
    }

    #[test]
    fn if_else() {
        let stmts = parse_str("if (true) { 1; } else { 2; }");
        assert!(matches!(&stmts[0], Stmt::If { .. }));
    }

    #[test]
    fn while_loop() {
        let stmts = parse_str("while (false) { break; }");
        assert!(matches!(&stmts[0], Stmt::While { .. }));
    }

    #[test]
    fn try_catch() {
        let stmts = parse_str("try { throw 1; } catch (e) { 2; }");
        assert!(matches!(&stmts[0], Stmt::Try { catch: Some(_), .. }));
    }

    #[test]
    fn nested_calls() {
        let stmts = parse_str("foo(1, bar(2));");
        assert!(matches!(&stmts[0], Stmt::Expr(Expr::Call { .. })));
    }
}
