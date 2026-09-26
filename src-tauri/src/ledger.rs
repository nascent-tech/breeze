use crate::local_time::{iso_date, local_now};
use crate::{lock, AppState};
use breeze_domain::constants::SECONDS_PER_MINUTE;
use breeze_ports::LedgerDay;
use chrono::{Days, NaiveDate};
use serde::Serialize;
use tauri::State;

const STATS_DAYS: u64 = 30;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct StatsDayDto {
    pub date: String,
    pub served: u32,
    pub interrupted: u32,
    pub validated: u32,
    pub served_minutes: u32,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct StatsDto {
    pub days: Vec<StatsDayDto>,
    pub served: u32,
    pub interrupted: u32,
    pub validated: u32,
    pub due: u32,
    pub served_minutes: u32,
    pub emergency_exits: u32,
    pub emergency_minutes: u32,
    pub debt_minutes: u32,
}

impl AppState {
    pub(crate) fn served_on(&self, date: NaiveDate) -> u32 {
        let iso = iso_date(date);
        match self.persistence.ledger_days(&iso) {
            Ok(days) => days
                .iter()
                .find(|day| day.local_date == iso)
                .map_or(0, |day| day.served),
            Err(error) => {
                eprintln!("breeze: could not read the break journal: {}", error.0);
                0
            }
        }
    }
}

fn minutes_of(seconds: u64) -> u32 {
    u32::try_from(seconds.saturating_add(SECONDS_PER_MINUTE / 2) / SECONDS_PER_MINUTE)
        .unwrap_or(u32::MAX)
}

fn first_day(today: NaiveDate) -> NaiveDate {
    today
        .checked_sub_days(Days::new(STATS_DAYS - 1))
        .unwrap_or(today)
}

// Un jour sans ligne au journal est un jour à zéro, jamais un jour absent.
fn day_of(rows: &[LedgerDay], date: NaiveDate) -> LedgerDay {
    let local_date = iso_date(date);
    rows.iter()
        .find(|row| row.local_date == local_date)
        .cloned()
        .unwrap_or(LedgerDay {
            local_date,
            ..LedgerDay::default()
        })
}

impl From<LedgerDay> for StatsDayDto {
    fn from(day: LedgerDay) -> Self {
        StatsDayDto {
            served_minutes: minutes_of(day.served_seconds),
            date: day.local_date,
            served: day.served,
            interrupted: day.interrupted,
            validated: day.validated,
        }
    }
}

pub(crate) fn build_stats(today: NaiveDate, rows: &[LedgerDay], debt_minutes: u16) -> StatsDto {
    let window: Vec<LedgerDay> = first_day(today)
        .iter_days()
        .take_while(|date| *date <= today)
        .map(|date| day_of(rows, date))
        .collect();
    let count = |pick: fn(&LedgerDay) -> u32| window.iter().map(pick).sum::<u32>();
    let seconds = |pick: fn(&LedgerDay) -> u64| window.iter().map(pick).sum::<u64>();
    let (served, interrupted, validated) = (
        count(|day| day.served),
        count(|day| day.interrupted),
        count(|day| day.validated),
    );
    StatsDto {
        served,
        interrupted,
        validated,
        due: served + interrupted + validated,
        served_minutes: minutes_of(seconds(|day| day.served_seconds)),
        emergency_exits: count(|day| day.emergency_exits),
        emergency_minutes: minutes_of(seconds(|day| day.emergency_unserved_seconds)),
        debt_minutes: u32::from(debt_minutes),
        days: window.into_iter().map(StatsDayDto::from).collect(),
    }
}

#[tauri::command]
pub fn get_stats(state: State<'_, AppState>) -> Result<StatsDto, String> {
    let today = local_now().date;
    let rows = state
        .persistence
        .ledger_days(&iso_date(first_day(today)))
        .map_err(|error| {
            eprintln!("breeze: could not read the break journal: {}", error.0);
            "persistence-failed".to_owned()
        })?;
    let debt_minutes = lock(&state.scheduler).debt().minutes();
    Ok(build_stats(today, &rows, debt_minutes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, day).unwrap()
    }

    fn row(day: u32, served: u32, served_minutes: u64) -> LedgerDay {
        LedgerDay {
            local_date: iso_date(date(day)),
            served,
            served_seconds: served_minutes * SECONDS_PER_MINUTE,
            ..LedgerDay::default()
        }
    }

    #[test]
    fn the_stats_span_thirty_days_ending_today() {
        let stats = build_stats(date(26), &[], 0);
        assert_eq!(stats.days.len(), 30);
        assert_eq!(stats.days[0].date, "2026-08-28");
        assert_eq!(stats.days[29].date, "2026-09-26");
        assert!(stats.days.iter().all(|day| day.served == 0));
    }

    #[test]
    fn days_and_totals_add_up() {
        let mut interrupted = row(26, 1, 10);
        interrupted.interrupted = 1;
        interrupted.emergency_exits = 1;
        interrupted.emergency_unserved_seconds = 7 * SECONDS_PER_MINUTE;
        let stats = build_stats(date(26), &[row(20, 3, 30), interrupted], 12);

        assert_eq!(stats.days[23].served, 3);
        assert_eq!(stats.days[23].served_minutes, 30);
        assert_eq!(stats.days[29].interrupted, 1);
        assert_eq!(stats.served, 4);
        assert_eq!(stats.interrupted, 1);
        assert_eq!(stats.due, 5);
        assert_eq!(stats.served_minutes, 40);
        assert_eq!(stats.emergency_exits, 1);
        assert_eq!(stats.emergency_minutes, 7);
        assert_eq!(stats.debt_minutes, 12);
    }
}
