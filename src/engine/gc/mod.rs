mod handle;
mod trace;

use std::alloc;
use std::ptr;
use std::cell::RefCell;
use std::collections::HashSet;
use handle::Handle;
use trace::{Trace, Visitor};
use std::rc::Rc;

const INITIAL_THRESHOLD: usize = 128 * 1024;

#[derive(Clone)]
pub struct Heap<T, B> {
    internal: Rc<RefCell<HeapInternal<T, B>>>,
}

impl<T, B> Heap<T, B> where B: Trace<T> {
    pub fn new(base: B) -> Self {
        Heap {
            internal: Rc::new(RefCell::new(HeapInternal::new(base))),
        }
    }

    pub fn insert(&self, value: T) -> Handle<T> {
        self.internal.borrow_mut().insert(value)
    }

    pub fn collect(&self) {
        self.internal.borrow_mut().collect();
    }
}

struct HeapInternal<T, B> {
    allocated: usize,
    threshold: usize,
    objects: HashSet<Handle<T>>,
    base: B,
}

impl<T, B> HeapInternal<T, B> where B:Trace<T> {
    fn new(base: B) -> Self {
        Self {
            allocated: 0,
            threshold: INITIAL_THRESHOLD,
            objects: HashSet::new(),
            base
        }
    }

    fn insert(&mut self, value: T) -> Handle<T> {
        let ptr = Box::into_raw(Box::new(value));
        let handle = Handle::new(ptr);
        self.objects.insert(handle);
        handle
    }

    fn update_allocated<F>(&mut self, f: F) where F:FnOnce(usize) -> usize {
        self.allocated = f(self.allocated);
    }

    fn collect(&mut self) {
        let visitor = Visitor::new();
        let marked = visitor.run(&self.base);
        
        for object in self.objects.difference(&marked) {
            unsafe {
                object.destroy();
            }
        }
    }
}

unsafe impl<T, B> alloc::Allocator for Heap<T, B> where B: Trace<T> {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.borrow_mut().update_allocated(|x| x + layout.size());
        alloc::Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: alloc::Layout) {
        self.internal.borrow_mut().update_allocated(|x| x - layout.size());
        alloc::Global.deallocate(ptr, layout)
    }

    unsafe fn grow(&self, ptr: ptr::NonNull<u8>, old_layout: alloc::Layout, new_layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.borrow_mut().update_allocated(|x| x + new_layout.size() - old_layout.size());
        alloc::Global.grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(&self, ptr: ptr::NonNull<u8>, old_layout: alloc::Layout, new_layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.borrow_mut().update_allocated(|x| x + new_layout.size() - old_layout.size());
        alloc::Global.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(&self, ptr: ptr::NonNull<u8>, old_layout: alloc::Layout, new_layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.borrow_mut().update_allocated(|x| x + new_layout.size() - old_layout.size());
        alloc::Global.shrink(ptr, old_layout, new_layout)
    }
}
