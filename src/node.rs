use crate::Environment;
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
    Or,
    And,
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
            Self::Or => write!(f, "or"),
            Self::And => write!(f, "and"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Print(Box<Node>),
    Ident {
        ident: String,
        args: Vec<Box<Node>>,
    },
    Variable {
        ident: String,
        param: Vec<String>,
        block: Box<Node>,
        environment: Option<Environment>,
    },
    True,
    False,
    Int(i128),
    Float(f64),
    Str(String),
    Block(Vec<Box<Node>>),
    Conditional {
        condition: Box<Node>,
        if_branch: Box<Node>,
        else_branch: Option<Box<Node>>,
    },
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
