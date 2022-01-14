pub use broom::prelude::{Trace,Tracer,Handle,Rooted};
use super::object::Object;
use std::alloc;
use std::cell::RefCell;
use std::rc::Rc;

const INITIAL_MAX: usize = 4 * 1024 * 1024;
const HEAP_GROW_FACTOR: f32 = 1.5;
const RECLAIM_TARGET_FACTOR: f32 = 0.1;

pub struct Heap {
    internal: RefCell<InternalHeap>,
    garbage: Rc<RefCell<Vec<GarbageItem>>>,
}

impl Heap {
    pub fn new() -> Self {
        let garbage = Rc::new(RefCell::new(Vec::new()));
        let internal = RefCell::new(InternalHeap::new(Rc::clone(&garbage)));

        Self {
            internal,
            garbage,
        }
    }

    pub fn track(&self, object: Object) -> Rooted<Object> {
        self.internal.borrow_mut().track(object)
    }

    pub fn track_temporary(&self, object: Object) -> Handle<Object> {
        self.internal.borrow_mut().track_temporary(object)
    }

    pub unsafe fn allocate_block(&self, layout: alloc::Layout) -> *mut u8 {
        self.internal.borrow_mut().allocate_block(layout)
    }

    pub unsafe fn free_block(&self, layout: alloc::Layout, ptr: *mut u8) {
        let item = GarbageItem {layout, ptr };
       self.garbage.borrow_mut().push(item);
    }

    pub unsafe fn collect(&self) {
        self.internal.borrow_mut().collect();
    }
}

struct GarbageItem {
    layout: alloc::Layout,
    ptr: *mut u8,
}

struct InternalHeap {
    graph: broom::Heap<Object>,
    garbage: Rc<RefCell<Vec<GarbageItem>>>,
    used_size: usize,
    trigger_threshold: usize,
}

impl InternalHeap {
    fn new(garbage: Rc<RefCell<Vec<GarbageItem>>>) -> Self {
        Self {
            graph: broom::Heap::new(),
            garbage,
            used_size: 0,
            trigger_threshold: INITIAL_MAX,
        }
    }

    fn collect(&mut self) {
        self.graph.clean();

        for item in self.garbage.borrow_mut().drain(..) {
            unsafe {
                self.used_size -= item.layout.size();
                alloc::dealloc(item.ptr, item.layout);
            }
        }

        self.retune();
    }

    fn retune(&mut self) {
        if self.used_size >= self.trigger_threshold {
            self.trigger_threshold = (self.used_size as f32 * HEAP_GROW_FACTOR) as usize;
        } else if (self.used_size as f32 / self.trigger_threshold as f32) < RECLAIM_TARGET_FACTOR {
            self.trigger_threshold = (self.used_size as f32 * HEAP_GROW_FACTOR) as usize;
        }
    }

    fn check_collect(&mut self) {
        if self.used_size > self.trigger_threshold {
            self.collect();
        }
    }

    unsafe fn allocate_block(&mut self, layout: alloc::Layout) -> *mut u8 {
        self.used_size += layout.size();
        self.check_collect();
        alloc::alloc(layout)
    }

    fn track(&mut self, object: Object) -> Rooted<Object> {
        self.graph.insert(object)
    }

    fn track_temporary(&mut self, object: Object) -> Handle<Object> {
        self.graph.insert_temp(object)
    }
}
