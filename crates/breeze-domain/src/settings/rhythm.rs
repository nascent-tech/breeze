use crate::constants::{PAUSE_MAX, PAUSE_MIN, WORK_MAX, WORK_MIN};
use crate::settings::active_days::ActiveDays;
use crate::settings::minutes::Minutes;
use crate::settings::rhythm_error::RhythmError;
use crate::settings::time_range::TimeRange;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rhythm {
    work: Minutes,
    pause: Minutes,
    schedule: Option<TimeRange>,
    active_days: ActiveDays,
}

impl Rhythm {
    pub fn new(
        work: Minutes,
        pause: Minutes,
        schedule: Option<TimeRange>,
        active_days: ActiveDays,
    ) -> Result<Self, RhythmError> {
        if !(WORK_MIN..=WORK_MAX).contains(&work.count()) {
            return Err(RhythmError::WorkOutOfBounds);
        }
        if !(PAUSE_MIN..=PAUSE_MAX).contains(&pause.count()) {
            return Err(RhythmError::PauseOutOfBounds);
        }
        if pause.count() > work.count() {
            return Err(RhythmError::PauseLongerThanWork);
        }
        Ok(Rhythm {
            work,
            pause,
            schedule,
            active_days,
        })
    }

    pub fn work(self) -> Minutes {
        self.work
    }

    pub fn pause(self) -> Minutes {
        self.pause
    }

    pub fn schedule(self) -> Option<TimeRange> {
        self.schedule
    }

    pub fn active_days(self) -> ActiveDays {
        self.active_days
    }
}
