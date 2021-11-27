/*
 * term/mod.rs
 *
 * This parser Handles turning &str into Operator Type
 */
use super::{common::trim, Operator};
use nom::{
    character::complete::one_of,
    combinator::map,
    error::{context, VerboseError},
    IResult,
};

// Parser looks for a + or -.
pub(crate) fn parser(input: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    context(
        "Not a + or - Operator",
        map(trim(one_of("+-")), |c: char| match c {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            _ => unreachable!(),
        }),
    )(input)
}

#[test]
fn unary_minus_parser() {
    assert_eq!(parser(" + "), Ok(("", Operator::Plus,)));
    assert_eq!(parser(" - "), Ok(("", Operator::Minus,)));
}
