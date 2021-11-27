mod binary;
mod call;
mod conditional;
mod constant;
mod error;
mod function;
mod logic;
mod unary;

use super::Environment;
use crate::parser::{Constant, Expr};
use error::{Error, ErrorKind};

type IResult = Result<(Constant, Environment), Error>;

pub fn eval_expr(expr: Expr, mut env: Environment) -> IResult {
    match expr {
        Expr::Constant(constant) => constant::eval(constant, env),
        Expr::Unary { op, child } => {
            let (result, e) = unary::eval(op, *child, env)?;
            env = e;
            Ok((result, env))
        }
        Expr::Binary { op, lhs, rhs } => {
            let (result, e) = binary::eval(op, *lhs, *rhs, env)?;
            env = e;
            Ok((result, env))
        }

        Expr::If(condision, statement) => {
            let (result, e) = conditional::eval_if(*condision, *statement, env)?;
            env = e;
            Ok((result, env))
        }
        Expr::IfElse(condision, stmt, else_stmt) => {
            let (result, e) = conditional::eval_if_else(*condision, *stmt, *else_stmt, env)?;
            env = e;
            Ok((result, env))
        }
        Expr::And(lhs, rhs) => {
            let (result, e) = logic::eval_and(*lhs, *rhs, env)?;
            env = e;
            Ok((result, env))
        }
        Expr::Or(lhs, rhs) => {
            let (result, e) = logic::eval_or(*lhs, *rhs, env)?;
            env = e;
            Ok((result, env))
        }
        Expr::Function(name, param, stmt) => {
            let (result, e) = function::eval(name, param, *stmt, env)?;
            env = e;
            Ok((result, env))
        }
        Expr::Call(keyword, args) => {
            let (result, e) = call::eval(*keyword, args, env)?;
            env = e;
            Ok((result, env))
        }
        expr => Err(Error::new(
            &format!(
                "You have not implemented this {:?} Expression in the Interrupter",
                expr
            ),
            ErrorKind::NotImplemented,
        )),
    }
}
