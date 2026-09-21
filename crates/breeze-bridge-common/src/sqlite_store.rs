use breeze_domain::Severity;
use breeze_ports::{PersistedState, PersistenceError, PersistencePort};
use core::time::Duration;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Mutex, PoisonError};

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const SCHEMA_VERSION: i64 = 1;

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
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS state (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    work_minutes INTEGER NOT NULL,
                    pause_minutes INTEGER NOT NULL,
                    severity TEXT NOT NULL,
                    served_breaks INTEGER NOT NULL
                )",
                [],
            )
            .map_err(into_error)?;
        connection
            .pragma_update(None, "user_version", SCHEMA_VERSION)
            .map_err(into_error)?;
        Ok(SqliteStore {
            connection: Mutex::new(connection),
        })
    }
}

fn into_error(error: rusqlite::Error) -> PersistenceError {
    PersistenceError(error.to_string())
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
            "SELECT work_minutes, pause_minutes, severity, served_breaks FROM state WHERE id = 1",
            [],
            |row| {
                Ok(PersistedState {
                    work_minutes: row.get(0)?,
                    pause_minutes: row.get(1)?,
                    severity: severity_from(&row.get::<_, String>(2)?),
                    served_breaks: row.get(3)?,
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
                "INSERT INTO state (id, work_minutes, pause_minutes, severity, served_breaks)
                    VALUES (1, ?1, ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET
                    work_minutes = ?1, pause_minutes = ?2, severity = ?3, served_breaks = ?4",
                params![
                    state.work_minutes,
                    state.pause_minutes,
                    severity_name(state.severity),
                    state.served_breaks,
                ],
            )
            .map(|_| ())
            .map_err(into_error)
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
            })
            .unwrap();
        let updated = PersistedState {
            work_minutes: 50,
            pause_minutes: 10,
            severity: Severity::Hardcore,
            served_breaks: 4,
        };
        store.save(updated).unwrap();
        assert_eq!(store.load(), Ok(Some(updated)));
    }
}
