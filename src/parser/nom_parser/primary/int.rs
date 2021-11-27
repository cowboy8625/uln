use super::{decimal, Constant};
use nom::{
    combinator::map,
    error::{context, VerboseError},
    IResult,
};
// INT
pub fn parser(input: &str) -> IResult<&str, Constant, VerboseError<&str>> {
    context(
        "Not a Int",
        map(decimal::parser, |digit_str: String| {
            digit_str.parse::<i128>().map(Constant::Int).unwrap()
        }),
    )(input)
}

#[test]
fn parse_int() {
    let int = "123";
    assert_eq!(parser(int), Ok(("", Constant::Int(123))));
    let not = "1.123";
    assert_eq!(parser(not), Ok((".123", Constant::Int(1))));
}
