use thiserror::Error;

pub type LuaResult<T> = Result<T, LuaError>;

#[derive(Error, Debug)]
pub enum LuaError {
    #[error("variable already declared in local scope")]
    VariableAlreadyDeclared,
}
