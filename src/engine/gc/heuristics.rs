use std::cell::Cell;

use super::{trace::Trace, Heap};

const INITIAL_THRESHOLD: usize = 128 * 1024;
const THRESHOLD_FACTOR: f32 = 1.75;

pub struct Heuristics {
    allocated: Cell<usize>,
    threshold: Cell<usize>,
    in_cycle: Cell<bool>,
}

impl Heuristics {
    pub fn new() -> Self {
        Self {
            allocated: Cell::new(0),
            threshold: Cell::new(INITIAL_THRESHOLD),
            in_cycle: Cell::new(false),
        }
    }

    fn threshold(&self) -> usize {
        (self.threshold.get() as f32 * THRESHOLD_FACTOR) as usize
    }

    fn check_collect<T, B>(&self, heap: &Heap<T, B>)
    where
        B: Trace<T>,
    {
        if !self.in_cycle.get() && self.allocated >= self.threshold {
            self.in_cycle.set(true);
            heap.collect();
            self.in_cycle.set(false);

            let new_threshold = self.threshold();
            self.threshold.set(new_threshold);
        }
    }

    pub fn update_allocated<T, B, F>(&self, heap: &Heap<T, B>, f: F)
    where
        B: Trace<T>,
        F: FnOnce(usize) -> usize,
    {
        self.allocated.update(f);
        self.check_collect(heap);
    }
}
