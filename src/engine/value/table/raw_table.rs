use super::super::Value;
use std::mem;
use std::alloc;
use super::super::super::gc::Heap;
use std::cmp;

const INITIAL_CAPACITY: usize = 8;

const EMPTY: u8 = 0b1111_1111;
const DELETED: u8 = 0b1000_0000;

fn hash(value: Value) -> u64 {
    match value {
        Value::Nil => 0,
        Value::Boolean(b) => b as u64,
        _ => todo!(),
    }
}

fn is_full(ctrl: u8) -> bool {
    ctrl & 0x80 == 0
}

fn h1(hash: u64) -> usize {
    // On 32-bit platforms we simply ignore the higher hash bits.
    hash as usize
}

fn h2(hash: u64) -> u8 {
    // Grab the top 7 bits of the hash. While the hash is normally x full 64-bit
    // value, some hash functions (such as FxHash) produce x usize result
    // instead, which means that the top 32 bits are 0 on 32-bit platforms.
    let hash_len = usize::min(mem::size_of::<usize>(), mem::size_of::<u64>());
    let top7 = hash >> (hash_len * 8 - 7);
    (top7 & 0x7f) as u8 // truncation
}

unsafe fn alloc_layout(capacity: usize) -> alloc::Layout {
    let size = capacity * mem::size_of::<Value>() + cmp::max(capacity, mem::align_of::<Value>());
    alloc::Layout::from_size_align_unchecked(size, mem::align_of::<Value>())
}

unsafe fn allocate(heap: &Heap, capacity: usize) -> (*mut u8, *mut TableEntry) {
    let layout = alloc_layout(capacity);
    let base = heap.allocate_block(layout);
    let entry_base = base.add(cmp::max(capacity, mem::align_of::<Value>()));
    (base, entry_base as _)
}

struct TableEntry {
    key: Value,
    value: Value,
}

pub struct RawTable {
    capacity: usize,
    len: usize,
    meta: *mut u8,
    slots: *mut TableEntry,
}

impl RawTable {
    pub fn new(heap: &Heap) -> Self {
        Self::with_capacity(heap, INITIAL_CAPACITY)
    }

    pub fn with_capacity(heap: &Heap, capacity: usize) -> Self {
        let (meta, slots) = unsafe { allocate(&heap, capacity) };
        
        Self {
            capacity,
            len: 0,
            meta,
            slots,
        }
    }
}
