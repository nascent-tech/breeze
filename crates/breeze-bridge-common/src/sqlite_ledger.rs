use crate::sqlite_error::into_error;
use breeze_domain::{BreakOutcome, InterruptionDoor};
use breeze_ports::{LedgerDay, LedgerEntry, PersistenceError};
use rusqlite::{params, Connection};

// v6 : journal des pauses, en ajout seul. CREATE IF NOT EXISTS : migration additive.
// `served_seconds` = temps de pause tenu : la pause entière si servie, la part tenue si
// interrompue, zéro si validée par absence.
pub(crate) fn create_table(connection: &Connection) -> Result<(), PersistenceError> {
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS break_ledger (
                id               INTEGER PRIMARY KEY AUTOINCREMENT,
                ended_at         INTEGER NOT NULL,
                local_date       TEXT NOT NULL,
                outcome          TEXT NOT NULL
                    CHECK (outcome IN ('Served', 'ValidatedByAbsence', 'Interrupted')),
                door             TEXT,
                served_seconds   INTEGER NOT NULL,
                unserved_seconds INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS break_ledger_by_date ON break_ledger (local_date);",
        )
        .map_err(into_error)
}

pub(crate) fn record(connection: &Connection, entry: &LedgerEntry) -> Result<(), PersistenceError> {
    let (outcome, door, unserved) = columns_of(entry.outcome);
    connection
        .execute(
            "INSERT INTO break_ledger
                (ended_at, local_date, outcome, door, served_seconds, unserved_seconds)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                entry.ended_at_unix,
                entry.local_date,
                outcome,
                door,
                entry.outcome.held().as_secs(),
                unserved,
            ],
        )
        .map(|_| ())
        .map_err(into_error)
}

pub(crate) fn days_from(
    connection: &Connection,
    from_date: &str,
) -> Result<Vec<LedgerDay>, PersistenceError> {
    let mut statement = connection
        .prepare(
            "SELECT local_date,
                    COALESCE(SUM(outcome = 'Served'), 0),
                    COALESCE(SUM(outcome = 'Interrupted'), 0),
                    COALESCE(SUM(outcome = 'ValidatedByAbsence'), 0),
                    COALESCE(SUM(served_seconds), 0),
                    COALESCE(SUM(door = 'HardcoreExitGesture'), 0),
                    COALESCE(SUM(CASE WHEN door = 'HardcoreExitGesture'
                                      THEN unserved_seconds END), 0)
             FROM break_ledger
             WHERE local_date >= ?1
             GROUP BY local_date
             ORDER BY local_date",
        )
        .map_err(into_error)?;
    let rows = statement
        .query_map(params![from_date], |row| {
            Ok(LedgerDay {
                local_date: row.get(0)?,
                served: row.get(1)?,
                interrupted: row.get(2)?,
                validated: row.get(3)?,
                served_seconds: row.get(4)?,
                emergency_exits: row.get(5)?,
                emergency_unserved_seconds: row.get(6)?,
            })
        })
        .map_err(into_error)?;
    rows.collect::<Result<_, _>>().map_err(into_error)
}

fn columns_of(outcome: BreakOutcome) -> (&'static str, Option<&'static str>, u64) {
    match outcome {
        BreakOutcome::Served { .. } => ("Served", None, 0),
        BreakOutcome::ValidatedByAbsence => ("ValidatedByAbsence", None, 0),
        BreakOutcome::Interrupted { unserved, door, .. } => {
            ("Interrupted", Some(door_name(door)), unserved.as_secs())
        }
    }
}

fn door_name(door: InterruptionDoor) -> &'static str {
    match door {
        InterruptionDoor::Quit => "Quit",
        InterruptionDoor::TrayMenu => "TrayMenu",
        InterruptionDoor::HardcoreExitGesture => "HardcoreExitGesture",
        InterruptionDoor::SuspensionOverrun => "SuspensionOverrun",
        InterruptionDoor::Crash => "Crash",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    const TEN_MIN: Duration = Duration::from_secs(600);

    fn ledger() -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        create_table(&connection).unwrap();
        connection
    }

    fn entry(date: &str, outcome: BreakOutcome) -> LedgerEntry {
        LedgerEntry {
            ended_at_unix: 1_790_000_000,
            local_date: date.to_owned(),
            outcome,
        }
    }

    fn served() -> BreakOutcome {
        BreakOutcome::Served { planned: TEN_MIN }
    }

    fn emergency_exit(unserved_secs: u64) -> BreakOutcome {
        BreakOutcome::Interrupted {
            planned: TEN_MIN,
            unserved: Duration::from_secs(unserved_secs),
            door: InterruptionDoor::HardcoreExitGesture,
        }
    }

    #[test]
    fn an_empty_ledger_has_no_day() {
        assert_eq!(days_from(&ledger(), "2026-01-01"), Ok(Vec::new()));
    }

    #[test]
    fn outcomes_are_counted_per_local_day() {
        let connection = ledger();
        let rows = [
            entry("2026-09-25", served()),
            entry("2026-09-26", served()),
            entry("2026-09-26", served()),
            entry("2026-09-26", BreakOutcome::ValidatedByAbsence),
            entry("2026-09-26", emergency_exit(420)),
        ];
        for row in &rows {
            record(&connection, row).unwrap();
        }

        let days = days_from(&connection, "2026-09-01").unwrap();

        assert_eq!(days.len(), 2);
        assert_eq!(days[0].local_date, "2026-09-25");
        assert_eq!(
            days[1],
            LedgerDay {
                local_date: "2026-09-26".to_owned(),
                served: 2,
                interrupted: 1,
                validated: 1,
                // Deux pauses servies entières + les 3 min tenues de la pause interrompue.
                served_seconds: 2 * 600 + 180,
                emergency_exits: 1,
                emergency_unserved_seconds: 420,
            }
        );
    }

    #[test]
    fn an_interruption_by_quitting_is_not_an_emergency_exit() {
        let connection = ledger();
        let quit = BreakOutcome::Interrupted {
            planned: TEN_MIN,
            unserved: TEN_MIN,
            door: InterruptionDoor::Quit,
        };
        record(&connection, &entry("2026-09-26", quit)).unwrap();

        let day = &days_from(&connection, "2026-09-26").unwrap()[0];

        assert_eq!(day.interrupted, 1);
        assert_eq!(day.emergency_exits, 0);
        assert_eq!(day.emergency_unserved_seconds, 0);
    }

    #[test]
    fn days_before_the_window_are_left_out() {
        let connection = ledger();
        record(&connection, &entry("2026-08-01", served())).unwrap();
        record(&connection, &entry("2026-09-26", served())).unwrap();

        let days = days_from(&connection, "2026-08-28").unwrap();

        assert_eq!(days.len(), 1);
        assert_eq!(days[0].local_date, "2026-09-26");
    }
}
