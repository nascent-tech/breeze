use breeze_domain::{Instant, WallClock};

pub trait ClockPort {
    fn monotonic(&self) -> Instant;
    fn wall(&self) -> WallClock;
}
