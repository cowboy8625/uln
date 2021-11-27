use nom::{
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    error::ParseError,
    multi::many0,
    sequence::delimited,
    sequence::preceded,
    sequence::tuple,
    IResult,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, and language comments starting with '//' returning the output of `inner`.
pub(crate) fn trim<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, comment(inner), multispace0)
}

fn comment<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        many0(tuple((tag("//"), take_until("\n"), tag("\n")))),
        inner,
        many0(tuple((tag("//"), take_until("\n"), tag("\n")))),
    )
}

fn _test_trim(i: &str) -> IResult<&str, &str> {
    trim(tag("tag"))(i)
}

#[test]
fn trim_parser() {
    let data = "    // Hey there \ntag";
    assert_eq!(_test_trim(data), Ok(("", "tag")));
}
