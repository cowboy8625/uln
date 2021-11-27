use super::eval_expr;
use super::{Environment, IResult};
use crate::parser::{Builtin, Constant};

pub fn eval(constant: Constant, env1: Environment) -> IResult {
    match constant {
        Constant::Builtin(builtin) => match builtin {
            Builtin::Print(expr) => {
                let (constant, env2) = eval_expr(*expr, env1)?;
                print!("{}", constant);
                Ok((constant, env2))
            }
            Builtin::PrintLn(expr) => {
                let (constant, env2) = eval_expr(*expr, env1)?;
                println!("{}", constant);
                Ok((constant, env2))
            }
        },
        _ => Ok((constant, env1)),
    }
}
