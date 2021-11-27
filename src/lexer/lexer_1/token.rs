#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub idx: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, idx: usize) -> Self {
        Self { line, column, idx }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
        self.idx += 1;
    }

    pub fn right_shift(&mut self) {
        self.column += 1;
        self.idx += 1;
    }

    pub fn into_span(self) -> Span {
        Span {
            start: self,
            end: self,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            idx: 0,
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned {
    pub content: Token,
    span: Span,
}
impl Spanned {
    pub fn new(content: Token, span: Span) -> Self {
        Self { content, span }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken(pub Spanned);

impl SpannedToken {
    pub fn to_span(&self) -> Span {
        self.0.span()
    }
    pub fn token(&self) -> &Token {
        &self.0.content
    }
    pub fn kind(&self) -> TokenKind {
        self.token().kind()
    }
}

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
    LeftBracket,
    RightBracket,
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

impl Token {
    pub fn kind(&self) -> TokenKind {
        match *self {
            Token::Identifier(_) => TokenKind::Identifier,
            Token::Int(_) | Token::Float(_) | Token::String(_) => TokenKind::Literal,
            Token::KeyWord(_) => TokenKind::Keyword,
            ref tok => TokenKind::Token(tok.clone()),
        }
    }

    pub fn is_variant(&self, tok: &Token) -> bool {
        let got_variant = core::mem::discriminant(self);
        let expected_variant = core::mem::discriminant(tok);
        let same_token_variant = got_variant == expected_variant;
        if !same_token_variant {
            return false;
        }
        // Check if the keywords are the same
        // Two tokens can be the same variant, but have different inner values
        // We especially care about the Keyword value however
        match (&self, tok) {
            (Token::KeyWord(x), Token::KeyWord(y)) => return x == y,
            (_, _) => {}
        };

        // If we arrive here, then the Token variants are the same and they are not the Keyword type
        same_token_variant
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Token(Token),
    Identifier,
    Literal,
    Keyword,
    Attribute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyWord {
    False,
    Fn,
    If,
    Or,
    Print,
    Return,
    True,
    Let,
    And,
    Else,
}

impl KeyWord {
    pub fn lookup(name: &str) -> Option<Self> {
        use KeyWord::*;
        match name {
            "true" => Some(True),
            "false" => Some(False),
            "fn" => Some(Fn),
            "if" => Some(If),
            "or" => Some(Or),
            "print" => Some(Print),
            "return" => Some(Return),
            "let" => Some(Let),
            "and" => Some(And),
            "else" => Some(Else),
            _ => None,
        }
    }
}
