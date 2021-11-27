/*
 * unary/mod.rs
 *
 * This parser Handles turning &str into Operator Type
 */
use super::{call, common, primary, Expr, Operator};
use nom::{
    branch::alt,
    combinator::map,
    error::{context, VerboseError},
    multi::many1,
    IResult,
};

mod unary_op;

// unary          → ( "!" | "-" ) unary | call ;
pub(crate) fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Term.");
    }
    context(
        "Unary Parser",
        alt((unary_const_or_unary_parser, call::parser)),
    )(input)
}

#[test]
fn unary_parser() {
    use super::{Constant, Operator};
    let data = "-123432";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Unary {
                op: Operator::Minus,
                child: Box::new(Constant::Int(123432).into())
            }
        ))
    );

    let data = "!true";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Unary {
                op: Operator::Bang,
                child: Box::new(Constant::Boolean(true).into())
            }
        ))
    );
}

fn many1_unary_op_parser(input: &str) -> IResult<&str, Option<Operator>, VerboseError<&str>> {
    map(many1(unary_op::parser), |vec_op| {
        if vec_op.len() % 2 == 0 {
            None
        } else {
            vec_op.first().map(Clone::clone)
        }
    })(input)
}

fn unary_const_or_unary_parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, some_op) = many1_unary_op_parser(input)?;
    let (i, child) = primary::parser(i)?;
    let expr = if let Some(op) = some_op {
        Expr::Unary {
            op,
            child: Box::new(child),
        }
    } else {
        child
    };
    Ok((i, expr))
}

#[test]
fn fail_unary_test() {
    use super::Constant;
    assert!(unary_const_or_unary_parser("1").ok().is_none());
    assert_eq!(
        unary_const_or_unary_parser("-1"),
        Ok((
            "",
            Expr::Unary {
                op: Operator::Minus,
                child: Box::new(Expr::Constant(Constant::Int(1)))
            }
        ))
    );
    assert_eq!(
        alt((unary_const_or_unary_parser, primary::parser))("1"),
        Ok(("", Expr::Constant(Constant::Int(1))))
    );
}
