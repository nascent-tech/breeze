#![forbid(unsafe_code)]

pub mod breaker;
pub mod clock;
pub mod command_error;
pub mod constants;
pub mod cycle;
pub mod debt;
pub mod outcome;
pub mod settings;

pub use breaker::Breaker;
pub use clock::{ClockJump, Instant, WallClock};
pub use command_error::CommandError;
pub use cycle::{
    absence_verdict, Absence, AbsenceVerdict, BreakMode, Countdown, Cycle, CycleState,
    DegradedReason, FreezeReason,
};
pub use debt::{PostureDebt, Settlement};
pub use outcome::{BreakOutcome, InterruptionDoor};
pub use settings::{
    ActiveDays, AppId, AppStatus, AppStatuses, InvalidAppId, Minutes, NextStart, OffHours, Rhythm,
    RhythmError, SafetyList, Severity, SparedApps, StatusChange, TimeRange, Weekday,
};
