use super::{common::trim, Operator};
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    error::{context, VerboseError},
    IResult,
};

// Parser looks for a >, >=, <, <=.
pub(crate) fn parser(input: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    let (i, front) = context(
        "Not a Comparison Operator of `>`, `>=`, `<`, `<=`.",
        trim(one_of("><")),
    )(input)?;

    fn equals_parse(i: &str) -> IResult<&str, &str> {
        trim(tag("="))(i)
    }

    match equals_parse(i) {
        Ok((i, _)) => Ok((
            i,
            match front {
                '>' => Operator::GreaterThenEqual,
                '<' => Operator::LessThenEqual,
                _ => unreachable!(),
            },
        )),
        Err(_) => Ok((
            i,
            match front {
                '>' => Operator::GreaterThen,
                '<' => Operator::LessThen,
                _ => unreachable!(),
            },
        )),
    }
}

#[test]
fn combinator_op_parser() {
    assert_eq!(parser(" > "), Ok(("", Operator::GreaterThen,)));
    assert_eq!(parser(" < "), Ok(("", Operator::LessThen,)));
    assert_eq!(parser(" >= "), Ok(("", Operator::GreaterThenEqual,)));
    assert_eq!(parser(" <= "), Ok(("", Operator::LessThenEqual,)));
}
