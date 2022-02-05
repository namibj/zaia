mod handle;
mod heuristics;
mod trace;

use std::{alloc, cell::RefCell, ptr, rc::Rc};

pub use handle::Handle;
use hashbrown::HashSet;
use heuristics::Heuristics;
pub use trace::{Trace, Visitor};

pub struct Heap<T, B> {
    internal: Rc<HeapInternal<T, B>>,
}

impl<T, B> Heap<T, B>
where
    B: Trace<T>,
{
    pub fn new(base: B) -> Self {
        Heap {
            internal: Rc::new(HeapInternal::new(base)),
        }
    }

    pub fn insert(&self, value: T) -> Handle<T> {
        self.internal.insert(value)
    }

    pub fn collect(&self) {
        self.internal.collect();
    }

    pub fn should_collect(&self) -> bool {
        self.internal.heuristics.should_collect()
    }
}

impl<T, B> Clone for Heap<T, B> {
    fn clone(&self) -> Self {
        Heap {
            internal: self.internal.clone(),
        }
    }
}

struct Tree<T> {
    objects: HashSet<Handle<T>>,
    visitor: Visitor<T>,
}

struct HeapInternal<T, B> {
    heuristics: Heuristics,
    tree: RefCell<Tree<T>>,
    base: B,
}

impl<T, B> HeapInternal<T, B>
where
    B: Trace<T>,
{
    fn new(base: B) -> Self {
        let tree = RefCell::new(Tree {
            objects: HashSet::new(),
            visitor: Visitor::new(),
        });

        Self {
            heuristics: Heuristics::new(),
            tree,
            base,
        }
    }

    fn insert(&self, value: T) -> Handle<T> {
        let ptr = Box::into_raw(Box::new(value));
        let handle = Handle::new(ptr);
        self.tree.borrow_mut().objects.insert(handle);
        handle
    }

    fn collect(&self) {
        let mut tree = self.tree.borrow_mut();
        tree.visitor.run(&self.base);
        for object in tree.visitor.unmarked(&tree.objects) {
            unsafe {
                object.destroy();
            }
        }

        tree.visitor.reset();
    }
}

unsafe impl<T, B> alloc::Allocator for Heap<T, B>
where
    B: Trace<T>,
{
    fn allocate(&self, layout: alloc::Layout) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal
            .heuristics
            .update_allocated(|x| x + layout.size());

        alloc::Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: alloc::Layout) {
        self.internal
            .heuristics
            .update_allocated(|x| x - layout.size());

        alloc::Global.deallocate(ptr, layout)
    }

    unsafe fn grow(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal
            .heuristics
            .update_allocated(|x| x + new_layout.size() - old_layout.size());

        alloc::Global.grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal
            .heuristics
            .update_allocated(|x| x + new_layout.size() - old_layout.size());

        alloc::Global.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: ptr::NonNull<u8>,
        old_layout: alloc::Layout,
        new_layout: alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, alloc::AllocError> {
        self.internal
            .heuristics
            .update_allocated(|x| x + new_layout.size() - old_layout.size());

        alloc::Global.shrink(ptr, old_layout, new_layout)
    }
}
