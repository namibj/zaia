mod gc;
mod marker;
mod scope;
mod value;
mod vm;

use std::ptr::NonNull;

use gc::Heap;
use marker::Marker;
use value::RefValue;
use vm::VM;

pub struct Runtime {
    heap: Heap<RefValue, Marker>,
    vm: Box<VM>,
}

impl Runtime {
    pub fn new() -> Self {
        let vm = Box::new(VM::new());
        let marker = Marker::new(NonNull::from(&*vm));
        let heap = Heap::new(marker);
        Self { heap, vm }
    }
}
