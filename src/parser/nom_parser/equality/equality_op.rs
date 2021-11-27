/*
 * equality/equality_op.rs
 *
 * This file Handles the Equality Operator's
 */

// use crate::parser::common::trim;
use super::Operator;
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::recognize,
    error::{context, VerboseError},
    sequence::pair,
    IResult,
};

// Parser looks for a ==, !=.
pub(crate) fn parser(input: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    let (i, token) = context(
        "Not a Equality Operator of `==`, `!=`.",
        recognize(pair(one_of("!="), tag("="))),
    )(input)?;
    Ok((
        i,
        match token {
            "==" => Operator::Equality,
            "!=" => Operator::NotEqual,
            _ => unreachable!(),
        },
    ))
}

#[test]
fn equality_op_parse() {
    assert_eq!(parser("=="), Ok(("", Operator::Equality,)));
    assert_eq!(parser("!="), Ok(("", Operator::NotEqual,)));
}
