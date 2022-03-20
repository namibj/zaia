pub mod ctx;
pub mod eval;

use ctx::Ctx;
use eval::Eval;
use crate::parser::machinery::cstree::interning::TokenInterner;

use super::{
    gc::Heap,
    value::{Table, Value},
    Error,
};

// TODO:
//   - gc root tracked values in the api
//   - vm eval impl
//   - catch break stmts & handle scoping
pub struct VM {
    global: Table,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        VM {
            global: Table::new(heap),
        }
    }

    pub fn eval<T>(&mut self, item: &T, heap: &Heap, interner: &TokenInterner) -> Result<Value, Error>
    where
        T: Eval,
    {
        let mut ctx = Ctx::new(&mut self.global, heap, interner);
        item.eval(&mut ctx).into()
    }
}
