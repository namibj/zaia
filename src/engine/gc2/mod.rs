//! The [`self`] module implements managed memory for the runtime. This includes an allocator,
//! a garbage collector and a set of systems to decide when to process garbage to maintain
//! optimal conditions for both the host and guest program.
//! 
//! The garbage collector is a generational, regionalized mark-sweep with incremental
//! old-space collecting allowing for a high-throughput garbage collection with soft
//! real-time capabilities at the expense of a slightly higher memory footprint and explicit tunability.
//! 
//! The collector switches between executing young-space and mixed-space passes to achieve it's target heap size.

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
