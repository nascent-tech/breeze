use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct WallClock(u64);

impl WallClock {
    pub fn from_unix_secs(seconds: u64) -> Self {
        WallClock(seconds)
    }

    pub fn as_unix_secs(self) -> u64 {
        self.0
    }

    pub fn saturating_duration_since(self, earlier: WallClock) -> Duration {
        Duration::from_secs(self.0.saturating_sub(earlier.0))
    }
}
