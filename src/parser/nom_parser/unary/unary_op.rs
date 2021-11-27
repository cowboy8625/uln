use super::common::trim;
use super::Operator;
use nom::{
    character::complete::one_of,
    combinator::map,
    error::{context, VerboseError},
    IResult,
};

// Parser looks for a - or ! and returns a Operator enum.
pub(crate) fn parser(input: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Unary Op");
    }
    context(
        "Not a Unary Operator of Minus `-` or `!`.",
        map(trim(one_of("-!")), |c: char| match c {
            '-' => Operator::Minus,
            '!' => Operator::Bang,
            _ => unreachable!(),
        }),
    )(input)
}

#[test]
fn unary_op_parser() {
    assert_eq!(parser(" ! "), Ok(("", Operator::Bang,)));
    assert_eq!(parser(" - "), Ok(("", Operator::Minus,)));
}
