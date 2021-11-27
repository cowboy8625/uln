// use crate::token::Token;
use crate::lexer::token::{KeyWord, Position, Span, Spanned, SpannedToken, Token};
use std::iter::Peekable;

pub struct Lexer {
    pos: Position,
    pub tokens: Vec<SpannedToken>,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            pos: Position::default(),
            tokens: Vec::new(),
        }
    }
}

impl Lexer {
    pub fn lex(&mut self, stream: &[char]) {
        if stream.is_empty() {
            self.add_token(Token::EOF);
            return ();
        }
        let mut stream = stream.iter().peekable();

        while let Some(c) = stream.next() {
            match c {
                '/' if stream.peek() == Some(&&'/') => self.commit(&mut stream),
                '/' => self.add_token(Token::Slash),
                '!' if stream.peek() == Some(&&'=') => self.add_token(Token::BangEqual),
                '!' => self.add_token(Token::Bang),
                '<' if stream.peek() == Some(&&'=') => self.add_token(Token::LessEqual),
                '<' => self.add_token(Token::Less),
                '>' if stream.peek() == Some(&&'=') => self.add_token(Token::GreaterEqual),
                '>' => self.add_token(Token::Greater),
                '=' if stream.peek() == Some(&&'=') => self.add_token(Token::EqualEqual),
                '=' => self.add_token(Token::Equal),
                '+' => self.add_token(Token::Plus),
                '-' if stream.peek().map_or(false, |c| c.is_numeric()) => {
                    self.add_number(c, &mut stream)
                }
                '-' => self.add_token(Token::Minus),
                '*' => self.add_token(Token::Star),
                '(' => self.add_token(Token::LeftParen),
                ')' => self.add_token(Token::RightParen),
                '[' => self.add_token(Token::LeftBracket),
                ']' => self.add_token(Token::RightBracket),
                '{' => self.add_token(Token::LeftBrace),
                '}' => self.add_token(Token::RightBrace),
                ',' => self.add_token(Token::Comma),
                '.' => self.add_token(Token::Dot),
                ';' => self.add_token(Token::Semicolon),
                '"' => self.add_string(&mut stream),
                ' ' => self.pos.right_shift(),
                '\n' => self.pos.new_line(),
                c if c.is_numeric() => self.add_number(c, &mut stream),
                c if c.is_ascii_alphabetic() => self.add_identifier(c, &mut stream),
                _ => self.error(c),
            }
        }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens
            .push(SpannedToken(Spanned::new(token, self.pos.into_span())));
        self.pos.right_shift();
    }

    fn commit(&mut self, stream: &mut Peekable<std::slice::Iter<'_, char>>) {
        while let Some(c) = stream.next() {
            self.pos.right_shift();
            if c == &'\n' {
                self.pos.new_line();
                break;
            }
        }
    }

    fn error(&mut self, c: &char) {
        self.tokens.push(SpannedToken(Spanned::new(
            Token::Error(format!("Unknown Char: {}", c)),
            self.pos.into_span(),
        )));
        self.pos.right_shift();
    }

    fn add_number(&mut self, c: &char, stream: &mut Peekable<std::slice::Iter<'_, char>>) {
        let mut number = c.to_string();
        let start = self.pos.clone();
        while let Some(c) =
            stream.next_if(|&c| c.is_numeric() || (c == &'.' && !number.contains('.')))
        {
            number.push(*c);
            self.pos.right_shift();
        }
        let token = if number.contains('.') {
            Token::Float(number.parse::<f64>().unwrap())
        } else {
            Token::Int(number.parse::<i128>().unwrap())
        };
        self.tokens.push(SpannedToken(Spanned::new(
            token,
            Span::new(start, self.pos),
        )));
        self.pos.right_shift();
    }

    fn add_identifier(&mut self, c: &char, stream: &mut Peekable<std::slice::Iter<'_, char>>) {
        let mut idt = c.to_string();
        let start = self.pos.clone();
        while let Some(c) = stream.next_if(|&c| c.is_ascii_alphabetic()) {
            idt.push(*c);
            self.pos.right_shift();
        }
        let token = KeyWord::lookup(&idt).map_or(Token::Identifier(idt), Token::KeyWord);
        self.tokens.push(SpannedToken(Spanned::new(
            token,
            Span::new(start, self.pos),
        )));
        self.pos.right_shift();
    }

    fn add_string(&mut self, stream: &mut Peekable<std::slice::Iter<'_, char>>) {
        let mut string = String::new();
        let start = self.pos.clone();
        while let Some(c) = stream.next() {
            if c == &'"' {
                let _ = stream.next();
                break;
            } else if c == &'\n' {
                string.push(*c);
                self.pos.new_line();
            } else {
                string.push(*c);
                self.pos.right_shift();
            }
        }
        self.tokens.push(SpannedToken(Spanned::new(
            Token::String(string),
            Span::new(start, self.pos),
        )));
        self.pos.right_shift();
    }
}

// fn add_token<T: ToString + Clone + std::fmt::Debug>(ttype: TokenType, c: T, stream: &mut ()) -> Token {
//     let t = Token::new(ttype, c.clone(), stream.row, stream.col);
//     for _ in 0..(c.to_string().len()) {
//         stream.next();
//     }
//     t
// }
//
//
// fn add_string(stream: &mut CharStream) -> Token {
//     let mut string = String::new();
//     let row = stream.row;
//     let col = stream.col;
//     stream.next();
//     while let Some(c) = stream.current {
//         if c == '"' {
//             stream.next();
//             break
//         }
//         string.push(c);
//         stream.next();
//     }
//     dbg!(stream);
//     Token::new(TokenType::String, &string, row, col)
// }
//
// fn add_number(stream: &mut CharStream) -> Token {
//     // check for multiple dots in float.
//     let mut number = stream.current.unwrap().to_string();
//     let row = stream.row;
//     let col = stream.col;
//     stream.next();
//     while let Some(c) = stream.current {
//         if c.is_numeric() || (c == '.' && !number.contains('.')) {
//             number.push(c);
//             stream.next();
//         } else {
//             break;
//         }
//     }
//     let ttype = if number.contains('.') {
//         TokenType::Float(number.parse::<f64>().unwrap())
//     } else {
//         TokenType::Int(number.parse::<i128>().unwrap())
//     };
//     Token::new(ttype, &number, row, col)
// }
//
// fn add_identifier(stream: &mut CharStream) -> Token {
//     let mut idt = stream.current.unwrap().to_string();
//     let row = stream.row;
//     let col = stream.col;
//     stream.next();
//     while let Some(c) = stream.current {
//         if c.is_ascii_alphabetic() {
//             idt.push(c);
//             stream.next();
//         } else {
//             break;
//         }
//     }
//     let ttype = KeyWord::lookup(&idt).map(TokenType::KeyWord).unwrap_or(TokenType::Identifier);
//     Token::new(ttype, &idt, row, col)
// }
