use super::{decimal, Constant};
use nom::{
    branch::alt,
    character::complete::char,
    character::complete::one_of,
    combinator::map,
    combinator::{opt, recognize},
    error::{context, VerboseError},
    sequence::preceded,
    sequence::tuple,
    IResult,
};
// FLOAT
pub fn parser(input: &str) -> IResult<&str, Constant, VerboseError<&str>> {
    context(
        "Not a Float",
        map(float, |float_str: &str| {
            Constant::Float(float_str.parse::<f64>().unwrap())
        }),
    )(input)
}

fn float(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "Float",
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal::parser,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal::parser))),
            ))), // Case two: 42e42 and 42.42e42
            recognize(tuple((
                decimal::parser,
                opt(preceded(char('.'), decimal::parser)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal::parser,
            ))), // Case three: 42. and 42.42
            recognize(tuple((decimal::parser, char('.'), opt(decimal::parser)))),
        )),
    )(input)
}

#[test]
fn parser_float() {
    assert_eq!(parser("1.23"), Ok(("", Constant::Float(1.23))));
    assert_eq!(parser("1."), Ok(("", Constant::Float(1.))));
    assert_eq!(parser(".23"), Ok(("", Constant::Float(0.23))));
}
