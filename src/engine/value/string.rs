use std::collections::HashSet;
use super::super::gc::{Heap, Handle, Compare};
use std::alloc::Layout;
use std::ptr;
use std::mem;

pub struct ZString {
    capacity: usize,
    len: usize,
    data: *mut u8,
}

impl ZString {
    unsafe fn layout(capacity: usize) -> Layout {
        Layout::from_size_align_unchecked(capacity, mem::align_of::<u8>())
    }

    pub fn new(heap: &Heap, from: &[u8]) -> Self {
        let capacity = from.len().next_power_of_two();
        
        unsafe {
            let layout = Self::layout(capacity);
            let data = heap.allocate_block(layout);
            ptr::copy_nonoverlapping(from.as_ptr(), data, from.len());

            Self {
                capacity,
                len: from.len(),
                data,
            }
        }
    }
}

//impl Drop for ZString {
//    fn drop(&mut self) {
//        unsafe {
//            let layout = Self::layout(self.capacity);
//            heap.deallocate_block(self.data, layout);
//        }
//    }
//}

pub struct Interner {
    map: HashSet<Compare<ZString>>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            map: HashSet::new(),
        }
    }
}
