use crate::sqlite_app_statuses;
use crate::sqlite_error::into_error;
use crate::sqlite_ledger;
use crate::sqlite_meta;
use crate::sqlite_schema;
use breeze_domain::{AppId, AppStatus, Severity};
use breeze_ports::{LedgerDay, LedgerEntry, PersistedState, PersistenceError, PersistencePort};
use core::time::Duration;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const EVERYDAY_MASK: i64 = 0b0111_1111;

pub struct SqliteStore {
    connection: Mutex<Connection>,
}

impl SqliteStore {
    pub fn open(path: &Path) -> Result<Self, PersistenceError> {
        Self::from_connection(Connection::open(path).map_err(into_error)?)
    }

    pub fn in_memory() -> Result<Self, PersistenceError> {
        Self::from_connection(Connection::open_in_memory().map_err(into_error)?)
    }

    fn from_connection(mut connection: Connection) -> Result<Self, PersistenceError> {
        connection.busy_timeout(BUSY_TIMEOUT).map_err(into_error)?;
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(into_error)?;
        sqlite_schema::migrate(&mut connection)?;
        Ok(SqliteStore {
            connection: Mutex::new(connection),
        })
    }

    fn lock(&self) -> MutexGuard<'_, Connection> {
        self.connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

fn write_state(connection: &Connection, state: PersistedState) -> Result<(), PersistenceError> {
    connection
        .execute(
            "INSERT INTO state
                (id, work_minutes, pause_minutes, severity, served_breaks, debt_seconds,
                 debt_recorded_at, active_days, schedule_start, schedule_end)
                VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET
                work_minutes = ?1, pause_minutes = ?2, severity = ?3, served_breaks = ?4,
                debt_seconds = ?5, debt_recorded_at = ?6, active_days = ?7,
                schedule_start = ?8, schedule_end = ?9",
            params![
                state.work_minutes,
                state.pause_minutes,
                severity_name(state.severity),
                state.served_breaks,
                state.debt_seconds,
                state.debt_recorded_at_unix,
                state.active_days,
                state.schedule_start,
                state.schedule_end,
            ],
        )
        .map(|_| ())
        .map_err(into_error)
}

fn severity_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Simple => "Simple",
        Severity::Hardcore => "Hardcore",
    }
}

fn severity_from(name: &str) -> Severity {
    match name {
        "Hardcore" => Severity::Hardcore,
        _ => Severity::Simple,
    }
}

