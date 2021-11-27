use super::{
    error::{Error, ErrorKind},
    eval_expr, Environment, IResult,
};
use crate::parser::{Constant, Expr};

pub fn eval_if(condision: Expr, statement: Expr, env: Environment) -> IResult {
    let (result, e) = eval_expr(condision, env)?;
    if let Constant::Boolean(boolean) = result {
        if boolean {
            return eval_expr(statement, e);
        }
        return return_error();
    }
    type_error(result)
}

pub fn eval_if_else(
    condision: Expr,
    statement: Expr,
    else_statement: Expr,
    env: Environment,
) -> IResult {
    let (result, e) = eval_expr(condision, env)?;
    if let Constant::Boolean(boolean) = result {
        if boolean {
            return eval_expr(statement, e);
        } else {
            return eval_expr(else_statement, e);
        }
    }
    type_error(result)
}

fn type_error(condision: Constant) -> IResult {
    Err(Error::new(
        &format!(
            "Can only use Expressions that evaluate to a Boolean.
{} != Boolean",
            condision.name()
        ),
        ErrorKind::TypeError,
    ))
}

fn return_error() -> IResult {
    Err(Error::new(
        "Must return something.",
        ErrorKind::ReturningNothing,
    ))
}
