/*
 * parser/declaration/mod.rs
 *
 * Handles all Declarations in Grammer.
 */
use super::{function, statement, Expr};

use nom::{
    branch::alt,
    error::{context, VerboseError},
    IResult,
};

// ❌   declaration  → funDecl | statement ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Declaration");
    }
    context(
        "Declarations Parser",
        alt((function::parser, statement::parser)),
    )(input)
}
