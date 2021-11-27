use super::{
    error::{Error, ErrorKind},
    eval_expr, Environment, IResult,
};
use crate::parser::{Constant, Expr, Operator};

pub fn eval(op: Operator, lhs: Expr, rhs: Expr, env1: Environment) -> IResult {
    let (lhsr, env2) = eval_expr(lhs, env1)?;
    let (rhsr, env3) = eval_expr(rhs, env2)?;
    match op {
        Operator::Minus => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Int(i1 - i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Float(f1 - f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::Plus => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Int(i1 + i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Float(f1 + f2), env3)),
            (Constant::String(s1), Constant::String(s2)) => Ok((Constant::String(s1 + &s2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::Multiply => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Int(i1 * i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Float(f1 * f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::Divide => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Int(i1 / i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Float(f1 / f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::GreaterThen => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Boolean(i1 > i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Boolean(f1 > f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::LessThen => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Boolean(i1 < i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Boolean(f1 < f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::GreaterThenEqual => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Boolean(i1 >= i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Boolean(f1 >= f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::LessThenEqual => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Boolean(i1 <= i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => Ok((Constant::Boolean(f1 <= f2), env3)),
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::Equality => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Boolean(i1 == i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => {
                Ok((Constant::Boolean((f1 - f2).abs() == f64::EPSILON), env3))
            }
            (Constant::String(s1), Constant::String(s2)) => Ok((Constant::Boolean(s1 == s2), env3)),
            (Constant::Boolean(b1), Constant::Boolean(b2)) => {
                Ok((Constant::Boolean(b1 == b2), env3))
            }
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::NotEqual => match (lhsr, rhsr) {
            (Constant::Int(i1), Constant::Int(i2)) => Ok((Constant::Boolean(i1 != i2), env3)),
            (Constant::Float(f1), Constant::Float(f2)) => {
                Ok((Constant::Boolean((f1 - f2).abs() != f64::EPSILON), env3))
            }
            (Constant::String(s1), Constant::String(s2)) => Ok((Constant::Boolean(s1 != s2), env3)),
            (Constant::Boolean(b1), Constant::Boolean(b2)) => {
                Ok((Constant::Boolean(b1 != b2), env3))
            }
            (t1, t2) => type_error(op, t1, t2),
        },
        Operator::Bang => type_error(op, lhsr, rhsr),
    }
}

fn type_error(op: Operator, lhs: Constant, rhs: Constant) -> IResult {
    Err(Error::new(
        &format!(
            "Can not {op_name} a <{rhs}> Type with a <{lhs}> Type
        Error -> {lhs} {op_symbol} {rhs}",
            op_name = op.name(),
            lhs = lhs.name(),
            rhs = rhs.name(),
            op_symbol = op.symbol(),
        ),
        ErrorKind::TypeError,
    ))
}
