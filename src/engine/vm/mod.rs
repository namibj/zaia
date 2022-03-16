pub mod ctx;
pub mod eval;

use ctx::Ctx;
use eval::Eval;

use super::{
    gc::Heap,
    value::{Table, Value},
    Error,
};


// TODO:
//   - gc root tracked values in the api
//   - ctx impl
//   - vm eval impl
pub struct VM {
    global: Table,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        VM {
            global: Table::new(heap),
        }
    }

    pub fn eval<T>(&mut self, item: &T, heap: &Heap) -> Result<Value,Error>
    where
        T: Eval,
    {
        let mut ctx = Ctx::new(&mut self.global, heap);
        item.eval(&mut ctx).into()
    }
}
