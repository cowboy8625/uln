use super::{
    error::{Error, ErrorKind},
    eval_expr, Environment, IResult,
};
use crate::parser::{Constant, Expr, Operator};
pub fn eval(op: Operator, child: Expr, env1: Environment) -> IResult {
    let (child, env2) = eval_expr(child, env1)?;
    match op {
        Operator::Minus => match child {
            Constant::Int(i) => Ok((Constant::Int(-i), env2)),
            Constant::Float(f) => Ok((Constant::Float(-f), env2)),
            c => can_not_prefix_error(op, c),
        },
        Operator::Plus => match child {
            Constant::Int(i) => Ok((Constant::Int(i), env2)),
            Constant::Float(f) => Ok((Constant::Float(f), env2)),
            c => can_not_prefix_error(op, c),
        },
        Operator::Bang => match child {
            Constant::Boolean(b) => Ok((Constant::Boolean(!b), env2)),
            c => can_not_prefix_error(op, c),
        },

        _ => can_not_prefix_error(op, child),
    }
}

fn can_not_prefix_error(op: Operator, constant: Constant) -> IResult {
    Err(Error::new(
        &format!(
            "Can not prefix a <{}> Type with a '{}' {} Operator",
            constant.name(),
            op.symbol(),
            op.name()
        ),
        ErrorKind::PrefixError,
    ))
}
