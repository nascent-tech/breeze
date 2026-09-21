use breeze_domain::Severity;
use breeze_ports::{PersistedState, PersistencePort};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

pub struct SqliteStore {
    connection: Mutex<Connection>,
}

impl SqliteStore {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        Self::from_connection(Connection::open(path)?)
    }

    pub fn in_memory() -> rusqlite::Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(connection: Connection) -> rusqlite::Result<Self> {
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS state (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                work_minutes INTEGER NOT NULL,
                pause_minutes INTEGER NOT NULL,
                severity TEXT NOT NULL,
                served_breaks INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(SqliteStore {
            connection: Mutex::new(connection),
        })
    }
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
    fn load(&self) -> Option<PersistedState> {
        let connection = self.connection.lock().ok()?;
        connection
            .query_row(
                "SELECT work_minutes, pause_minutes, severity, served_breaks FROM state WHERE id = 1",
                [],
                |row| {
                    let work: i64 = row.get(0)?;
                    let pause: i64 = row.get(1)?;
                    let severity: String = row.get(2)?;
                    let served: i64 = row.get(3)?;
                    Ok(PersistedState {
                        work_minutes: work as u16,
                        pause_minutes: pause as u16,
                        severity: severity_from(&severity),
                        served_breaks: served as u32,
                    })
                },
            )
            .ok()
    }

    fn save(&self, state: PersistedState) {
        let Ok(connection) = self.connection.lock() else {
            return;
        };
        let _ = connection.execute(
            "INSERT INTO state (id, work_minutes, pause_minutes, severity, served_breaks)
                VALUES (1, ?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET
                work_minutes = ?1, pause_minutes = ?2, severity = ?3, served_breaks = ?4",
            params![
                i64::from(state.work_minutes),
                i64::from(state.pause_minutes),
                severity_name(state.severity),
                i64::from(state.served_breaks),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_store_holds_nothing() {
        let store = SqliteStore::in_memory().unwrap();
        assert!(store.load().is_none());
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
        store.save(state);
        assert_eq!(store.load(), Some(state));
    }

    #[test]
    fn a_second_save_replaces_the_first() {
        let store = SqliteStore::in_memory().unwrap();
        store.save(PersistedState {
            work_minutes: 25,
            pause_minutes: 5,
            severity: Severity::Simple,
            served_breaks: 1,
        });
        let updated = PersistedState {
            work_minutes: 50,
            pause_minutes: 10,
            severity: Severity::Hardcore,
            served_breaks: 4,
        };
        store.save(updated);
        assert_eq!(store.load(), Some(updated));
    }
}
