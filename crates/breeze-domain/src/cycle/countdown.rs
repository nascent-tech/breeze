use crate::clock::Instant;
use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Countdown {
    Running { deadline: Instant },
    Frozen { remaining: Duration },
    Due { since: Instant },
}
