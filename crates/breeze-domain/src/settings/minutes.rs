use crate::constants::SECONDS_PER_MINUTE;
use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Minutes(pub u16);

impl Minutes {
    pub fn count(self) -> u16 {
        self.0
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_secs(u64::from(self.0) * SECONDS_PER_MINUTE)
    }
}