impl PersistencePort for SqliteStore {
    fn load(&self) -> Result<Option<PersistedState>, PersistenceError> {
        let connection = self.lock();
        let row = connection.query_row(
            "SELECT work_minutes, pause_minutes, severity, served_breaks, debt_seconds, debt_recorded_at,
                    active_days, schedule_start, schedule_end
             FROM state WHERE id = 1",
            [],
            |row| {
                let active_days: i64 = row.get(6)?;
                Ok(PersistedState {
                    work_minutes: row.get(0)?,
                    pause_minutes: row.get(1)?,
                    severity: severity_from(&row.get::<_, String>(2)?),
                    served_breaks: row.get(3)?,
                    debt_seconds: row.get(4)?,
                    debt_recorded_at_unix: row.get(5)?,
                    active_days: u8::try_from(active_days & EVERYDAY_MASK).unwrap_or(0b0111_1111),
                    schedule_start: row.get::<_, Option<u16>>(7)?,
                    schedule_end: row.get::<_, Option<u16>>(8)?,
                })
            },
        );
        match row {
            Ok(state) => Ok(Some(state)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(into_error(error)),
        }
    }

    fn save(&self, state: PersistedState) -> Result<(), PersistenceError> {
        write_state(&self.lock(), state)
    }

    fn reset_preferences(&self, state: PersistedState) -> Result<(), PersistenceError> {
        let mut connection = self.lock();
        let transaction = connection.transaction().map_err(into_error)?;
        write_state(&transaction, state)?;
        sqlite_app_statuses::clear(&transaction)?;
        sqlite_meta::forget_preferences(&transaction)?;
        transaction.commit().map_err(into_error)
    }

    fn load_app_statuses(&self) -> Result<Vec<(AppId, AppStatus)>, PersistenceError> {
        let connection = self.lock();
        sqlite_app_statuses::load(&connection)
    }

    fn replace_app_statuses(
        &self,
        statuses: &[(AppId, AppStatus)],
    ) -> Result<(), PersistenceError> {
        let mut connection = self.lock();
        sqlite_app_statuses::replace(&mut connection, statuses)
    }

    fn is_onboarding_done(&self) -> Result<bool, PersistenceError> {
        let connection = self.lock();
        sqlite_meta::is_onboarding_done(&connection)
    }

    fn mark_onboarding_done(&self) -> Result<(), PersistenceError> {
        let connection = self.lock();
        sqlite_meta::mark_onboarding_done(&connection)
    }

    fn is_update_check_enabled(&self) -> Result<bool, PersistenceError> {
        let connection = self.lock();
        sqlite_meta::is_update_check_enabled(&connection)
    }

    fn set_update_check(&self, enabled: bool) -> Result<(), PersistenceError> {
        let connection = self.lock();
        sqlite_meta::set_update_check(&connection, enabled)
    }

    fn flag(&self, key: &str) -> Result<Option<bool>, PersistenceError> {
        let connection = self.lock();
        sqlite_meta::flag(&connection, key)
    }

    fn set_flag(&self, key: &str, value: bool) -> Result<(), PersistenceError> {
        let connection = self.lock();
        sqlite_meta::set_flag(&connection, key, value)
    }

    fn remembered_schedule(&self) -> Result<Option<(u16, u16)>, PersistenceError> {
        let connection = self.lock();
        sqlite_meta::remembered_schedule(&connection)
    }

    fn remember_schedule(&self, start: u16, end: u16) -> Result<(), PersistenceError> {
        let connection = self.lock();
        sqlite_meta::remember_schedule(&connection, start, end)
    }

    fn record_break(&self, entry: &LedgerEntry) -> Result<(), PersistenceError> {
        let connection = self.lock();
        sqlite_ledger::record(&connection, entry)
    }

    fn ledger_days(&self, from_date: &str) -> Result<Vec<LedgerDay>, PersistenceError> {
        let connection = self.lock();
        sqlite_ledger::days_from(&connection, from_date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_store_holds_nothing() {
        let store = SqliteStore::in_memory().unwrap();
        assert_eq!(store.load(), Ok(None));
    }

    #[test]
    fn it_saves_and_reads_the_same_state_back() {
        let store = SqliteStore::in_memory().unwrap();
        let state = PersistedState {
            work_minutes: 50,
            pause_minutes: 10,
            severity: Severity::Hardcore,
            served_breaks: 3,
            debt_seconds: 450,
            debt_recorded_at_unix: 1_700_000_000,
            active_days: 0b0001_1111,
            schedule_start: Some(540),
            schedule_end: Some(1110),
        };
        store.save(state).unwrap();
        assert_eq!(store.load(), Ok(Some(state)));
    }

    #[test]
    fn a_second_save_replaces_the_first() {
        let store = SqliteStore::in_memory().unwrap();
        store
            .save(PersistedState {
                work_minutes: 25,
                pause_minutes: 5,
                severity: Severity::Simple,
                served_breaks: 1,
                debt_seconds: 0,
                debt_recorded_at_unix: 0,
                active_days: 0b0111_1111,
                schedule_start: None,
                schedule_end: None,
            })
            .unwrap();
        let updated = PersistedState {
            work_minutes: 50,
            pause_minutes: 10,
            severity: Severity::Hardcore,
            served_breaks: 4,
            debt_seconds: 120,
            debt_recorded_at_unix: 1_700_000_500,
            active_days: 0b0011_0000,
            schedule_start: Some(480),
            schedule_end: Some(1020),
        };
        store.save(updated).unwrap();
        assert_eq!(store.load(), Ok(Some(updated)));
    }

    #[test]
    fn a_v1_database_migrates_and_defaults_its_debt_to_zero() {
        // Une base au schéma 1 : table sans colonnes de dette, une ligne, user_version = 1.
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute(
                "CREATE TABLE state (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    work_minutes INTEGER NOT NULL,
                    pause_minutes INTEGER NOT NULL,
                    severity TEXT NOT NULL,
                    served_breaks INTEGER NOT NULL
                )",
                [],
            )
            .unwrap();
        connection
            .execute("INSERT INTO state VALUES (1, 50, 10, 'Hardcore', 7)", [])
            .unwrap();
        connection
            .pragma_update(None, "user_version", 1i64)
            .unwrap();

        let store = SqliteStore::from_connection(connection).unwrap();

        assert_eq!(
            store.load(),
            Ok(Some(PersistedState {
                work_minutes: 50,
                pause_minutes: 10,
                severity: Severity::Hardcore,
                served_breaks: 7,
                debt_seconds: 0,
                debt_recorded_at_unix: 0,
                active_days: 0b0111_1111,
                schedule_start: None,
                schedule_end: None,
            }))
        );
    }

    fn app(raw: &str) -> AppId {
        AppId::parse(raw).unwrap()
    }

    #[test]
    fn a_fresh_store_has_no_app_statuses() {
        let store = SqliteStore::in_memory().unwrap();
        assert_eq!(store.load_app_statuses(), Ok(Vec::new()));
    }

    #[test]
    fn it_saves_and_reads_app_statuses_back() {
        let store = SqliteStore::in_memory().unwrap();
        let statuses = vec![
            (app("com.apple.Music"), AppStatus::Spared),
            (app("com.tinyspeck.slackmacgap"), AppStatus::Ignored),
        ];
        store.replace_app_statuses(&statuses).unwrap();
        let mut loaded = store.load_app_statuses().unwrap();
        loaded.sort_by(|a, b| a.0.as_str().cmp(b.0.as_str()));
        let mut expected = statuses;
        expected.sort_by(|a, b| a.0.as_str().cmp(b.0.as_str()));
        assert_eq!(loaded, expected);
    }

    #[test]
    fn replacing_app_statuses_drops_the_previous_set() {
        let store = SqliteStore::in_memory().unwrap();
        store
            .replace_app_statuses(&[(app("com.apple.Music"), AppStatus::Spared)])
            .unwrap();
        store
            .replace_app_statuses(&[(app("com.apple.Notes"), AppStatus::Ignored)])
            .unwrap();
        assert_eq!(
            store.load_app_statuses(),
            Ok(vec![(app("com.apple.Notes"), AppStatus::Ignored)])
        );
    }

    #[test]
    fn a_blocked_status_is_never_stored() {
        let store = SqliteStore::in_memory().unwrap();
        store
            .replace_app_statuses(&[
                (app("com.apple.Music"), AppStatus::Blocked),
                (app("com.apple.Notes"), AppStatus::Spared),
            ])
            .unwrap();
        assert_eq!(
            store.load_app_statuses(),
            Ok(vec![(app("com.apple.Notes"), AppStatus::Spared)])
        );
    }

    #[test]
    fn a_v5_database_gains_an_empty_break_ledger_and_keeps_its_state() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE state (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    work_minutes INTEGER NOT NULL,
                    pause_minutes INTEGER NOT NULL,
                    severity TEXT NOT NULL,
                    served_breaks INTEGER NOT NULL,
                    debt_seconds INTEGER NOT NULL DEFAULT 0,
                    debt_recorded_at INTEGER NOT NULL DEFAULT 0,
                    active_days INTEGER NOT NULL DEFAULT 127,
                    schedule_start INTEGER,
                    schedule_end INTEGER
                );
                INSERT INTO state VALUES (1, 50, 10, 'Simple', 4, 0, 0, 31, 540, 1080);
                PRAGMA user_version = 5;",
            )
            .unwrap();

        let store = SqliteStore::from_connection(connection).unwrap();

        assert_eq!(store.ledger_days("2000-01-01"), Ok(Vec::new()));
        store
            .record_break(&LedgerEntry {
                ended_at_unix: 1_790_000_000,
                local_date: "2026-09-26".to_owned(),
                outcome: breeze_domain::BreakOutcome::Served {
                    planned: Duration::from_secs(600),
                },
            })
            .unwrap();
        assert_eq!(store.ledger_days("2026-09-26").unwrap()[0].served, 1);
        assert_eq!(
            store.load().unwrap().map(|state| state.served_breaks),
            Some(4)
        );
    }

