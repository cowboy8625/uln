/*
 * call/mod.rs
 *
 * Handles Calling Grammer
 */
use super::{arguments, primary, Constant, Expr};

use nom::{error::VerboseError, IResult};

// call → primary ( arguments? )* ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for a Call");
    }
    let (i1, ident) = primary::parser(input)?;
    if let Expr::Constant(Constant::Keyword(_)) = ident {
        let (i2, args) = arguments::parser(i1)?;
        return Ok((i2, Expr::Call(Box::new(ident), args)));
    }
    Ok((i1, ident))
}
