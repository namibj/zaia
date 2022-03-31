use std::time::Duration;
use std::cmp;

const EDEN_SIZE_MINIMUM: usize = 1024 * 16;
const EDEN_SIZE_MAXIMUM: usize = 1024 * 1024 * 16;

fn smoothed(current: f32, new: f32) -> f32 {
    (current * 2.0 + new) / 3.0
}

pub struct Pacer {
    /// `max_pause` is the maximum GC pause time that we should allow in seconds.
    max_pause: f32,

    /// `evacuation_rate` is the evacuation speed from the eden region in bytes per second.
    /// This is stored in the form of an exponential moving average in
    /// order to smooth out various extreme values caused by the environment.
    evacuation_rate: f32,

    /// `evacuation_survivors` is the factor of the eden space that survived the last evacuation.
    evacuation_survivors: f32,
}

impl Pacer {
    pub fn new(max_pause: Duration) -> Self {
        Self {
            max_pause: max_pause.as_secs_f32(),
            evacuation_rate: 0.0,
            evacuation_survivors: 0.0,
        }
    }

    pub fn adjust_max_pause(&mut self, max_pause: Duration) {
        self.max_pause = max_pause.as_secs_f32();
    }

    pub fn report_evacuation_metrics(&mut self, bytes: usize, elapsed: Duration, objects: usize, survivors: usize) {
        let observed_evacuation_rate = bytes as f32 / elapsed.as_secs_f32();
        self.evacuation_rate = smoothed(self.evacuation_rate, observed_evacuation_rate);
        
        let observed_evacuation_survivors = objects as f32 / survivors as f32;
        self.evacuation_survivors = smoothed(self.evacuation_survivors, observed_evacuation_survivors);
    }

    // TODO: optimize after survivorship ratio
    // TODO: optimize after total work done (copying etc)
    // NOTE: we want to minimize the survivor ratio
    // NOTE: we assume the evacuation rate remains fairly steady as the eden size changes
    // NOTE: survivor ratio is a convex function of eden size that we need to optimize
    // NOTE: gradient descent? other optimization techniques?
    pub fn recommended_eden_size(&self, heap_size: usize) -> usize {
        // The starting point we'll use is 90% of what we can process while hitting latency goals.
        let mut size = (self.max_pause * self.evacuation_rate * 0.9) as usize;

        // Limit eden size to a proportion of the heap size which is generally more stable.
        // This is done to prevent the eden size from growing extremely large
        // in comparison to heap which may be unexpected.
        let proportional = heap_size / 5;
        size = cmp::min(size, proportional);

        // Bound the eden size to a minimum and maximum.
        size = cmp::max(size, EDEN_SIZE_MINIMUM);
        size = cmp::min(size, EDEN_SIZE_MAXIMUM);

        size
    }
}
