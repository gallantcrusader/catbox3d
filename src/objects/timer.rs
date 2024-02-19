use std::{
    cell::Cell,
    time::{Duration, Instant},
};

pub struct Timer {
    time: Cell<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            time: Cell::new(Instant::now()),
        }
    }

    pub fn reset(&mut self) {
        self.time.replace(Instant::now());
    }

    pub fn get_time(&self) -> Duration {
        self.time.get().elapsed()
    }
}
