mod handle;
mod heuristics;
mod set;
mod trace;

use std::{alloc, cell::RefCell, ptr, rc::Rc};

pub use handle::{Handle, PtrTag, TaggedHandle};
use heuristics::Heuristics;
use set::ObjectSet;
pub use trace::{Trace, Visitor};

use super::value::{encoding, ByteString, Function, Table, Userdata};

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

    pub fn insert<T>(&self, value: T) -> Handle<T>
    where
        T: PtrTag,
    {
        self.internal.insert(value)
    }

    pub fn insert_string(&self, bytes: &[u8]) -> Handle<ByteString> {
        self.internal.insert_string(bytes)
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
        let ptr = Box::into_raw(Box::new_in(value, self));
        let handle = Handle::new(ptr);
        self.tree.borrow_mut().objects.insert(handle.tagged());
        handle
    }

    fn insert_string(&self, bytes: &[u8]) -> Handle<ByteString> {
        let len = bytes.len() as u32;
        let layout = ByteString::layout(len);

        unsafe {
            let ptr = alloc::Allocator::allocate(self, layout).unwrap().as_ptr() as *mut ByteString;
            ByteString::initialize_into(ptr, len);
            ptr::copy_nonoverlapping(bytes.as_ptr(), (&mut *ptr).offset(0), len as usize);
            let handle = Handle::new(ptr);
            self.tree.borrow_mut().objects.insert(handle.tagged());
            handle
        }
    }

    unsafe fn destroy(&self, handle: TaggedHandle) {
        let tagged = handle.value();

        match tagged {
            _ if encoding::is_string(tagged) => {
                let ptr = encoding::get_string(tagged) as *mut ByteString;
                let len = (*ptr).len();
                let layout = ByteString::layout(len as u32);
                let ptr_nn = ptr::NonNull::new_unchecked(ptr as *mut u8);
                alloc::Allocator::deallocate(self, ptr_nn, layout);
            },
            _ if encoding::is_table(tagged) => {
                let ptr = encoding::get_table(tagged) as *mut Table;
                Box::from_raw_in(ptr, self);
            },
            _ if encoding::is_function(tagged) => {
                let ptr = encoding::get_function(tagged) as *mut Function;
                Box::from_raw_in(ptr, self);
            },
            _ if encoding::is_userdata(tagged) => {
                let ptr = encoding::get_userdata(tagged) as *mut Userdata;
                Box::from_raw_in(ptr, self);
            },
            _ => panic!("unknown pointer type {:b}", tagged),
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

#[cfg(test)]
mod tests {
    use super::Heap;
    use super::super::value::Table;

    #[test]
    fn collect_no_trace() {
        let heap = Heap::new();
        let table1 = Table::new(heap.clone());
        let table2 = Table::new(heap.clone());

        heap.insert(table1);
        heap.insert(table2);

        let mut ctr = 0;
        heap.collect(|_| (), |_| ctr += 1);
        assert_eq!(ctr, 2);
    }

    #[test]
    fn collect_mark_direct() {
        let heap = Heap::new();
        let table1 = Table::new(heap.clone());
        let table2 = Table::new(heap.clone());
        let table3 = Table::new(heap.clone());
        let table4 = Table::new(heap.clone());

        let handle1 = heap.insert(table1).tagged();
        heap.insert(table2);
        heap.insert(table3);
        heap.insert(table4);

        let mut ctr = 0;
        heap.collect(|visitor| visitor.mark(handle1), |_| ctr += 1);
        assert_eq!(ctr, 3);
    }
}
