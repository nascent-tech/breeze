#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct WallClock(u64);

impl WallClock {
    pub fn from_unix_secs(seconds: u64) -> Self {
        WallClock(seconds)
    }

    pub fn as_unix_secs(self) -> u64 {
        self.0
    }
}
