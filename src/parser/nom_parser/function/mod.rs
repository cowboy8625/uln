/*
 * parser/function/mod.rs
 *
 * Handles all Declarations in Grammer.
 */
use super::{common::trim, parameters, statement, Expr};

use nom::{
    bytes::complete::tag,
    bytes::complete::take_till,
    combinator::map,
    error::{context, VerboseError},
    IResult,
};

// ❌   function      → IDENTIFIER parameter? "=" statement "\n" ;
pub fn parser(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    if cfg!(feature = "announce") {
        println!("Checking Function");
    }
    let (i, func) = trim(take_till(|c| c == '\n'))(input)?;
    Ok((i, function(func)?.1))
}

// ❌   function      → IDENTIFIER parameter? "=" statement "\n" ;
pub fn function(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    // TODO: Make these primary calls
    let (i, ident) = parameters::identifier(input)?;
    let (i, param) = parameters::parser(i)?;
    let (i, _) = equals(i)?;
    let (i, stmt) = statement::parser(i)?;
    Ok((i, Expr::Function(ident, param, Box::new(stmt))))
}

pub fn equals(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    context("Function Equals", map(trim(tag("=")), |_| ()))(input)
}

#[test]
fn funciton_line_test() {
    use super::{Constant, Expr, Operator};
    let data = "add x y = x + y\n1";
    assert_eq!(
        parser(data),
        Ok((
            "1",
            Expr::Function(
                "add".into(),
                vec!["x".to_string(), "y".to_string()],
                Box::new(Expr::Binary {
                    op: Operator::Plus,
                    lhs: Box::new(Expr::Call(
                        Box::new(Expr::Constant(Constant::Keyword("x".into()))),
                        vec![]
                    )),
                    rhs: Box::new(Expr::Call(
                        Box::new(Expr::Constant(Constant::Keyword("y".into()))),
                        vec![]
                    ))
                })
            )
        ))
    );
}

#[test]
fn funciton_no_args() {
    use super::{Constant, Expr};
    let data = "name = \"Cowboy\"\n1";
    assert_eq!(
        parser(data),
        Ok((
            "1",
            Expr::Function(
                "name".into(),
                Vec::new(),
                Box::new(Constant::String("Cowboy".into()).into())
            )
        ))
    );
}

#[test]
fn funciton_after_function() {
    use super::{Constant, Expr};

    let data = "name = \"Cowboy\"\nnum = 1\n";
    let (i, left) = parser(data).unwrap();
    assert_eq!(
        left,
        Expr::Function(
            "name".into(),
            Vec::new(),
            Box::new(Constant::String("Cowboy".into()).into())
        )
    );
    assert_eq!(i, "num = 1\n");
    let (i, left) = parser(i).unwrap();
    assert_eq!(
        left,
        Expr::Function("num".into(), Vec::new(), Box::new(Constant::Int(1).into()))
    );
    assert_eq!(i, "");
}
