mod tree_walker;
// mod vm;

use crate::parser::{Constant, Expr};

/// Environment
use std::collections::HashMap;
pub type Environment = HashMap<String, DataStruct>;

pub fn interpreter(expressions: Vec<Expr>) -> Vec<Constant> {
    let mut environment = Environment::new();
    let mut constants = Vec::new();
    for expr in expressions {
        match tree_walker::eval_expr(expr, environment.clone()) {
            Ok((con, e)) => {
                constants.push(con);
                environment = e;
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
    constants
}
pub fn interpreter_expr(expr: Expr, mut env: Environment) -> (Vec<Constant>, Environment) {
    let mut constants = Vec::new();
    match tree_walker::eval_expr(expr, env.clone()) {
        Ok((con, e)) => {
            constants.push(con);
            env = e
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }
    (constants, env)
}

#[derive(Debug, Clone)]
pub enum DataStruct {
    Function(String, Vec<String>, Expr),
    Argument(Expr),
}
