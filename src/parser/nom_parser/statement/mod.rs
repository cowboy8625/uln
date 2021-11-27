/*
 * statement/mod.rs
 */
use super::{block, conditional, expression, print, Expr};

use nom::{branch::alt, error::VerboseError, IResult};

// statement    → printStmt | expression | ifStmt | returnStmt | block ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for a Statement");
    }
    alt((
        print::parser,
        conditional::parser,
        expression::parser,
        block::parser,
    ))(input)
}
