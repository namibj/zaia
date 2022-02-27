use std::{alloc, mem::MaybeUninit};

// TODO: make this neater
use super::{super::gc::PtrTag, encoding};

#[repr(C)]
pub struct ByteString {
    len: u32,
    data: [MaybeUninit<u8>],
}

impl ByteString {
    pub fn initialize_into(ptr: *mut Self, len: u32) {
        unsafe {
            (*ptr).len = len;
        }
    }

    pub fn offset(&mut self, offset: usize) -> *mut u8 {
        unsafe { (&mut self.data as *mut [MaybeUninit<u8>] as *mut u8).add(offset) }
    }

    pub fn layout(len: u32) -> alloc::Layout {
        let size = std::mem::size_of::<u32>() + len as usize;
        let align = std::mem::align_of::<u32>();
        alloc::Layout::from_size_align(size, align).unwrap()
    }

    pub unsafe fn len_from_thin(ptr: *mut u8) -> u32 {
        *(ptr as *mut u32)
    }
}

unsafe impl PtrTag for ByteString {
    fn is(x: u64) -> bool {
        encoding::is_string(x)
    }

    fn tag(x: usize) -> u64 {
        encoding::make_string(x as *mut u8)
    }
}
