use crate::error::{Error, ErrorKind};
use std::fmt;

pub type Input = String;
pub type InputStream = (Input, Option<Error<Input>>);
pub type ParseResult<Output> = Result<(InputStream, Output), InputStream>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: InputStream) -> ParseResult<Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: fmt::Debug + 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn map_err<F>(self, map_err_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: fmt::Debug + 'a,
        F: Fn(InputStream) -> InputStream + 'a,
    {
        BoxedParser::new(map_err(self, map_err_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
    fn dbg(self, msg: &'a str) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: fmt::Debug + 'a,
    {
        BoxedParser::new(dbg_name(self, msg))
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(InputStream) -> ParseResult<Output>,
{
    fn parse(&self, input: InputStream) -> ParseResult<Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> fmt::Debug for BoxedParser<'a, Output> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BoxedParser")
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: InputStream) -> ParseResult<Output> {
        self.parser.parse(input)
    }
}

pub(crate) fn tag<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |(input, _error): InputStream| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok(((input[expected.len()..].into(), None), ())),
        _ => Err((
            input.clone(),
            Some(Error::new(input, ErrorKind::Tag(expected.into()))),
        )),
    }
}

#[test]
fn tag_parser() {
    let parse_joe = tag("Hello Joe!");
    assert_eq!(
        Ok((("".into(), None), ())),
        parse_joe.parse(("Hello Joe!".into(), None))
    );
    assert_eq!(
        Err((
            "Hello!".into(),
            Some(Error::new(
                "Hello!".into(),
                ErrorKind::Tag("Hello Joe!".into())
            ))
        )),
        parse_joe.parse(("Hello!".into(), None))
    );
    assert_eq!(
        Ok((("".into(), None), ())),
        tag("\n").parse(("\n".into(), None))
    );
    assert_eq!(
        Ok((("=".into(), None), ())),
        tag(">").parse((">=".into(), None))
    );
}

pub(crate) fn identifier<'a>((input, _error): InputStream) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();
    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err((input.clone(), Some(Error::new(input, ErrorKind::Ident)))),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '_' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok(((input[next_index..].into(), None), matched))
}

#[test]
fn identifier_parser() {
    assert_eq!(
        Ok((("".into(), None), "i_am_an_identifier".into())),
        identifier(("i_am_an_identifier".into(), None))
    );
    assert_eq!(
        Ok(((" entirely an identifier".into(), None), "not".into())),
        identifier(("not entirely an identifier".into(), None))
    );
    assert_eq!(
        Ok((("two".into(), None), "one".into())),
        trim(identifier).parse(("one two".into(), None))
    );
    let input: String = "!not at all an identifier".into();
    assert_eq!(
        Err((
            input.clone(),
            Some(Error::new(input.clone(), ErrorKind::Ident))
        )),
        identifier((input, None))
    );
}

pub(crate) fn number((input, error): InputStream) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_numeric() => matched.push(next),
        Some(next) if !next.is_numeric() => {
            return Err((input.clone(), Some(Error::new(input, ErrorKind::Float))));
        }
        _ => return Err((input, error)),
    }

    while let Some(next) = chars.next() {
        if next.is_numeric() || (next == '.' && !matched.contains('.')) {
            matched.push(next);
        } else {
            break;
        }
    }
    Ok(((input[matched.len()..].into(), None), matched))
}

#[test]
fn number_parser() {
    assert_eq!(
        Ok((("".into(), None), "123.321".into())),
        number(("123.321".into(), None))
    );
}

pub(crate) fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

