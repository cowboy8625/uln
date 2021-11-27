use super::{common::trim, statement, Constant, Expr};
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, error::VerboseError, sequence::delimited,
    IResult,
};

mod boolean;
mod decimal;
mod float;
mod identifier;
mod int;
mod string;

// primary → FLOAT | INT | STRING | "true" | "false" | "(" expression ")" | IDENTIFIER | COMMENT;
pub(crate) fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Primary's");
    }
    alt((
        map(trim(string::parser), Into::into),
        map(trim(float::parser), Into::into),
        map(trim(int::parser), Into::into),
        map(trim(boolean::parser), Into::into),
        map(trim(identifier::parser), Into::into),
        trim(delimited(tag("("), statement::parser, tag(")"))),
    ))(input)
}
