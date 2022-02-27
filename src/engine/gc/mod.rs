mod handle;
mod heuristics;
mod set;
mod trace;

use std::{alloc, cell::RefCell, ptr, rc::Rc};

pub use handle::{Handle, PtrTag, TaggedHandle};
use heuristics::Heuristics;
use set::ObjectSet;
pub use trace::{Trace, Visitor};

use super::value::ByteString;

pub struct Heap {
    internal: Rc<HeapInternal>,
}

impl Heap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Heap {
            internal: Rc::new(HeapInternal::new()),
        }
    }

    pub unsafe fn destroy(&self, handle: TaggedHandle) {
        self.internal.destroy(handle);
    }

    pub fn collect<F1, F2>(&self, trace: F1, finalize: F2)
    where
        F1: FnOnce(&mut Visitor),
        F2: FnMut(TaggedHandle),
    {
        self.internal.collect(trace, finalize);
    }

    pub fn should_collect(&self) -> bool {
        self.internal.heuristics.should_collect()
    }
}

impl Clone for Heap {
    fn clone(&self) -> Self {
        Heap {
            internal: self.internal.clone(),
        }
    }
}

struct Tree {
    objects: ObjectSet,
    visitor: Visitor,
}

impl Tree {
    fn collect<F1, F2>(&mut self, heap: &HeapInternal, trace: F1, mut finalize: F2)
    where
        F1: FnOnce(&mut Visitor),
        F2: FnMut(TaggedHandle),
    {
        trace(&mut self.visitor);

        for object in self.visitor.unmarked(&self.objects) {
            finalize(object);
            self.objects.remove(object);

            unsafe {
                heap.destroy(object);
            }
        }

        self.visitor.reset();
    }
}

struct HeapInternal {
    heuristics: Heuristics,
    tree: RefCell<Tree>,
}

impl HeapInternal {
    fn new() -> Self {
        let tree = RefCell::new(Tree {
            objects: ObjectSet::new(),
            visitor: Visitor::new(),
        });

        Self {
            heuristics: Heuristics::new(),
            tree,
        }
    }

    fn insert<T>(&self, value: T) -> Handle<T>
    where
        T: PtrTag,
    {
        let ptr = Box::into_raw(Box::new(value));
        let handle = Handle::new(ptr);
        self.tree.borrow_mut().objects.insert(handle.tagged());
        handle
    }

    unsafe fn destroy(&self, handle: TaggedHandle) {
        const TAG_MASK: usize = 0b111;
        const CLEAR_MASK: usize = !0b111;

        let tagged = handle.value();
        let tag = tagged & TAG_MASK;
        let ptr = (tagged & CLEAR_MASK) as *mut u8;
        let ptr_nn = ptr::NonNull::new_unchecked(ptr);

        match tag {
            ByteString::PTR_TAG => {
                let len = ByteString::len_from_thin(ptr);
                let layout = ByteString::layout(len);
                alloc::Allocator::deallocate(self, ptr_nn, layout);
            },
            _ => panic!("unknown pointer tag {:b}", tag),
        }
    }

    fn collect<F1, F2>(&self, trace: F1, finalize: F2)
    where
        F1: FnOnce(&mut Visitor),
        F2: FnMut(TaggedHandle),
    {
        self.tree.borrow_mut().collect(self, trace, finalize);
        self.heuristics.adjust();
    }
}

unsafe impl alloc::Allocator for Heap {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: alloc::Layout) {
        self.internal.deallocate(ptr, layout)
    }

    unsafe fn grow(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal.shrink(ptr, old_layout, new_layout)
    }
}

unsafe impl alloc::Allocator for HeapInternal {
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.heuristics.update_allocated(|x| x + layout.size());

        alloc::Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: alloc::Layout) {
        self.heuristics.update_allocated(|x| x - layout.size());

        alloc::Global.deallocate(ptr, layout)
    }

    unsafe fn grow(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.heuristics
            .update_allocated(|x| x + new_layout.size() - old_layout.size());

        alloc::Global.grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.heuristics
            .update_allocated(|x| x + new_layout.size() - old_layout.size());

        alloc::Global.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.heuristics
            .update_allocated(|x| x + new_layout.size() - old_layout.size());

        alloc::Global.shrink(ptr, old_layout, new_layout)
    }
}

impl Drop for HeapInternal {
    fn drop(&mut self) {
        let tree = self.tree.borrow();
        tree.objects.iter().for_each(|object| unsafe {
            self.destroy(object);
        });
    }
}
