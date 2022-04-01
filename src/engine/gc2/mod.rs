mod internal;

use std::cell::RefCell;

use internal::InternalHeap;

pub struct Heap {
    internal: RefCell<InternalHeap>,
}

impl Heap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Heap {
        Heap {
            internal: RefCell::new(InternalHeap::new()),
        }
    }
}
