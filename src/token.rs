use crate::keyword::KeyWord;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeyWord(KeyWord),
    String(String),
    Int(i128),
    Float(f64),
    Identifier(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Error(String),
    #[allow(clippy::upper_case_acronyms)]
    EOF,
}
