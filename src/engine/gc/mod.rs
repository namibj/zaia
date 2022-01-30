use std::mem::MaybeUninit;
use std::alloc::{alloc, Layout, dealloc};
use std::ptr;

const BITMAP_SIZE: usize = 1008;
const CELLS_SIZE: usize = 129024;

#[repr(C)]
#[repr(align(131072))]
pub struct Arena {
    gray_stack: Vec<*mut u8>,
    block: [u8; BITMAP_SIZE],
    mark: [u8; BITMAP_SIZE],
    cells: MaybeUninit<[u8; CELLS_SIZE]>,
}

impl Arena {
    pub fn new() -> *mut Self {
        let layout = Layout::new::<Self>();
        
        unsafe {
            let arena = alloc(layout) as *mut Self;
            let gray_stack_field = ptr::addr_of_mut!((*arena).gray_stack);
            let block_field = ptr::addr_of_mut!((*arena).block);
            let mark_field = ptr::addr_of_mut!((*arena).mark);
            ptr::write(gray_stack_field, Vec::new());
            ptr::write_bytes(block_field, 0, BITMAP_SIZE);
            ptr::write_bytes(mark_field, 0xFF, BITMAP_SIZE);
            arena
        }
    }

    pub fn container_of(block_ptr: *mut u8) -> *mut Self {
        let arena_raw = block_ptr as usize & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1110_0000_0000_0000_0000;
        arena_raw as *mut Self
    }

    pub unsafe fn destroy(self: *mut Self) {
        let layout = Layout::new::<Self>();
        dealloc(self as *mut u8, layout);
    }
}
