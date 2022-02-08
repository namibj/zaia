mod handle;
mod heuristics;
mod trace;

use std::{alloc, cell::RefCell, ptr, rc::Rc};

pub use handle::Handle;
use hashbrown::HashSet;
use heuristics::Heuristics;
pub use trace::{Trace, Visitor};

pub struct Heap<T> {
    internal: Rc<HeapInternal<T>>,
}

impl<T> Heap<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Heap {
            internal: Rc::new(HeapInternal::new()),
        }
    }

    pub fn insert(&self, value: T) -> Handle<T> {
        self.internal.insert(value)
    }

    pub fn collect<F1, F2>(&self, trace: F1, finalize: F2)
    where
        F1: FnOnce(&mut Visitor<T>),
        F2: FnMut(Handle<T>),
    {
        self.internal.collect(trace, finalize);
    }

    pub fn should_collect(&self) -> bool {
        self.internal.heuristics.should_collect()
    }
}

impl<T> Clone for Heap<T> {
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

impl<T> Tree<T> {
    fn collect<F1, F2>(&mut self, trace: F1, mut finalize: F2)
    where
        F1: FnOnce(&mut Visitor<T>),
        F2: FnMut(Handle<T>),
    {
        trace(&mut self.visitor);

        for object in self.visitor.unmarked(&self.objects) {
            finalize(*object);
            self.objects.remove(object);

            unsafe {
                object.destroy();
            }
        }

        self.visitor.reset();
    }
}

struct HeapInternal<T> {
    heuristics: Heuristics,
    tree: RefCell<Tree<T>>,
}

impl<T> HeapInternal<T> {
    fn new() -> Self {
        let tree = RefCell::new(Tree {
            objects: HashSet::new(),
            visitor: Visitor::new(),
        });

        Self {
            heuristics: Heuristics::new(),
            tree,
        }
    }

    fn insert(&self, value: T) -> Handle<T> {
        let ptr = Box::into_raw(Box::new(value));
        let handle = Handle::new(ptr);
        self.tree.borrow_mut().objects.insert(handle);
        handle
    }

    fn collect<F1, F2>(&self, trace: F1, finalize: F2)
    where
        F1: FnOnce(&mut Visitor<T>),
        F2: FnMut(Handle<T>),
    {
        self.tree.borrow_mut().collect(trace, finalize);
        self.heuristics.adjust();
    }
}

unsafe impl<T> alloc::Allocator for Heap<T> {
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

impl<T> Drop for HeapInternal<T> {
    fn drop(&mut self) {
        let tree = self.tree.get_mut();
        tree.objects.iter().for_each(|object| unsafe {
            object.destroy();
        });
    }
}
