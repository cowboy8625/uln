use super::{trim, Constant};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::{context, VerboseError},
    IResult,
};

// BOOLEAN
pub fn parser(input: &str) -> IResult<&str, Constant, VerboseError<&str>> {
    context(
        "Not a Bool",
        alt((
            map(trim(tag("true")), |_| Constant::Boolean(true)),
            map(trim(tag("false")), |_| Constant::Boolean(false)),
        )),
    )(input)
}

#[test]
fn boolean_parser() {
    assert_eq!(parser(" true"), Ok(("", Constant::Boolean(true))));
    assert_eq!(parser("false"), Ok(("", Constant::Boolean(false))));
}
