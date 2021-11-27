use super::{common, unary, Expr, Operator};

use nom::{
    branch::alt,
    error::{context, VerboseError},
    multi::many1,
    sequence::pair,
    IResult,
};

mod factor_op;

// factor         → unary ( ( "/" | "*" ) unary )* ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Factor");
    }
    context("Factor Parser", alt((factor_parser, unary::parser)))(input)
}

#[test]
fn factor_parse() {
    use super::node::{Constant, Operator};
    let data = " 1 * 2 ";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Binary {
                op: Operator::Multiply,
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
fn factor_parser_mult_depth() {
    use super::node::{Constant, Operator};
    let data = "5 * 2 * 2 / 2";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Binary {
                op: Operator::Divide,
                lhs: Box::new(Expr::Binary {
                    op: Operator::Multiply,
                    lhs: Box::new(Expr::Binary {
                        op: Operator::Multiply,
                        lhs: Box::new(Expr::Constant(Constant::Int(5))),
                        rhs: Box::new(Expr::Constant(Constant::Int(2)))
                    }),
                    rhs: Box::new(Expr::Constant(Constant::Int(2)))
                }),
                rhs: Box::new(Expr::Constant(Constant::Int(2)))
            }
        ))
    );
}

pub fn factor_parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, lhs) = unary::parser(input)?;
    let (i, op) = factor_op::parser(i)?;
    let (i, rhs) = unary::parser(i)?;
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    let expr = Expr::Binary { op, lhs, rhs };
    match many1(pair(factor_op::parser, unary::parser))(i) {
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