#[test]
fn pair_combinator() {
    // let tag_opener = pair(tag("<".into()), identifier);
    // assert_eq!(
    //     Ok(("/>".into(), ((), "my-first-element".to_string()))),
    //     tag_opener.parse("<my-first-element/>".into())
    // );
    // assert_eq!(
    //     Err(ParseInput::from("oops").with(Error::FailureToMatchLitral("<"))),
    //     tag_opener.parse("oops".into())
    // );
    // assert_eq!(
    //     Err(ParseInput::from("!oops")),
    //     tag_opener.parse("<!oops".into())
    // );
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub(crate) fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

pub(crate) fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

#[test]
fn right_combinator() {
    // let tag_opener = right(tag("<"), identifier);
    // assert_eq!(
    //     Ok(("/>".into(), "my-first-element".to_string())),
    //     tag_opener.parse("<my-first-element/>".into())
    // );
    // assert_eq!(
    //     Err(ParseInput::from("oops").with(Error::FailureToMatchLitral("<"))),
    //     tag_opener.parse("oops".into())
    // );
    // assert_eq!(Err("!oops".into()), tag_opener.parse("<!oops".into()));
}

#[allow(dead_code)]
pub(crate) fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input: InputStream| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

#[test]
fn one_or_more_combinator() {
    // let parser = one_or_more(tag("ha"));
    // assert_eq!(
    //     Ok(("".into(), vec![(), (), ()])),
    //     parser.parse("hahaha".into())
    // );
    // assert_eq!(Err("ahah".into()), parser.parse("ahah".into()));
    // assert_eq!(Err("".into()), parser.parse("".into()));
}

pub(crate) fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input: InputStream| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

#[test]
fn zero_or_more_combinator() {
    // let parser = zero_or_more(tag("ha"));
    // assert_eq!(
    //     Ok(("".into(), vec![(), (), ()])),
    //     parser.parse("hahaha".into())
    // );
    // assert_eq!(Ok(("ahah".into(), vec![])), parser.parse("ahah".into()));
    // assert_eq!(Ok(("".into(), vec![])), parser.parse("".into()));
}

pub(crate) fn any_char<'a>((input, _error): InputStream) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok(((input[next.len_utf8()..].into(), None), next)),
        _ => Err((input.clone(), Some(Error::new(input, ErrorKind::AnyChar)))),
    }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input: InputStream| {
        if let Ok((next_input, value)) = parser.parse(input.clone()) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');
    assert_eq!(
        Ok((("mg".into(), None), 'o')),
        parser.parse(("omg".into(), None))
    );
}

pub(crate) fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

#[allow(dead_code)]
pub(crate) fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

pub(crate) fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

pub(crate) fn quoted_string<'a>() -> impl Parser<'a, String> {
    right(
        tag("\""),
        left(zero_or_more(any_char.pred(|c| *c != '"')), tag("\"")),
    )
    .map(|chars| chars.into_iter().collect())
}

#[test]
pub fn quoted_string_parser() {
    assert_eq!(
        Ok((("".into(), None), "Hello Joe!".to_string())),
        quoted_string().parse(("\"Hello Joe!\"".into(), None))
    );
}

pub(crate) fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input: InputStream| match parser1.parse(input.clone()) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

#[test]
pub fn either_combiantor() {
    assert_eq!(
        Ok(((" \"two\"".into(), None), "2".into())),
        either(number, quoted_string()).parse(("2 \"two\"".into(), None))
    );
    assert_eq!(
        Ok(((" 2".into(), None), "two".into())),
        either(number, quoted_string()).parse(("\"two\" 2".into(), None))
    );
    assert_eq!(
        Ok((("".into(), None), ())),
        either(tag("hey"), either(tag("there"), tag("one"))).parse(("one".into(), None))
    );
}

pub fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

pub(crate) fn trim<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    right(space0(), left(parser, space0()))
}

fn map_err<'a, P, F, A>(parser: P, map_err_fn: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(InputStream) -> InputStream + 'a,
{
    move |input| {
        parser
            .parse(input)
            .map_err(|next_input| map_err_fn(next_input))
    }
}

pub(crate) fn dbg_name<'a, P, O>(parser: P, msg: &'a str) -> impl Parser<'a, O>
where
    O: fmt::Debug + 'a,
    P: Parser<'a, O>,
{
    move |input| {
        eprintln!("----START--{}----", msg);
        let result = parser.parse(input);
        match &result {
            Ok(((next_input, error), output)) => {
                eprintln!("next_input: {:?}", next_input);
                eprintln!("output: {:?}", output);
                eprintln!("error: {:?}", error);
            }
            Err((input, error)) => {
                eprintln!("input: {:?}", input);
                eprintln!("error: {:?}", &error);
            }
        }
        eprintln!("----End----{}----", msg);
        result
    }
}
