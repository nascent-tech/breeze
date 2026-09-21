use breeze_domain::{Instant, WallClock};
use breeze_ports::ClockPort;
use std::time::{Instant as StdInstant, SystemTime, UNIX_EPOCH};

pub struct SystemClock {
    started: StdInstant,
}

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {
            started: StdInstant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        SystemClock::new()
    }
}

impl ClockPort for SystemClock {
    fn monotonic(&self) -> Instant {
        let millis = u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX);
        Instant::at_millis(millis)
    }

    fn wall(&self) -> WallClock {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|since_epoch| since_epoch.as_secs())
            .unwrap_or(0);
        WallClock::from_unix_secs(seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    const JAN_1_2020_UNIX_SECS: u64 = 1_577_836_800;

    #[test]
    fn the_monotonic_reading_advances_with_time() {
        let clock = SystemClock::new();
        let first = clock.monotonic();
        std::thread::sleep(Duration::from_millis(5));
        assert!(clock.monotonic().elapsed_since(first) >= Duration::from_millis(4));
    }

    #[test]
    fn the_wall_reading_is_a_plausible_unix_time() {
        let clock = SystemClock::new();
        assert!(clock.wall().as_unix_secs() > JAN_1_2020_UNIX_SECS);
    }
}
