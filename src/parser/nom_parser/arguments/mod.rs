/*
 * arguments/mod.rs
 *
 * Handles Argument's for Calling DataStruct's Grammer
 */
use super::{expression, Expr};

use nom::{combinator::map, error::VerboseError, multi::many0, IResult};

// arguments → expression ( expression)* ;
pub fn parser(input: &str) -> IResult<&str, Vec<Box<Expr>>, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for arguments");
    }
    let (i, args) = many0(map(expression::parser, Box::new))(input)?;
    Ok((i, args))
}
