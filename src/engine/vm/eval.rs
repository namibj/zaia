use super::ctx::Ctx;
use super::super::Error;

pub trait Eval {
    fn eval(&mut self, ctx: &mut Ctx) -> Result<(), Error>;
}
