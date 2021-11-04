use std::fmt;

use crate::node::{Node, Operator};

type EvalResult = Result<Value, EvalError>;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Value {
    fn value_type(&self) -> String {
        match self {
            Self::Number(_) => "Number".into(),
            Self::String(_) => "String".into(),
            Self::Bool(b) => b.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvalError {
    TypeError(String),
    SyntaxError(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeError(e) => write!(f, "TypeError: {}", e),
            Self::SyntaxError(e) => write!(f, "SyntaxError: {}", e),
        }
    }
}

pub fn eval(node: Node) -> EvalResult {
    match node {
        Node::True => Ok(Value::Bool(true)),
        Node::False => Ok(Value::Bool(false)),
        Node::Int(n) => Ok(Value::Number(n)),
        Node::Str(string) => Ok(Value::String(string)),
        Node::UnaryExpr { op, child } => {
            let child = eval(*child);
            match op {
                Operator::Minus => match child? {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    Value::String(s) => Err(EvalError::TypeError(format!("{}", s))),
                    Value::Bool(b) => Err(EvalError::TypeError(format!("{}", b))),
                },

                Operator::Bang => match child? {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    v => Err(EvalError::SyntaxError(format!(
                        "cannot apply unary operator `!` to type {}",
                        v.value_type()
                    ))),
                },
                _ => child,
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs_ret = eval(*lhs)?;
            let rhs_ret = eval(*rhs)?;

            match (lhs_ret, rhs_ret) {
                (Value::Number(n1), Value::Number(n2)) => match op {
                    Operator::Plus => Ok(Value::Number(n1 + n2)),
                    Operator::Minus => Ok(Value::Number(n1 - n2)),
                    Operator::Multiply => Ok(Value::Number(n1 * n2)),
                    Operator::Divide => Ok(Value::Number(n1 / n2)),
                    Operator::Equality => Ok(Value::Bool(n1 == n2)),
                    Operator::NotEqual => Ok(Value::Bool(n1 != n2)),
                    Operator::GreaterThan => Ok(Value::Bool(n1 > n2)),
                    Operator::LessThan => Ok(Value::Bool(n1 < n2)),
                    Operator::GreaterEqual => Ok(Value::Bool(n1 >= n2)),
                    Operator::LessEqual => Ok(Value::Bool(n1 <= n2)),
                    Operator::Bang => Err(EvalError::SyntaxError(format!(
                        "Expected {} {{ + | - | * | / }} {} not a !",
                        n1, n2
                    ))),
                },
                (Value::String(s1), Value::String(s2)) => match op {
                    Operator::Plus => Ok(Value::String(s1 + &s2)),
                    Operator::Equality => Ok(Value::Bool(s1 == s2)),
                    Operator::NotEqual => Ok(Value::Bool(s1 != s2)),
                    _ => Err(EvalError::TypeError(format!(
                        "Can not {} a String with a String.",
                        op
                    ))),
                },
                (l, r) => Err(EvalError::TypeError(format!(
                    "Can not {} a {} with a {}.",
                    op,
                    l.value_type(),
                    r.value_type()
                ))),
            }
        }
    }
}
