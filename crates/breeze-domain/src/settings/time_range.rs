use crate::constants::MINUTES_PER_DAY;
use crate::settings::rhythm_error::RhythmError;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TimeRange {
    start_minute: u16,
    end_minute: u16,
}

impl TimeRange {
    pub fn from_minutes(start_minute: u16, end_minute: u16) -> Result<Self, RhythmError> {
        let out_of_day = start_minute >= MINUTES_PER_DAY || end_minute >= MINUTES_PER_DAY;
        if out_of_day || start_minute == end_minute {
            return Err(RhythmError::DegenerateRange);
        }
        Ok(TimeRange {
            start_minute,
            end_minute,
        })
    }

    pub fn start(self) -> u16 {
        self.start_minute
    }

    pub fn end(self) -> u16 {
        self.end_minute
    }

    pub fn crosses_midnight(self) -> bool {
        self.end_minute < self.start_minute
    }

    pub fn contains(self, minute_of_day: u16) -> bool {
        if self.crosses_midnight() {
            return minute_of_day >= self.start_minute || minute_of_day < self.end_minute;
        }
        minute_of_day >= self.start_minute && minute_of_day < self.end_minute
    }
}
