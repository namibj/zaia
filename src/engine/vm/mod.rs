pub mod ctx;
pub mod eval;

use std::collections::hash_map::RandomState;

use ctx::Ctx;
use eval::Eval;
use hashbrown::{HashMap, HashSet};

use super::{
    gc::{Handle, Heap},
    value::{ByteString, Table, Value},
    Error,
};
use crate::parser::machinery::cstree::interning::TokenInterner;

// TODO:
//   - vm eval impl
//   - gc root tracked values in the api
//   - impl _ENV
//   - handle multivalue
pub struct VM {
    global: Table,
    strings: HashSet<Handle<ByteString>, RandomState>,
    extern_ref: HashMap<Value, usize, RandomState>,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        VM {
            global: Table::new(heap),
            strings: HashSet::with_hasher(RandomState::new()),
            extern_ref: HashMap::with_hasher(RandomState::new()),
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
        let ctx = Ctx::new(&mut self.global, heap, interner, &mut self.strings);
        item.eval(&ctx).into()
    }
}
