use super::{common::trim, Operator};
use nom::{
    character::complete::one_of,
    combinator::map,
    error::{context, VerboseError},
    IResult,
};

// Parser looks for a * or /.
pub(crate) fn parser(input: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    context(
        "Not a * or / Operator",
        map(trim(one_of("*/")), |c: char| match c {
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            _ => unreachable!(),
        }),
    )(input)
}

#[test]
fn factor_op_parser() {
    assert_eq!(parser(" * "), Ok(("", Operator::Multiply,)));
    assert_eq!(parser(" / "), Ok(("", Operator::Divide,)));
}
