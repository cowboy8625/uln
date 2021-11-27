use nom::error::VerboseError;
use nom::IResult;
mod arguments;
mod block;
mod call;
mod common;
mod comparison;
mod conditional;
mod declaration;
mod equality;
mod expression;
mod factor;
mod function;
mod logic;
mod parameters;
mod primary;
mod print;
mod program;
mod statement;
mod term;
mod unary;

pub use crate::parser::{Builtin, Constant, Expr, Operator};

pub fn parser(input: &str) -> IResult<&str, Vec<Expr>, VerboseError<&str>> {
    common::trim(program::parser)(input)
}
