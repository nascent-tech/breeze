use crate::clock::Instant;
use crate::constants::{BREAKER_DISARM, BREAKER_THRESHOLD, BREAKER_WINDOW};

#[derive(Clone, Debug, Default)]
pub struct Breaker {
    crashes: Vec<Instant>,
    armed: bool,
}

impl Breaker {
    pub fn new() -> Self {
        Breaker::default()
    }

    pub fn record_crash(&mut self, at: Instant) {
        self.crashes
            .retain(|crash| at.elapsed_since(*crash) < BREAKER_DISARM);
        self.crashes.push(at);
        let recent = self
            .crashes
            .iter()
            .filter(|crash| at.elapsed_since(**crash) < BREAKER_WINDOW)
            .count();
        if recent >= BREAKER_THRESHOLD as usize {
            self.armed = true;
        }
    }

    pub fn tick(&mut self, now: Instant) {
        if !self.armed {
            return;
        }
        let Some(last) = self.crashes.last().copied() else {
            return;
        };
        if now.elapsed_since(last) >= BREAKER_DISARM {
            self.reset();
        }
    }

    pub fn is_armed(&self) -> bool {
        self.armed
    }

    pub fn reset(&mut self) {
        self.armed = false;
        self.crashes.clear();
    }
}
