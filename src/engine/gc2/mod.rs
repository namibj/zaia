mod internal;

use std::cell::RefCell;
use internal::InternalHeap;

pub struct Heap {
    internal: RefCell<InternalHeap>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            internal: RefCell::new(InternalHeap::new()),
        }
    }
}
