use nom::{
    character::complete::char,
    character::complete::one_of,
    error::{context, VerboseError},
    multi::{fold_many1, many0, many1},
    sequence::terminated,
    IResult,
};
pub fn parser(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "Decimal",
        fold_many1(
            many1(terminated(one_of("0123456789"), many0(char('_')))),
            String::new,
            |acc: String, item| acc + &item.iter().collect::<String>(),
        ),
    )(input)
}

#[test]
fn decimal_parse() {
    assert_eq!(parser("12_3_123"), Ok(("", "123123".into())));
    assert_eq!(parser("123123"), Ok(("", "123123".into())));
}
