use super::ctx::Ctx;
use super::super::Error;
use super::super::value::Value;

pub trait Eval {
    fn eval(&self, ctx: &mut Ctx) -> Result<Value, Error>;
}
