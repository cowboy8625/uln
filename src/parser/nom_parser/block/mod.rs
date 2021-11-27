/*
 * parser/block/mod.rs
 *
 * Handles all Block in Grammer.
 */
use super::{common::trim, statement, Expr};

use nom::{
    bytes::complete::tag,
    error::{context, VerboseError},
    sequence::delimited,
    IResult,
};

// ❌   block → "{" declaration* "}"
// ❌   block → "{" statement* "}"
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for block");
    }
    context(
        "Block Parser",
        delimited(trim(tag("{")), statement::parser, trim(tag("}"))),
    )(input)
}
