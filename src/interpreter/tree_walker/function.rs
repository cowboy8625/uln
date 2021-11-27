use super::{
    error::{Error, ErrorKind},
    Environment, IResult,
};
use crate::interpreter::DataStruct;
use crate::parser::{Constant, Expr};
pub fn eval(name: String, param: Vec<String>, stmt: Expr, mut env: Environment) -> IResult {
    if env.contains_key(&name) {
        return mutation_error(&name);
    }

    env.insert(name.clone(), DataStruct::Function(name, param, stmt));
    Ok((Constant::Int(0), env))
}

fn mutation_error(func_name: &str) -> IResult {
    Err(Error::new(
        &format!(
            "Function {} already exist!  Mutations is not allowed.",
            func_name
        ),
        ErrorKind::MutationError,
    ))
}
