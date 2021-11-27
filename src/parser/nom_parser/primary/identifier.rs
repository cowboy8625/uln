use super::{trim, Constant};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::map,
    combinator::recognize,
    error::{context, VerboseError},
    multi::many0,
    sequence::pair,
    IResult,
};

// IDENTIFIER
pub fn parser(input: &str) -> IResult<&str, Constant, VerboseError<&str>> {
    context(
        "Not a IDENTIFIER",
        map(
            trim(recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            ))),
            |keyword: &str| Constant::Keyword(keyword.into()),
        ),
    )(input)
}

#[test]
fn identifier_parse() {
    assert_eq!(
        parser(" func_name "),
        Ok(("", Constant::Keyword("func_name".into())))
    );
    assert_eq!(
        parser(" func_name          = 1"),
        Ok(("= 1", Constant::Keyword("func_name".into())))
    );
}
