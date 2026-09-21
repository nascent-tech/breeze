use crate::clock::Instant;
use crate::cycle::break_mode::BreakMode;
use crate::cycle::countdown::Countdown;
use crate::settings::Severity;
use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CycleState {
    Inactive,
    Working {
        countdown: Countdown,
    },
    Notice {
        deadline: Instant,
    },
    BreakActive {
        deadline: Instant,
        severity: Severity,
        mode: BreakMode,
    },
    Returning {
        deadline: Instant,
    },
    Suspended {
        resume_at: Instant,
        frozen: Duration,
    },
}
