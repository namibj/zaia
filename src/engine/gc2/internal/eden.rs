use std::alloc::{Layout, alloc};
use std::mem;

const EDEN_INITIAL_SIZE: usize = 1024 * 64;

fn layout(size: usize) -> Layout {
    unsafe {
        Layout::from_size_align_unchecked(size, 1)
    }
}

pub struct Eden {
    size: usize,
    base: *mut u8,
    cursor: *mut u8,
}

impl Eden {
    pub fn new() -> Self {
        let layout = layout(EDEN_INITIAL_SIZE);
        let base = unsafe { alloc(layout) };

        Self {
            size: EDEN_INITIAL_SIZE,
            base,
            cursor: base,
        }
    }

    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        // Calculate total size.
        let alloc_start = self.cursor;
        let total_size = mem::size_of::<usize>() + size;

        // Make sure we have enough space eden memory for this allocation
        let remaining = unsafe { self.cursor.sub(self.base as usize) as usize };
        if remaining < total_size {
            self.evacuate();
        }

        // Advance the eden cursor
        self.cursor = unsafe { self.cursor.add(total_size) };
        
        // Write the size of the allocation into the header
        let header = alloc_start as *mut usize;
        unsafe { *header = size };

        // Compute the payload pointer by skipping past the header
        let payload = unsafe { alloc_start.add(mem::size_of::<usize>()) };
        payload
    }

    pub fn evacuate(&mut self) {
        todo!()
    }
}
