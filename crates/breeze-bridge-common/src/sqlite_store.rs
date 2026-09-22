use crate::sqlite_app_statuses;
use crate::sqlite_error::into_error;
use crate::sqlite_meta;
use breeze_domain::{AppId, AppStatus, Severity};
use breeze_ports::{PersistedState, PersistenceError, PersistencePort};
use core::time::Duration;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Mutex, PoisonError};

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const SCHEMA_VERSION: i64 = 5;
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

    fn from_connection(connection: Connection) -> Result<Self, PersistenceError> {
        connection.busy_timeout(BUSY_TIMEOUT).map_err(into_error)?;
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(into_error)?;
        let version: i64 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(into_error)?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS state (
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
                )",
                [],
            )
            .map_err(into_error)?;
        // Une base au schéma 1 a une table sans les colonnes de dette : on les ajoute.
        if version == 1 {
            migrate_v1_to_v2(&connection)?;
        }
        // v5 : plage horaire + jours actifs persistés. Une base existante (v1..v4) a la
        // table sans ces colonnes ; une base fraîche (v0) les a déjà par le CREATE.
        if (1..5).contains(&version) {
            migrate_add_schedule_columns(&connection)?;
        }
        // v3 : table des statuts d'apps. CREATE IF NOT EXISTS est atomique et sûr
        // pour toute version antérieure (aucune donnée existante à transformer).
        sqlite_app_statuses::create_table(&connection)?;
        // v4 : magasin de drapeaux (onboarding_done, etc.).
        sqlite_meta::create_table(&connection)?;
        connection
            .pragma_update(None, "user_version", SCHEMA_VERSION)
            .map_err(into_error)?;
        Ok(SqliteStore {
            connection: Mutex::new(connection),
        })
    }
}

fn migrate_v1_to_v2(connection: &Connection) -> Result<(), PersistenceError> {
    // Atomique : les deux colonnes, ou aucune. Un échec laisse la base en v1, réessayable
    // au prochain démarrage, plutôt qu'à moitié migrée (le second ADD échouerait alors en boucle).
    connection
        .execute_batch(
            "BEGIN;
             ALTER TABLE state ADD COLUMN debt_seconds INTEGER NOT NULL DEFAULT 0;
             ALTER TABLE state ADD COLUMN debt_recorded_at INTEGER NOT NULL DEFAULT 0;
             COMMIT;",
        )
        .map_err(into_error)
}

fn migrate_add_schedule_columns(connection: &Connection) -> Result<(), PersistenceError> {
    // Atomique : les trois colonnes, ou aucune. Défaut : tous les jours, plage désactivée.
    connection
        .execute_batch(
            "BEGIN;
             ALTER TABLE state ADD COLUMN active_days INTEGER NOT NULL DEFAULT 127;
             ALTER TABLE state ADD COLUMN schedule_start INTEGER;
             ALTER TABLE state ADD COLUMN schedule_end INTEGER;
             COMMIT;",
        )
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
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
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
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
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

    fn load_app_statuses(&self) -> Result<Vec<(AppId, AppStatus)>, PersistenceError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        sqlite_app_statuses::load(&connection)
    }

    fn replace_app_statuses(
        &self,
        statuses: &[(AppId, AppStatus)],
    ) -> Result<(), PersistenceError> {
        let mut connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        sqlite_app_statuses::replace(&mut connection, statuses)
    }

    fn is_onboarding_done(&self) -> Result<bool, PersistenceError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        sqlite_meta::is_onboarding_done(&connection)
    }

    fn mark_onboarding_done(&self) -> Result<(), PersistenceError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        sqlite_meta::mark_onboarding_done(&connection)
    }

    fn is_update_check_enabled(&self) -> Result<bool, PersistenceError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        sqlite_meta::is_update_check_enabled(&connection)
    }

    fn set_update_check(&self, enabled: bool) -> Result<(), PersistenceError> {
        let connection = self
            .connection
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        sqlite_meta::set_update_check(&connection, enabled)
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
}
