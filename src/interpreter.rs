use std::{collections::HashMap, fmt};

use crate::node::{Node, Operator};

type EvalResult = Result<(Value, Environment), (EvalError, Environment)>;

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i128),
    String(String),
    Bool(bool),
    NONE,
}

impl Value {
    fn value_type(&self) -> String {
        match self {
            Self::Float(_) => "Float".into(),
            Self::Int(_) => "Int".into(),
            Self::String(_) => "String".into(),
            Self::Bool(b) => b.to_string(),
            Self::NONE => "NONE".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(n) => write!(f, "{}", n),
            Self::Int(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::NONE => write!(f, "NONE"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvalError {
    TypeError(String),
    SyntaxError(String),
    UnKnownIdent(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeError(e) => write!(f, "TypeError: {}", e),
            Self::SyntaxError(e) => write!(f, "SyntaxError: {}", e),
            Self::UnKnownIdent(e) => write!(f, "UnKnownIdent: {}", e),
        }
    }
}

pub type Environment = HashMap<String, Node>;
pub fn eval(node: Node, mut env: Environment) -> EvalResult {
    match node {
        Node::Print(exp) => match eval(*exp, env) {
            Ok((v, env)) => {
                println!("{}", v);
                return Ok((Value::NONE, env));
            }
            err @ Err(_) => return err,
        },
        Node::Ident(ident) => match env.get(&ident) {
            Some(exp) => eval(exp.clone(), env),
            None => Err((EvalError::UnKnownIdent(ident), env)),
        },
        Node::Variable { ident, exp } => {
            env.insert(ident, *exp);
            return Ok((Value::NONE, env));
        }
        Node::True => Ok((Value::Bool(true), env)),
        Node::False => Ok((Value::Bool(false), env)),
        Node::Int(n) => Ok((Value::Int(n), env)),
        Node::Float(n) => Ok((Value::Float(n), env)),
        Node::Str(string) => Ok((Value::String(string), env)),
        Node::UnaryExpr { op, child } => {
            let (child, env) = eval(*child, env)?;
            match op {
                Operator::Minus => match child {
                    Value::Int(n) => Ok((Value::Int(-n), env)),
                    Value::Float(n) => Ok((Value::Float(-n), env)),
                    Value::String(s) => Err((EvalError::TypeError(format!("{}", s)), env)),
                    Value::Bool(b) => Err((EvalError::TypeError(format!("{}", b)), env)),
                    none @ Value::NONE => Err((EvalError::TypeError(format!("{}", none)), env)),
                },

                Operator::Bang => match child {
                    Value::Bool(b) => Ok((Value::Bool(!b), env)),
                    v => Err((
                        EvalError::SyntaxError(format!(
                            "cannot apply unary operator `!` to type {}",
                            v.value_type()
                        )),
                        env,
                    )),
                },
                _ => Ok((child, env)),
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let (lhs_ret, env) = eval(*lhs, env)?;
            let (rhs_ret, env) = eval(*rhs, env)?;

            match (lhs_ret, rhs_ret) {
                (Value::Int(n1), Value::Int(n2)) => match op {
                    Operator::Plus => Ok((Value::Int(n1 + n2), env)),
                    Operator::Minus => Ok((Value::Int(n1 - n2), env)),
                    Operator::Multiply => Ok((Value::Int(n1 * n2), env)),
                    Operator::Divide => Ok((Value::Int(n1 / n2), env)),
                    Operator::Equality => Ok((Value::Bool(n1 == n2), env)),
                    Operator::NotEqual => Ok((Value::Bool(n1 != n2), env)),
                    Operator::GreaterThan => Ok((Value::Bool(n1 > n2), env)),
                    Operator::LessThan => Ok((Value::Bool(n1 < n2), env)),
                    Operator::GreaterEqual => Ok((Value::Bool(n1 >= n2), env)),
                    Operator::LessEqual => Ok((Value::Bool(n1 <= n2), env)),
                    Operator::Bang => Err((
                        EvalError::SyntaxError(format!(
                            "Expected {} {{ + | - | * | / }} {} not a !",
                            n1, n2
                        )),
                        env,
                    )),
                },

                (Value::Float(n1), Value::Float(n2)) => match op {
                    Operator::Plus => Ok((Value::Float(n1 + n2), env)),
                    Operator::Minus => Ok((Value::Float(n1 - n2), env)),
                    Operator::Multiply => Ok((Value::Float(n1 * n2), env)),
                    Operator::Divide => Ok((Value::Float(n1 / n2), env)),
                    Operator::Equality => Ok((Value::Bool(n1 == n2), env)),
                    Operator::NotEqual => Ok((Value::Bool(n1 != n2), env)),
                    Operator::GreaterThan => Ok((Value::Bool(n1 > n2), env)),
                    Operator::LessThan => Ok((Value::Bool(n1 < n2), env)),
                    Operator::GreaterEqual => Ok((Value::Bool(n1 >= n2), env)),
                    Operator::LessEqual => Ok((Value::Bool(n1 <= n2), env)),
                    Operator::Bang => Err((
                        EvalError::SyntaxError(format!(
                            "Expected {} {{ + | - | * | / }} {} not a !",
                            n1, n2
                        )),
                        env,
                    )),
                },
                (Value::String(s1), Value::String(s2)) => match op {
                    Operator::Plus => Ok((Value::String(s1 + &s2), env)),
                    Operator::Equality => Ok((Value::Bool(s1 == s2), env)),
                    Operator::NotEqual => Ok((Value::Bool(s1 != s2), env)),
                    _ => Err((
                        EvalError::TypeError(format!("Can not {} a String with a String.", op)),
                        env,
                    )),
                },
                (Value::Bool(b1), Value::Bool(b2)) => match op {
                    Operator::Equality => Ok((Value::Bool(b1 == b2), env)),
                    Operator::NotEqual => Ok((Value::Bool(b1 != b2), env)),
                    Operator::GreaterThan => Ok((Value::Bool(b1 > b2), env)),
                    Operator::LessThan => Ok((Value::Bool(b1 < b2), env)),
                    Operator::GreaterEqual => Ok((Value::Bool(b1 >= b2), env)),
                    Operator::LessEqual => Ok((Value::Bool(b1 <= b2), env)),
                    op => Err((
                        EvalError::SyntaxError(format!(
                            "Can not use {} on type BOOL | {} {} {}",
                            op, b1, op, b2
                        )),
                        env,
                    )),
                },
                (l, r) => Err((
                    EvalError::TypeError(format!(
                        "Can not {} a {} with a {}.",
                        op,
                        l.value_type(),
                        r.value_type()
                    )),
                    env,
                )),
            }
        }
    }
}
