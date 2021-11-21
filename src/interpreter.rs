#[cfg(test)]
use pretty_assertions::assert_eq;
use std::{collections::HashMap, fmt};

use crate::node::{Node, Operator};
use crate::value::Value;

type EvalResult = Result<(Value, Environment), (EvalError, Environment)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    TypeError(String),
    SyntaxError(String),
    UnKnownIdent(String),
    Mutations(String),
    MismatchedType(String),
    FunctionParameters(usize, usize),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeError(e) => write!(f, "TypeError: {}", e),
            Self::SyntaxError(e) => write!(f, "SyntaxError: {}", e),
            Self::UnKnownIdent(e) => write!(f, "UnKnownIdent: {}", e),
            Self::Mutations(v) => write!(
                f,
                "Mutation is not allowed! Variable `{}` already exists.",
                v
            ),
            Self::MismatchedType(e) => write!(f, "MismatchedType: {}", e),
            Self::FunctionParameters(param, args) => write!(
                f,
                "Function Parameter do not match with calling arguments. Expected: {} but got {}",
                param, args
            ),
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
        Node::Ident { ident, args } => match env.get(&ident) {
            Some(node) => match node {
                Node::Variable { param, block, .. } => {
                    let mut block_env = Environment::new();
                    for (p, arg) in param.iter().zip(args) {
                        block_env.insert(p.clone(), *arg);
                    }
                    match execute_block(*block.clone(), block_env) {
                        Ok((v, e)) => Ok((v, env)),
                        Err((input, e)) => {
                            println!("INPUT: {:?} ENV: {:#?}", input, e);
                            eval(*block.clone(), env)
                        }
                    }
                }
                n => {
                    eval(n.clone(), env)
                    //     Err((
                    //     EvalError::TypeError("I think this is the wrong thing maybe.....".into()),
                    //     env,
                    // ));
                }
            },
            None => {
                // println!("VAR env: {:?}", env);
                // println!("VAR args: {:?}", args);
                Err((EvalError::UnKnownIdent(ident), env))
            }
        },
        Node::Variable {
            ident,
            param,
            block,
        } => {
            if env.contains_key(&ident) {
                return Err((EvalError::Mutations(ident), env));
            }
            env.insert(
                ident.clone(),
                Node::Variable {
                    ident,
                    param,
                    block,
                },
            );
            return Ok((Value::NONE, env));
        }
        Node::True => Ok((Value::Bool(true), env)),
        Node::False => Ok((Value::Bool(false), env)),
        Node::Int(n) => Ok((Value::Int(n), env)),
        Node::Float(n) => Ok((Value::Float(n), env)),
        Node::Str(string) => Ok((Value::String(string), env)),
        Node::Conditional {
            condition,
            if_branch,
            else_branch,
        } => {
            let (node, env) = eval(*condition, env)?;
            match node {
                Value::Bool(b) => {
                    if b {
                        return eval(*if_branch, env);
                    } else {
                        if let Some(eb) = else_branch {
                            return eval(*eb, env);
                        }
                    }
                    return Ok((Value::NONE, env));
                }
                v => {
                    return Err((
                        EvalError::TypeError(format!("Expected a Bool here not `{}`", v)),
                        env,
                    ))
                }
            }
        }
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
                    Operator::Or => Err((
                        EvalError::MismatchedType(format!(
                            "`{}` or `{}` expected `BOOL` found `Int`",
                            n1, n2
                        )),
                        env,
                    )),
                    Operator::And => Err((
                        EvalError::MismatchedType(format!(
                            "`{}` and `{}` expected `BOOL` found `Int`",
                            n1, n2
                        )),
                        env,
                    )),
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
                    Operator::Or => Err((
                        EvalError::MismatchedType(format!(
                            "`{}` or `{}` expected `BOOL` found `Float`",
                            n1, n2
                        )),
                        env,
                    )),
                    Operator::And => Err((
                        EvalError::MismatchedType(format!(
                            "`{}` and `{}` expected `BOOL` found `Float`",
                            n1, n2
                        )),
                        env,
                    )),
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
                    Operator::Or => Ok((Value::Bool(b1 || b2), env)),
                    Operator::And => Ok((Value::Bool(b1 && b2), env)),
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
        block @ Node::Block(_) => Ok((execute_block(block, Environment::new())?.0, env)),
    }
}

fn execute_block(node: Node, mut block_env: Environment) -> EvalResult {
    if let Node::Block(expressions) = node {
        // println!("BLOCK expressions: {:?}", expressions);
        // println!("BLOCK block_env: {:?}", block_env);
        let mut inner = Value::NONE;
        for exp in expressions {
            let (i, e) = eval(*exp, block_env)?;
            block_env = e;
            inner = i;
        }
        return Ok((inner, block_env));
    }
    Err((
        EvalError::TypeError("Never Will Fail HERE. But if so I am in execute_block".into()),
        block_env,
    ))
}

#[test]
fn execute_block_test() {
    let mut env = Environment::new();
    let func = Node::Variable {
        ident: "add".into(),
        param: vec!["x".into(), "y".into()],
        block: Box::new(Node::BinaryExpr {
            op: Operator::Plus,
            rhs: Box::new(Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Ident {
                    ident: "x".into(),
                    args: vec![],
                }),
            }),
            lhs: Box::new(Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Ident {
                    ident: "y".into(),
                    args: vec![],
                }),
            }),
        }),
    };

    env.insert("add".to_string(), func);
    let func_call = Node::Ident {
        ident: "add".into(),

        args: vec![
            Box::new(Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(1)),
            }),
            Box::new(Node::UnaryExpr {
                op: Operator::Plus,
                child: Box::new(Node::Int(2)),
            }),
        ],
    };
    // eprintln!("{:#?}", env);
    // eprintln!("----");
    // eprintln!("{:#?}", func_call);
    let value = eval(func_call, env.clone());
    assert_eq!(value, Ok((Value::Int(3), env)));
}
