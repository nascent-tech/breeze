use breeze_domain::{NextStart, Weekday};
use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};

const MINUTES_PER_HOUR: u16 = 60;

// Lecture unique de l'heure locale : jour, jour de semaine et minute tirés du même instant.
#[derive(Clone, Copy, Debug)]
pub struct LocalNow {
    pub date: NaiveDate,
    pub weekday: Weekday,
    pub minute_of_day: u16,
}

pub fn local_now() -> LocalNow {
    let now = Local::now();
    let days_from_monday = u8::try_from(now.weekday().num_days_from_monday()).unwrap_or(0);
    let minute_of_day =
        u16::try_from(now.hour() * u32::from(MINUTES_PER_HOUR) + now.minute()).unwrap_or(0);
    LocalNow {
        date: now.date_naive(),
        weekday: Weekday::from_days_from_monday(days_from_monday),
        minute_of_day,
    }
}

pub fn local_date_of(unix_secs: u64) -> Option<NaiveDate> {
    let secs = i64::try_from(unix_secs).ok()?;
    let utc = DateTime::from_timestamp(secs, 0)?;
    Some(utc.with_timezone(&Local).date_naive())
}

pub fn iso_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

fn day_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "lundi",
        Weekday::Tuesday => "mardi",
        Weekday::Wednesday => "mercredi",
        Weekday::Thursday => "jeudi",
        Weekday::Friday => "vendredi",
        Weekday::Saturday => "samedi",
        Weekday::Sunday => "dimanche",
    }
}

fn clock_label(minute_of_day: u16) -> String {
    format!(
        "{}:{:02}",
        minute_of_day / MINUTES_PER_HOUR,
        minute_of_day % MINUTES_PER_HOUR
    )
}

pub fn next_start_label(next: NextStart) -> String {
    let clock = clock_label(next.minute_of_day);
    match next.days_ahead {
        0 => clock,
        1 => format!("demain {clock}"),
        // Une semaine plus tard : le même jour de semaine qu'aujourd'hui, dit sans ambiguïté.
        7 => format!("{} prochain {clock}", day_name(next.weekday)),
        _ => format!("{} {clock}", day_name(next.weekday)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next(days_ahead: u8, weekday: Weekday, minute_of_day: u16) -> NextStart {
        NextStart {
            days_ahead,
            weekday,
            minute_of_day,
        }
    }

    #[test]
    fn a_start_later_today_shows_only_the_time() {
        assert_eq!(next_start_label(next(0, Weekday::Friday, 9 * 60)), "9:00");
    }

    #[test]
    fn a_start_tomorrow_says_demain() {
        assert_eq!(
            next_start_label(next(1, Weekday::Saturday, 8 * 60 + 30)),
            "demain 8:30"
        );
    }

    #[test]
    fn a_later_start_names_the_day_in_lowercase_french() {
        assert_eq!(
            next_start_label(next(3, Weekday::Monday, 9 * 60)),
            "lundi 9:00"
        );
    }

    #[test]
    fn a_start_a_week_away_says_prochain() {
        assert_eq!(
            next_start_label(next(7, Weekday::Monday, 9 * 60)),
            "lundi prochain 9:00"
        );
    }

    #[test]
    fn midnight_reads_zero_hundred() {
        assert_eq!(
            next_start_label(next(2, Weekday::Sunday, 0)),
            "dimanche 0:00"
        );
    }

    #[test]
    fn dates_are_iso_formatted() {
        let date = NaiveDate::from_ymd_opt(2026, 9, 6).unwrap();
        assert_eq!(iso_date(date), "2026-09-06");
    }
}
