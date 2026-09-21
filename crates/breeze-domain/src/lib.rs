#![forbid(unsafe_code)]

pub mod breaker;
pub mod clock;
pub mod command_error;
pub mod constants;
pub mod cycle;
pub mod outcome;
pub mod settings;

pub use breaker::Breaker;
pub use clock::{ClockJump, Instant, WallClock};
pub use command_error::CommandError;
pub use cycle::{
    absence_verdict, Absence, AbsenceVerdict, BreakMode, Countdown, Cycle, CycleState,
};
pub use outcome::{BreakOutcome, InterruptionDoor};
pub use settings::{
    ActiveDays, AppStatus, Minutes, Rhythm, RhythmError, Severity, TimeRange, Weekday,
};
