use super::{
    error::{Error, ErrorKind},
    eval_expr, Environment, IResult,
};
use crate::interpreter::DataStruct;
use crate::parser::{Constant, Expr};

pub fn eval(ident: Expr, args: Vec<Box<Expr>>, env1: Environment) -> IResult {
    let (ident, env2) = eval_expr(ident, env1)?;
    if let Constant::Keyword(name) = ident {
        match env2.get(&name) {
            Some(data_struct) => match data_struct {
                DataStruct::Function(name, param, stmt) => {
                    if param.len() != args.len() {
                        return Err(Error::new(
                        &format!(
                            "Miss matched parameter count for Function <{}> Expected {} but found {}",
                            name, param.len(), args.len()
                            ),
                        ErrorKind::MisMatchedPramas,
                    ));
                    }
                    let mut block_env = Environment::new();
                    for (p, expr) in param.iter().zip(args) {
                        block_env.insert(p.clone(), DataStruct::Argument(*expr));
                    }
                    let (result, _) = eval_expr(stmt.clone(), block_env)?;
                    return Ok((result, env2));
                }
                DataStruct::Argument(arg) => return eval_expr(arg.clone(), env2),
            },
            None => {
                return Err(Error::new(
                    &format!("{} is not Defined", name),
                    ErrorKind::Undefined,
                ))
            }
        }
    }
    unreachable!()
}
