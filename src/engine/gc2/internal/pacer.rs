use std::time::Duration;

pub struct Pacer {
    /// `max_pause` is the maximum GC pause time that we should allow.
    max_pause: Duration,

    /// `evacuation_rate` is the evacuation speed from the eden region in bytes per second.
    /// This is stored in the form of an exponential moving average in
    /// order to smooth out various extreme values caused by the environment.
    evacuation_rate: f32,
}

impl Pacer {
    pub fn new(max_pause: Duration) -> Self {
        Self {
            max_pause,
            evacuation_rate: 0.0,
        }
    }

    pub fn max_pause(&self) -> Duration {
        self.max_pause
    }

    pub fn report_evacuation_metrics(&mut self, bytes: usize, elapsed: Duration) {
        let new_rate = bytes as f32 / elapsed.as_secs_f32();
        self.evacuation_rate = (self.evacuation_rate + new_rate) / 2.0;
    }
}
