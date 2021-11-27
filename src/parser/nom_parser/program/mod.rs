/*
 * program/mod.rs
 */
use super::{declaration, Expr};

use nom::{error::VerboseError, multi::many0, IResult};

// program      → declaration* EOF ;
pub fn parser(input: &str) -> IResult<&str, Vec<Expr>, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Starting Parse");
    }
    many0(declaration::parser)(input)
}
