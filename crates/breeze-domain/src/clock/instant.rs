use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(Duration);

impl Instant {
    pub const EPOCH: Instant = Instant(Duration::ZERO);

    pub fn at_millis(milliseconds: u64) -> Self {
        Instant(Duration::from_millis(milliseconds))
    }

    pub fn at_secs(seconds: u64) -> Self {
        Instant(Duration::from_secs(seconds))
    }

    pub fn plus(self, elapsed: Duration) -> Self {
        Instant(self.0.saturating_add(elapsed))
    }

    pub fn checked_plus(self, elapsed: Duration) -> Option<Instant> {
        self.0.checked_add(elapsed).map(Instant)
    }

    pub fn elapsed_since(self, earlier: Instant) -> Duration {
        self.0.saturating_sub(earlier.0)
    }

    pub fn has_reached(self, deadline: Instant) -> bool {
        self >= deadline
    }
}
