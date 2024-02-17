use std::time::{Duration, Instant};

pub struct Timer {
    last: Instant,
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }

    pub fn ready(&mut self) -> bool {
        if self.last.elapsed() >= self.interval {
            self.last = Instant::now();
            true
        } else {
            false
        }
    }
}
