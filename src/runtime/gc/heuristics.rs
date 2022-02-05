use std::cell::Cell;

const INITIAL_THRESHOLD: usize = 128 * 1024;
const THRESHOLD_FACTOR: f32 = 1.75;

pub struct Heuristics {
    allocated: Cell<usize>,
    threshold: Cell<usize>,
    should_collect: Cell<bool>,
}

impl Heuristics {
    pub fn new() -> Self {
        Self {
            allocated: Cell::new(0),
            threshold: Cell::new(INITIAL_THRESHOLD),
            should_collect: Cell::new(false),
        }
    }

    fn threshold(&self) -> usize {
        (self.threshold.get() as f32 * THRESHOLD_FACTOR) as usize
    }

    pub fn adjust(&self) {
        let new_threshold = self.threshold();
        self.threshold.set(new_threshold);
    }

    fn check_collect(&self) {
        if self.allocated >= self.threshold {
            self.should_collect.set(true);
            let new_threshold = self.threshold();
            self.threshold.set(new_threshold);
        }
    }

    pub fn update_allocated<F>(&self, f: F)
    where
        F: FnOnce(usize) -> usize,
    {
        self.allocated.update(f);
        self.check_collect();
    }

    pub fn should_collect(&self) -> bool {
        self.should_collect.get()
    }
}
