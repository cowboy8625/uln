/*
 * expression/mod.rs
 */
use super::{logic, Expr};

use nom::{error::VerboseError, IResult};

// ❌✅ expression   → logic_or ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Equality");
    }
    logic::or::parser(input)
}
