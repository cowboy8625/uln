/*
 * parser/logic/or.rs
 *
 * Handles all 'or' in Grammer.
 */
use super::{and, common::trim, Expr};

use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{context, VerboseError},
    multi::many1,
    sequence::preceded,
    IResult,
};

//✅logic_or     → logic_and ( "or" logic_and )* ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for OR");
    }
    let (i, lhs) = and::parser(input)?;
    match and_word_parser(i) {
        Ok((i, _)) => {
            let (i, rhs) = and::parser(i)?;
            let expr = Expr::Or(Box::new(lhs), Box::new(rhs));
            match many1(preceded(and_word_parser, and::parser))(i) {
                Ok((i, vec)) => Ok((
                    i,
                    vec.iter().fold(expr, |lhs, rhs| {
                        Expr::Or(Box::new(lhs), Box::new(rhs.clone()))
                    }),
                )),
                Err(_) => Ok((i, expr)),
            }
        }
        Err(_) => Ok((i, lhs)),
    }
}

fn and_word_parser(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    context("or and keyword", map(trim(tag("or")), |_| ()))(input)
}
