/*
 * parser/parameter/mod.rs
 *
 * Handles all Function Parameter in Grammer.
 */

use super::common::trim;
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

// ❌   paramenters  → IDENTIFIER ( IDENTIFIER )* ;
pub fn parser(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Parameter's");
    }
    context("Declarations Parser", trim(many0(identifier)))(input)
}

// TODO: FIXME: only use the IDENTIFIER parser from the primary module.
pub fn identifier(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "Not a paramenter IDENTIFIER",
        map(
            trim(recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            ))),
            |keyword: &str| keyword.into(),
        ),
    )(input)
}