    #[test]
    fn a_v2_database_gains_the_app_statuses_table() {
        // Base au schéma 2 : table state complète, user_version = 2, pas de app_statuses.
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute(
                "CREATE TABLE state (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    work_minutes INTEGER NOT NULL,
                    pause_minutes INTEGER NOT NULL,
                    severity TEXT NOT NULL,
                    served_breaks INTEGER NOT NULL,
                    debt_seconds INTEGER NOT NULL DEFAULT 0,
                    debt_recorded_at INTEGER NOT NULL DEFAULT 0
                )",
                [],
            )
            .unwrap();
        connection
            .pragma_update(None, "user_version", 2i64)
            .unwrap();

        let store = SqliteStore::from_connection(connection).unwrap();

        assert_eq!(store.load_app_statuses(), Ok(Vec::new()));
        store
            .replace_app_statuses(&[(app("com.apple.Music"), AppStatus::Spared)])
            .unwrap();
        assert_eq!(
            store.load_app_statuses(),
            Ok(vec![(app("com.apple.Music"), AppStatus::Spared)])
        );
    }

    #[test]
    fn a_reset_writes_the_state_and_forgets_every_preference_but_onboarding() {
        let store = SqliteStore::in_memory().unwrap();
        store.mark_onboarding_done().unwrap();
        store.set_update_check(false).unwrap();
        store.set_flag("sounds", false).unwrap();
        store
            .replace_app_statuses(&[(app("com.apple.Music"), AppStatus::Ignored)])
            .unwrap();
        let defaults = PersistedState {
            work_minutes: 50,
            pause_minutes: 10,
            severity: Severity::Simple,
            served_breaks: 3,
            debt_seconds: 0,
            debt_recorded_at_unix: 1_790_000_000,
            active_days: 0b0111_1111,
            schedule_start: None,
            schedule_end: None,
        };

        store.reset_preferences(defaults).unwrap();

        assert_eq!(store.load(), Ok(Some(defaults)));
        assert_eq!(store.load_app_statuses(), Ok(Vec::new()));
        assert_eq!(store.is_update_check_enabled(), Ok(true));
        assert_eq!(store.flag("sounds"), Ok(None));
        assert_eq!(store.is_onboarding_done(), Ok(true));
    }
}
