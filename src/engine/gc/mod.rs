mod handle;

use std::alloc;
use std::ptr;
use std::cell::Cell;
use std::collections::HashSet;
use handle::Handle;

const INITIAL_THRESHOLD: usize = 128 * 1024;

pub struct Heap<T> {
    allocated: Cell<usize>,
    threshold: usize,
    objects: HashSet<Handle<T>>,
}

impl<T> Heap<T> {
    pub fn new() -> Self {
        Heap {
            allocated: Cell::new(0),
            threshold: INITIAL_THRESHOLD,
            objects: HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        let ptr = Box::into_raw(Box::new(value));
        let handle = Handle::new(ptr);
        self.objects.insert(handle);
        handle
    }

    pub fn collect(&mut self) {
        todo!()
    }
}

unsafe impl<T> alloc::Allocator for Heap<T> {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.allocated.update(|x| x + layout.size());
        alloc::Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: alloc::Layout) {
        self.allocated.update(|x| x - layout.size());
        alloc::Global.deallocate(ptr, layout)
    }

    unsafe fn grow(&self, ptr: ptr::NonNull<u8>, old_layout: alloc::Layout, new_layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.allocated.update(|x| x + new_layout.size() - old_layout.size());
        alloc::Global.grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(&self, ptr: ptr::NonNull<u8>, old_layout: alloc::Layout, new_layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.allocated.update(|x| x + new_layout.size() - old_layout.size());
        alloc::Global.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(&self, ptr: ptr::NonNull<u8>, old_layout: alloc::Layout, new_layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.allocated.update(|x| x + new_layout.size() - old_layout.size());
        alloc::Global.shrink(ptr, old_layout, new_layout)
    }
}
