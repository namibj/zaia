use std::{mem::MaybeUninit, ops::Add};

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
        unsafe {
            (&mut self.data as *mut [MaybeUninit<u8>] as *mut u8).add(offset)
        }
    }
}