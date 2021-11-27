use super::{common, term, Expr, Operator};

use nom::{
    branch::alt,
    error::{context, VerboseError},
    multi::many1,
    sequence::pair,
    IResult,
};

mod comparison_op;

// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Comparison");
    }
    context("Term Parser", alt((comparison_parser, term::parser)))(input)
}

#[test]
fn comparison_parse() {
    use super::Constant;
    let data = " 1 > 2 ";
    assert_eq!(
        parser(data),
        Ok((
            "",
            Expr::Binary {
                op: Operator::GreaterThen,
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
fn comparison_parser_mult_depth() {
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

pub fn comparison_parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, lhs) = term::parser(input)?;
    let (i, op) = comparison_op::parser(i)?;
    let (i, rhs) = term::parser(i)?;
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    let expr = Expr::Binary { op, lhs, rhs };
    match many1(pair(comparison_op::parser, term::parser))(i) {
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
