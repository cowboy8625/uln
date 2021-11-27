use super::{common::trim, expression, Builtin, Expr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    error::{context, VerboseError},
    IResult,
};

// ❌   printStmt    → "print" expression
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Print Statement");
    }
    context("Print Statement", alt((print_line_parser, print_parser)))(input)
}

pub fn print_line_parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = trim(tag("println"))(input)?;
    let (i, expr) = expression::parser(i)?;
    Ok((i, Builtin::PrintLn(Box::new(expr)).into()))
}

pub fn print_parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = trim(tag("print"))(input)?;
    let (i, expr) = expression::parser(i)?;
    Ok((i, Builtin::Print(Box::new(expr)).into()))
}
