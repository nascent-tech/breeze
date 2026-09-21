use crate::clock::WallClock;
use crate::constants::{BREAKER_DISARM, BREAKER_THRESHOLD, BREAKER_WINDOW};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Breaker {
    crashes: Vec<WallClock>,
    armed: bool,
}

impl Breaker {
    pub fn new() -> Self {
        Breaker::default()
    }

    pub fn restore(crashes: Vec<WallClock>, armed: bool) -> Self {
        Breaker { crashes, armed }
    }

    pub fn crashes(&self) -> &[WallClock] {
        &self.crashes
    }

    pub fn record_crash(&mut self, at: WallClock) {
        self.tick(at);
        self.crashes
            .retain(|crash| at.saturating_duration_since(*crash) < BREAKER_WINDOW);
        self.crashes.push(at);
        if self.crashes.len() >= BREAKER_THRESHOLD as usize {
            self.armed = true;
        }
    }

    pub fn tick(&mut self, now: WallClock) {
        if !self.armed {
            return;
        }
        let Some(last) = self.crashes.iter().copied().max() else {
            return;
        };
        if now.saturating_duration_since(last) >= BREAKER_DISARM {
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
