/*
 * unary/mod.rs
 *
 * This file Handles the Unary Grammer
 */

use super::{common, factor, Expr, Operator};

use nom::{
    branch::alt,
    error::{context, VerboseError},
    multi::many1,
    sequence::pair,
    IResult,
};

mod term_op;

// term           → factor ( ( "-" | "+" ) factor )* ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Term.");
    }
    context("Term Parser", alt((term_parser, factor::parser)))(input)
}

#[test]
fn term_parse() {
    use super::Constant;
    let data = " 1 + 2 ";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Binary {
                op: Operator::Plus,
                lhs: Box::new(Constant::Int(1).into()),
                rhs: Box::new(Constant::Int(2).into()),
            }
        ))
    );

    let data = "// lkjdsfaklsfjkjasdflkjdafs \n!true";
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

#[test]
fn term_parser_mult_depth() {
    use super::Constant;
    let data = "1 + 1 + 1 + 1";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Binary {
                op: Operator::Plus,
                lhs: Box::new(Expr::Binary {
                    op: Operator::Plus,
                    lhs: Box::new(Expr::Binary {
                        op: Operator::Plus,
                        lhs: Box::new(Expr::Constant(Constant::Int(1))),
                        rhs: Box::new(Expr::Constant(Constant::Int(1)))
                    }),
                    rhs: Box::new(Expr::Constant(Constant::Int(1)))
                }),
                rhs: Box::new(Expr::Constant(Constant::Int(1)))
            }
        ))
    );
}

pub fn term_parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, lhs) = factor::parser(input)?;
    let (i, op) = term_op::parser(i)?;
    let (i, rhs) = factor::parser(i)?;
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    let expr = Expr::Binary { op, lhs, rhs };
    match many1(pair(term_op::parser, factor::parser))(i) {
        Ok((i, vec)) => Ok((
            i,
            vec.iter().fold(expr, |lhs, (op, rhs)| Expr::Binary {
                op: *op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs.clone()),
            }),
        )),
        Err(_) => Ok((i, expr)),
    }
}
