use std::{cmp, time::Duration};

use super::optimizer::ConvexOptimizer;

const EDEN_SIZE_MINIMUM: usize = 1024 * 16;
const EDEN_SIZE_STABILITY_THRESHOLD: f32 = 1024.0;
const EDEN_SIZE_ROUNDING: usize = 1024;

fn smoothed(current: f32, new: f32) -> f32 {
    (current * 2.0 + new) / 3.0
}

pub struct Pacer {
    /// `max_pause` is the maximum GC pause time that we should allow in
    /// seconds.
    max_pause: f32,

    /// `evacuation_rate` is the evacuation speed from the eden region in bytes
    /// per second. This is stored in the form of an exponential moving
    /// average in order to smooth out various extreme values caused by the
    /// environment.
    evacuation_rate: f32,

    /// `eden_optimizer` is a convex optimizer used to find the optimal eden
    /// size.
    eden_optimizer: ConvexOptimizer,
}

impl Pacer {
    pub fn new(max_pause: Duration) -> Self {
        Self {
            max_pause: max_pause.as_secs_f32(),
            evacuation_rate: 0.0,
            eden_optimizer: ConvexOptimizer::new(
                EDEN_SIZE_MINIMUM as f32,
                EDEN_SIZE_STABILITY_THRESHOLD,
            ),
        }
    }

    pub fn adjust_max_pause(&mut self, max_pause: Duration) {
        self.max_pause = max_pause.as_secs_f32();
    }

    /// `step_eden` collects eden evacuation metrics and recommends the new size
    /// of the eden region. The algorithm attempts to adapts to runtime
    /// conditions of the existing system by dynamically modifying internal
    /// tuning parameters based on previous collection metrics to minimize
    /// the runtime of the next collection and hit latency goals.
    pub fn step_eden(
        &mut self,
        eden_size: usize,
        heap_size: usize,
        elapsed: Duration,
        objects: usize,
        survivors: usize,
    ) -> usize {
        // Calculate the evacauation rate. This is smoothed to prevent
        // fluctuations in the metrics from throwing off the maximum eden size
        // which could potentially cause us to miss latency goals during the next cycle.
        let observed_evacuation_rate = eden_size as f32 / elapsed.as_secs_f32();
        self.evacuation_rate = smoothed(self.evacuation_rate, observed_evacuation_rate);

        // Measure the amount of work that was required for this collection and
        // step the optimizer to produce a new initial eden size.
        let observed_evacuation_survivors = survivors as f32 / objects as f32;
        let evacuation_work = observed_evacuation_survivors * eden_size as f32;
        let mut size = self.eden_optimizer.step(evacuation_work) as usize;

        // Constrict size so that we always evacuate within the time limit.
        let latency = (self.max_pause * self.evacuation_rate * 0.9) as usize;
        size = cmp::min(size, latency);

        // Limit eden size to a proportion of the heap size which is generally more
        // stable. This is done to prevent the eden size from growing extremely
        // large in comparison to heap which may be unexpected.
        let proportional = heap_size / 4;
        size = cmp::min(size, proportional);

        // Bound the eden size to a minimum.
        size = cmp::max(size, EDEN_SIZE_MINIMUM);

        // Round the eden size to prevent tiny fluctuations.
        size as usize / EDEN_SIZE_ROUNDING * EDEN_SIZE_ROUNDING
    }
}
