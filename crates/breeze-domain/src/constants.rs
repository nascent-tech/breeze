use core::time::Duration;

pub const SECONDS_PER_MINUTE: u64 = 60;
pub(crate) const MINUTES_PER_HOUR: u16 = 60;
pub(crate) const HOURS_PER_DAY: u16 = 24;
pub const MINUTES_PER_DAY: u16 = MINUTES_PER_HOUR * HOURS_PER_DAY;

pub const NOTICE: Duration = Duration::from_secs(SECONDS_PER_MINUTE);
pub const RETURN_HOLD: Duration = Duration::from_secs(3);
pub const IDLE_FREEZE: Duration = Duration::from_secs(3 * SECONDS_PER_MINUTE);
pub const ESCAPE_HOLD: Duration = Duration::from_secs(10);
pub const HARDCORE_PLACEMENT: Duration = Duration::from_millis(200);
pub const LATE_PLACEMENT: Duration = Duration::from_millis(500);
pub const BREAKER_WINDOW: Duration = Duration::from_secs(5 * SECONDS_PER_MINUTE);
pub const BREAKER_THRESHOLD: u32 = 3;
pub const BREAKER_DISARM: Duration =
    Duration::from_secs(HOURS_PER_DAY as u64 * MINUTES_PER_HOUR as u64 * SECONDS_PER_MINUTE);

pub const WORK_MIN: u16 = 5;
pub const WORK_MAX: u16 = 180;
pub const PAUSE_MIN: u16 = 1;
pub const PAUSE_MAX: u16 = 60;
