use crate::lexer::{SpannedToken, Token};
// use std::fmt;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Operator {
//     Plus,
//     Minus,
//     Multiply,
//     Divide,
//     Bang,
//     Equality,
//     NotEqual,
//     GreaterThan,
//     GreaterEqual,
//     LessThan,
//     LessEqual,
//     Or,
//     And,
// }
//
// impl fmt::Display for Operator {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Plus => write!(f, "+"),
//             Self::Minus => write!(f, "-"),
//             Self::Multiply => write!(f, "*"),
//             Self::Divide => write!(f, "/"),
//             Self::Bang => write!(f, "!"),
//             Self::Equality => write!(f, "=="),
//             Self::NotEqual => write!(f, "!="),
//             Self::GreaterThan => write!(f, ">"),
//             Self::GreaterEqual => write!(f, ">="),
//             Self::LessThan => write!(f, "<"),
//             Self::LessEqual => write!(f, "<="),
//             Self::Or => write!(f, "or"),
//             Self::And => write!(f, "and"),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Call {
        ident: String,
        args: Vec<Box<AST>>,
    },
    ForeignFunction(StdFunc),
    Grouping(Box<AST>),
    Block(Vec<Box<AST>>),
    Bool(bool),
    Int(i128),
    Float(f64),
    String(String),
    Conditional {
        condition: Box<AST>,
        if_branch: Box<AST>,
        else_branch: Option<Box<AST>>,
    },
    UnaryExpr {
        op: Token,
        child: Box<AST>,
    },
    BinaryExpr {
        op: Token,
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Error {
        span: Option<SpannedToken>,
        msg: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StdFunc {
    Print(String),
}
