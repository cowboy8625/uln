/*
 * parser/logic/and.rs
 *
 * Handles all 'and' in Grammer.
 */
use super::{common::trim, equality, Expr};

use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{context, VerboseError},
    multi::many1,
    sequence::preceded,
    IResult,
};

// ✅ logic_and    → equality ( "and" equality )* ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for AND");
    }
    let (i, lhs) = equality::parser(input)?;
    match and_word_parser(i) {
        Ok((i, _)) => {
            let (i, rhs) = equality::parser(i)?;
            let expr = Expr::And(Box::new(lhs), Box::new(rhs));
            match many1(preceded(tag("and"), equality::parser))(i) {
                Ok((i, vec)) => Ok((
                    i,
                    vec.iter().fold(expr, |lhs, rhs| {
                        Expr::And(Box::new(lhs), Box::new(rhs.clone()))
                    }),
                )),
                Err(_) => Ok((i, expr)),
            }
        }
        Err(_) => Ok((i, lhs)),
    }
}

fn and_word_parser(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    context("not and keyword", map(trim(tag("and")), |_| ()))(input)
}
