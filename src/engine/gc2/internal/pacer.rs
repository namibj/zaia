use std::time::Duration;

pub struct Pacer {
    max_pause: Duration,
}

impl Pacer {
    pub fn new(max_pause: Duration) -> Self {
        Self {
            max_pause,
        }
    }

    pub fn max_pause(&self) -> Duration {
        self.max_pause
    }
}
