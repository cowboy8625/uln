/*
 * conditional/mod.rs
 */
use super::{common::trim, expression, statement, Expr};

use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

// ifStmt → "if" expression "then" statement ( "else" statement )? ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking for Conditional");
    }
    alt((if_else_stmt, if_stmt, expression::parser))(input)
}

fn if_stmt(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = trim(tag("if"))(input)?;
    let (i, expr) = expression::parser(i)?;
    let (i, _) = trim(tag("then"))(i)?;
    let (i, statement) = statement::parser(i)?;
    Ok((i, Expr::If(Box::new(expr), Box::new(statement))))
}

fn if_else_stmt(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = trim(tag("if"))(input)?;
    let (i, expr) = expression::parser(i)?;
    let (i, _) = trim(tag("then"))(i)?;
    let (i, statement) = statement::parser(i)?;
    let (i, _) = trim(tag("else"))(i)?;
    let (i, else_statement) = statement::parser(i)?;
    Ok((
        i,
        Expr::IfElse(
            Box::new(expr),
            Box::new(statement),
            Box::new(else_statement),
        ),
    ))
}
