use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,

    // Identifier
    Ident(String),

    // Keywords
    Var,
    Let,
    Const,
    Function,
    If,
    Else,
    While,
    For,
    Return,
    Throw,
    Try,
    Catch,
    Finally,
    New,
    This,
    True,
    False,
    NullKw,
    UndefinedKw,
    Void,
    Typeof,
    Delete,
    In,
    Instanceof,
    Do,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    With,
    Debugger,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    Eq,
    EqEq,
    EqEqEq,
    NotEq,
    NotEqEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    AmpAmp,
    PipePipe,
    Bang,
    Tilde,
    Question,
    Colon,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    StarStarAssign,
    PlusPlus,
    MinusMinus,
    LtLt,
    GtGt,
    GtGtGt,
    Amp,
    Pipe,
    Caret,
    AmpEq,
    PipeEq,
    CaretEq,
    LtLtEq,
    GtGtEq,
    GtGtGtEq,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Dot,

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub msg: String,
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}:{}: {}", self.line, self.col, self.msg)
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut line = 1;
    let mut col = 1;

    while i < chars.len() {
        let c = chars[i];

        // Skip whitespace
        if c.is_ascii_whitespace() {
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            i += 1;
            continue;
        }

        // Skip line comments
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Skip block comments
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            i += 2;
            col += 2;
            while i < chars.len() {
                if chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                    i += 2;
                    col += 2;
                    break;
                }
                if chars[i] == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
                i += 1;
            }
            continue;
        }

        // Numbers
        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
                col += 1;
            }
            if i < chars.len() && chars[i] == '.' {
                i += 1;
                col += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                    col += 1;
                }
            }
            if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                col += 1;
                if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                    col += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                    col += 1;
                }
            }
            let s: String = chars[start..i].iter().collect();
            let n: f64 = s.parse().map_err(|_| LexError {
                msg: format!("invalid number: {s}"),
                line,
                col,
            })?;
            tokens.push(Token::Number(n));
            continue;
        }

        // Strings
        if c == '\'' || c == '"' {
            let quote = c;
            i += 1;
            col += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                    col += 1;
                    match chars[i] {
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '\'' => s.push('\''),
                        '"' => s.push('"'),
                        '0' => s.push('\0'),
                        _ => {
                            s.push('\\');
                            s.push(chars[i]);
                        }
                    }
                } else {
                    if chars[i] == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                    s.push(chars[i]);
                }
                i += 1;
            }
            if i >= chars.len() {
                return Err(LexError {
                    msg: "unterminated string".to_string(),
                    line,
                    col,
                });
            }
            i += 1;
            col += 1;
            tokens.push(Token::String(s));
            continue;
        }

        // Identifiers and keywords
        if c.is_ascii_alphabetic() || c == '_' || c == '$' {
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '$')
            {
                i += 1;
                col += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let tok = match word.as_str() {
                "var" => Token::Var,
                "let" => Token::Let,
                "const" => Token::Const,
                "function" => Token::Function,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "for" => Token::For,
                "return" => Token::Return,
                "throw" => Token::Throw,
                "try" => Token::Try,
                "catch" => Token::Catch,
                "finally" => Token::Finally,
                "new" => Token::New,
                "this" => Token::This,
                "true" => Token::Bool(true),
                "false" => Token::Bool(false),
                "null" => Token::Null,
                "undefined" => Token::Undefined,
                "void" => Token::Void,
                "typeof" => Token::Typeof,
                "delete" => Token::Delete,
                "in" => Token::In,
                "instanceof" => Token::Instanceof,
                "do" => Token::Do,
                "switch" => Token::Switch,
                "case" => Token::Case,
                "default" => Token::Default,
                "break" => Token::Break,
                "continue" => Token::Continue,
                "with" => Token::With,
                "debugger" => Token::Debugger,
                _ => Token::Ident(word),
            };
            tokens.push(tok);
            continue;
        }

        // Multi-char operators and delimiters
        let peek_char = |i_val: usize, offset: usize| -> Option<char> {
            let idx = i_val + offset;
            if idx < chars.len() {
                Some(chars[idx])
            } else {
                None
            }
        };

        let tok = match c {
            '+' => match peek_char(i, 1) {
                Some('+') => {
                    i += 1;
                    col += 1;
                    Token::PlusPlus
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::PlusAssign
                }
                _ => Token::Plus,
            },
            '-' => match peek_char(i, 1) {
                Some('-') => {
                    i += 1;
                    col += 1;
                    Token::MinusMinus
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::MinusAssign
                }
                _ => Token::Minus,
            },
            '*' => match peek_char(i, 1) {
                Some('*') => {
                    i += 1;
                    col += 1;
                    match peek_char(i, 1) {
                        Some('=') => {
                            i += 1;
                            col += 1;
                            Token::StarStarAssign
                        }
                        _ => Token::StarStar,
                    }
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::StarAssign
                }
                _ => Token::Star,
            },
            '/' => match peek_char(i, 1) {
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::SlashAssign
                }
                _ => Token::Slash,
            },
            '%' => match peek_char(i, 1) {
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::PercentAssign
                }
                _ => Token::Percent,
            },
            '=' => match peek_char(i, 1) {
                Some('=') => {
                    i += 1;
                    col += 1;
                    match peek_char(i, 1) {
                        Some('=') => {
                            i += 1;
                            col += 1;
                            Token::EqEqEq
                        }
                        _ => Token::EqEq,
                    }
                }
                _ => Token::Assign,
            },
            '!' => match peek_char(i, 1) {
                Some('=') => {
                    i += 1;
                    col += 1;
                    match peek_char(i, 1) {
                        Some('=') => {
                            i += 1;
                            col += 1;
                            Token::NotEqEq
                        }
                        _ => Token::NotEq,
                    }
                }
                _ => Token::Bang,
            },
            '<' => match peek_char(i, 1) {
                Some('<') => {
                    i += 1;
                    col += 1;
                    match peek_char(i, 1) {
                        Some('=') => {
                            i += 1;
                            col += 1;
                            Token::LtLtEq
                        }
                        _ => Token::LtLt,
                    }
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::LtEq
                }
                _ => Token::Lt,
            },
            '>' => match peek_char(i, 1) {
                Some('>') => {
                    i += 1;
                    col += 1;
                    match peek_char(i, 1) {
                        Some('>') => {
                            i += 1;
                            col += 1;
                            match peek_char(i, 1) {
                                Some('=') => {
                                    i += 1;
                                    col += 1;
                                    Token::GtGtGtEq
                                }
                                _ => Token::GtGtGt,
                            }
                        }
                        Some('=') => {
                            i += 1;
                            col += 1;
                            Token::GtGtEq
                        }
                        _ => Token::GtGt,
                    }
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::GtEq
                }
                _ => Token::Gt,
            },
            '&' => match peek_char(i, 1) {
                Some('&') => {
                    i += 1;
                    col += 1;
                    Token::AmpAmp
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::AmpEq
                }
                _ => Token::Amp,
            },
            '|' => match peek_char(i, 1) {
                Some('|') => {
                    i += 1;
                    col += 1;
                    Token::PipePipe
                }
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::PipeEq
                }
                _ => Token::Pipe,
            },
            '^' => match peek_char(i, 1) {
                Some('=') => {
                    i += 1;
                    col += 1;
                    Token::CaretEq
                }
                _ => Token::Caret,
            },
            '~' => Token::Tilde,
            '?' => Token::Question,
            ':' => Token::Colon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => {
                if peek_char(i, 1).is_some_and(|c| c.is_ascii_digit()) {
                    let start = i;
                    i += 1;
                    col += 1;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                        col += 1;
                    }
                    let s: String = chars[start..i].iter().collect();
                    let n: f64 = s.parse().unwrap();
                    tokens.push(Token::Number(n));
                    continue;
                }
                Token::Dot
            }
            _ => {
                return Err(LexError {
                    msg: format!("unexpected character: {c}"),
                    line,
                    col,
                });
            }
        };

        tokens.push(tok);
        i += 1;
        col += 1;
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let toks = tokenize("42 3.15 1e10 .5").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Number(42.0),
                Token::Number(3.15),
                Token::Number(1e10),
                Token::Number(0.5),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn strings() {
        let toks = tokenize("'hello' \"world\\n\"").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::String("hello".to_string()),
                Token::String("world\n".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn keywords_and_idents() {
        let toks = tokenize("var x = true;").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Var,
                Token::Ident("x".to_string()),
                Token::Assign,
                Token::Bool(true),
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn operators() {
        let toks = tokenize("a + b * 2 == 3 !== c").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Ident("a".to_string()),
                Token::Plus,
                Token::Ident("b".to_string()),
                Token::Star,
                Token::Number(2.0),
                Token::EqEq,
                Token::Number(3.0),
                Token::NotEqEq,
                Token::Ident("c".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn comments_skipped() {
        let toks = tokenize("1 /* block */ + 2 // line").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
                Token::Eof,
            ]
        );
    }
}
