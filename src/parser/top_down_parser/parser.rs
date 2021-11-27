// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

use crate::lexer::{KeyWord, SpannedToken, Token};
use crate::parser::ast::AST;

#[derive(Debug, Clone)]
pub struct ParseError {
    spanned_token: Option<SpannedToken>,
    msg: String,
}

pub struct Parser<'a> {
    tokens: &'a [SpannedToken],
    errors: Vec<ParseError>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [SpannedToken]) -> Self {
        Self {
            tokens,
            errors: Vec::new(),
            index: 0,
        }
    }
    pub fn parse(&mut self) -> AST {
        self.expression()
    }

    fn matches(&mut self, types: &[Token]) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().map(|t| t.token() == token).unwrap_or(false)
    }

    fn advance(&mut self) -> Option<&SpannedToken> {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .map(|t| t.token() == &Token::EOF)
            .unwrap_or(false)
    }

    fn peek(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.index + 1)
    }

    fn previous(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.index.saturating_sub(1))
    }

    ///expression     → equality ;
    fn expression(&mut self) -> AST {
        self.equality()
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    pub fn equality(&mut self) -> AST {
        let mut expr = self.comparison();

        while self.matches(&[Token::BangEqual, Token::EqualEqual]) {
            match self.previous() {
                Some(span_token) => {
                    let op: Token = span_token.token().clone();
                    let rhs: Box<AST> = Box::new(self.comparison());
                    expr = AST::BinaryExpr {
                        lhs: Box::new(expr),
                        op,
                        rhs,
                    };
                }
                None => break,
            }
        }

        expr
    }

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> AST {
        let mut expr = self.term();
        while self.matches(&[
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            match self.previous() {
                Some(span_token) => {
                    let op: Token = span_token.token().clone();
                    let rhs: Box<AST> = Box::new(self.term());
                    expr = AST::BinaryExpr {
                        lhs: Box::new(expr),
                        op,
                        rhs,
                    };
                }
                None => break,
            }
        }

        expr
    }

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> AST {
        let mut expr = self.factor();
        while self.matches(&[Token::Minus, Token::Plus]) {
            match self.previous() {
                Some(span_token) => {
                    let op: Token = span_token.token().clone();
                    let rhs: Box<AST> = Box::new(self.factor());
                    expr = AST::BinaryExpr {
                        lhs: Box::new(expr),
                        op,
                        rhs,
                    };
                }
                None => break,
            }
        }

        expr
    }

    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> AST {
        let mut expr = self.unary();
        while self.matches(&[Token::Slash, Token::Star]) {
            match self.previous() {
                Some(span_token) => {
                    let op: Token = span_token.token().clone();
                    let rhs: Box<AST> = Box::new(self.unary());
                    expr = AST::BinaryExpr {
                        lhs: Box::new(expr),
                        op,
                        rhs,
                    };
                }
                None => break,
            }
        }

        expr
    }

    /// unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> AST {
        if self.matches(&[Token::Bang, Token::Minus]) {
            match self.previous() {
                Some(span_token) => {
                    let op: Token = span_token.token().clone();
                    let child: Box<AST> = Box::new(self.unary());
                    return AST::UnaryExpr { op, child };
                }
                None => {}
            }
        }
        self.primary()
    }
    /// primary        → NUMBER | STRING | "true" | "false" | "(" expression ")" ;
    fn primary(&mut self) -> AST {
        match self.advance().map(Clone::clone) {
            Some(span_token) => match span_token.token() {
                Token::Int(n) => AST::Int(*n),
                Token::Float(f) => AST::Float(*f),
                Token::KeyWord(key_word) => match key_word {
                    KeyWord::True => AST::Bool(true),
                    KeyWord::False => AST::Bool(false),
                    _ => AST::Error {
                        span: Some(span_token.clone()),
                        msg: "".into(),
                    },
                },
                Token::String(s) => AST::String(s.clone()),
                Token::LeftParen => {
                    let expr = self.expression().clone();
                    let check = !self.check(&Token::RightParen);
                    if !check {
                        self.errors.push(ParseError {
                            spanned_token: Some(span_token.clone()),
                            msg: "Expect ')' after expression.".into(),
                        });
                    }
                    return AST::Grouping(Box::new(expr));
                }
                _ => AST::Error {
                    span: Some(span_token.clone()),
                    msg: "".into(),
                },
            },
            None => {
                self.errors.push(ParseError {
                    spanned_token: None,
                    msg: "I think this is the end of the file".into(),
                });
                AST::Error {
                    span: None,
                    msg: "I think this is the end of the file".into(),
                }
            }
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Value {
//     Float(f64),
//     Int(i128),
//     String(String),
//     Bool(bool),
//     NONE,
// }
//
// impl Value {
//     pub fn value_type(&self) -> String {
//         match self {
//             Self::Float(_) => "Float".into(),
//             Self::Int(_) => "Int".into(),
//             Self::String(_) => "String".into(),
//             Self::Bool(b) => b.to_string(),
//             Self::NONE => "NONE".to_string(),
//         }
//     }
// }
//
// impl std::fmt::Display for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Float(n) => write!(f, "{}", n),
//             Self::Int(n) => write!(f, "{}", n),
//             Self::String(s) => write!(f, "{}", s),
//             Self::Bool(b) => write!(f, "{}", b),
//             Self::NONE => write!(f, "NONE"),
//         }
//     }
// }
