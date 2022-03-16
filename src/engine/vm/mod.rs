pub mod ctx;
pub mod eval;

use super::value::Table;
use super::gc::Heap;
use ctx::Ctx;

pub struct VM {
    global: Table,
}

impl VM {
    pub fn new(heap: Heap) -> Self {
        VM {
            global: Table::new(heap),
        }
    }

    fn ctx<'a>(&'a mut self, heap: &'a Heap) -> Ctx<'a> {
        Ctx::new(&mut self.global, heap)
    }
}
