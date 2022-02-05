mod gc;
mod marker;
mod scope;
mod value;
mod vm;

use gc::Heap as GenericHeap;
use marker::Marker;
use value::RefValue;
use vm::VM;

pub type Heap = GenericHeap<RefValue, Marker>;

pub struct Runtime {
    heap: Heap,
    vm: Box<VM>,
}

impl Runtime {
    pub fn new() -> Self {
        let vm = Box::new(VM::new());
        let marker = Marker::new(&*vm);
        let heap = Heap::new(marker);
        Self { heap, vm }
    }
}
