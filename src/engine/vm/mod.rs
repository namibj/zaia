pub mod ctx;
pub mod eval;

use std::collections::hash_map::RandomState;

use ctx::Ctx;
use eval::Eval;
use hashbrown::HashMap;

use super::{
    gc::{Handle, Heap},
    value::{ByteString, Table, Value},
    Error,
};
use crate::parser::machinery::cstree::interning::TokenInterner;

// TODO:
//   - gc root tracked values in the api
//   - vm eval impl
//   - catch break stmts & handle scoping
//   - impl _ENV
//   - handle multivalue
//   - gc interned strings
pub struct VM {
    global: Table,
    strings: HashMap<Vec<u8>, Handle<ByteString>, RandomState>,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        VM {
            global: Table::new(heap),
            strings: HashMap::with_hasher(RandomState::new()),
        }
    }

    pub fn eval<T>(
        &mut self,
        item: &T,
        heap: &Heap,
        interner: &TokenInterner,
    ) -> Result<Value, Error>
    where
        T: Eval,
    {
        let ctx = Ctx::new(&mut self.global, heap, interner);
        item.eval(&ctx).into()
    }
}
