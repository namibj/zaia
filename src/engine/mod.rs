mod gc;
mod scope;
mod value;
mod vm;

use value::RefValue;
use vm::VM;

pub type Heap = gc::Heap<RefValue>;

pub struct Runtime {
    heap: Heap,
    vm: VM,
}

impl Runtime {
    pub fn new() -> Self {
        let heap = Heap::new();
        let vm = VM::new(heap.clone());
        Self { heap, vm }
    }
}
