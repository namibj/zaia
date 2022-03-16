pub mod ctx;
pub mod eval;

use super::value::{Table, Value};
use super::gc::Heap;
use ctx::Ctx;
use eval::Eval;
use super::Error;

pub struct VM {
    global: Table,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        VM {
            global: Table::new(heap),
        }
    }

    pub fn eval<T>(&mut self, item: &T) -> Result<Value,Error> where T:Eval {
        let mut ctx = Ctx::new(&mut self.global);
        item.eval(&mut ctx)
    }
}
