mod gc;
mod scope;
mod value;
mod vm;

use gc::Heap as GenericHeap;
use value::RefValue;
use vm::VM;

pub type Heap = GenericHeap<RefValue>;

pub struct Runtime {
    heap: Heap,
    vm: Box<VM>,
}

impl Runtime {
    pub fn new() -> Self {
        let vm = Box::new(VM::new());
        let heap = Heap::new();
        Self { heap, vm }
    }
}
