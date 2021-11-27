use super::{
    error::{Error, ErrorKind},
    eval_expr, Environment, IResult,
};
use crate::parser::{Constant, Expr};
pub fn eval_and(lhs: Expr, rhs: Expr, env1: Environment) -> IResult {
    let (lhsr, env2) = eval_expr(lhs, env1)?;
    let (rhsr, env3) = eval_expr(rhs, env2)?;
    match (lhsr, rhsr) {
        (Constant::Boolean(l), Constant::Boolean(r)) => Ok((Constant::Boolean(l && r), env3)),
        (l, r) => type_error(l, r),
    }
}

pub fn eval_or(lhs: Expr, rhs: Expr, env1: Environment) -> IResult {
    let (lhsr, env2) = eval_expr(lhs, env1)?;
    let (rhsr, env3) = eval_expr(rhs, env2)?;
    match (lhsr, rhsr) {
        (Constant::Boolean(l), Constant::Boolean(r)) => Ok((Constant::Boolean(l || r), env3)),
        (l, r) => type_error(l, r),
    }
}

fn type_error(lhs: Constant, rhs: Constant) -> IResult {
    Err(Error::new(
        &format!(
            "'and' Must be used with a Expressions not a <{}> Type with a <{}> Type",
            lhs.name(),
            rhs.name(),
        ),
        ErrorKind::TypeError,
    ))
}
