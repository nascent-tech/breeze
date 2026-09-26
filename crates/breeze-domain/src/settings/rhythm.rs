use crate::constants::{
    MINUTES_PER_DAY, PAUSE_MAX, PAUSE_MIN, SECONDS_PER_MINUTE, WORK_MAX, WORK_MIN,
};
use crate::settings::active_days::ActiveDays;
use crate::settings::minutes::Minutes;
use crate::settings::next_start::NextStart;
use crate::settings::off_hours::OffHours;
use crate::settings::rhythm_error::RhythmError;
use crate::settings::time_range::TimeRange;
use crate::settings::weekday::Weekday;
use core::time::Duration;

const DAYS_SCANNED: u8 = 7;
const DAYS_PER_WEEK: u64 = 7;

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

    pub fn is_active_at(self, weekday: Weekday, minute_of_day: u16) -> bool {
        self.off_hours_at(weekday, minute_of_day).is_none()
    }

    // Une plage qui franchit minuit appartient au jour où elle commence : 1 h du matin
    // un mardi, dans une plage 22 h–2 h, relève de la session du lundi.
    pub fn off_hours_at(self, weekday: Weekday, minute_of_day: u16) -> Option<OffHours> {
        let session_day = match self.schedule {
            Some(range) if range.crosses_midnight() && minute_of_day < range.end() => {
                weekday.previous()
            }
            _ => weekday,
        };
        if !self.active_days.contains(session_day) {
            return Some(OffHours::InactiveDay);
        }
        match self.schedule {
            Some(range) if !range.contains(minute_of_day) => Some(OffHours::OutsideSchedule),
            _ => None,
        }
    }

    // Les heures actives couvrent-elles, minute par minute, tout l'intervalle qui part de
    // l'instant donné ? Au-delà d'une semaine, le motif se répète : une semaine suffit.
    pub fn is_active_throughout(
        self,
        weekday: Weekday,
        minute_of_day: u16,
        span: Duration,
    ) -> bool {
        let minutes_per_day = u64::from(MINUTES_PER_DAY);
        let span_minutes =
            (span.as_secs() / SECONDS_PER_MINUTE).min(minutes_per_day * DAYS_PER_WEEK);
        (0..=span_minutes).all(|offset| {
            let total = u64::from(minute_of_day) + offset;
            let days_ahead = u8::try_from(total / minutes_per_day % DAYS_PER_WEEK).unwrap_or(0);
            let minute = u16::try_from(total % minutes_per_day).unwrap_or(0);
            self.is_active_at(weekday.plus_days(days_ahead), minute)
        })
    }

    // Prochain début de session strictement après l'instant donné. Toujours défini :
    // au moins un jour est actif, et une semaine plus tard revient au même jour.
    pub fn next_start_after(self, weekday: Weekday, minute_of_day: u16) -> NextStart {
        let start = self.schedule.map_or(0, TimeRange::start);
        let days_ahead = (0..DAYS_SCANNED)
            .find(|&ahead| {
                let later_today = ahead > 0 || start > minute_of_day;
                later_today && self.active_days.contains(weekday.plus_days(ahead))
            })
            .unwrap_or(DAYS_SCANNED);
        NextStart {
            days_ahead,
            weekday: weekday.plus_days(days_ahead),
            minute_of_day: start,
        }
    }
}
