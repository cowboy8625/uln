use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Bang,
    Equality,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Bang => write!(f, "!"),
            Self::Equality => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Print(Box<Node>),
    True,
    False,
    Int(i128),
    Float(f64),
    Str(String),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
