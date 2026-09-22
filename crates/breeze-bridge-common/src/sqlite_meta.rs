use crate::sqlite_error::into_error;
use breeze_ports::PersistenceError;
use rusqlite::{params, Connection, OptionalExtension};

const ONBOARDING_DONE: &str = "onboarding_done";

// Petit magasin clé→entier pour les drapeaux d'app (hors état du cycle).
pub(crate) fn create_table(connection: &Connection) -> Result<(), PersistenceError> {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS meta (
                key   TEXT PRIMARY KEY,
                value INTEGER NOT NULL
            )",
            [],
        )
        .map(|_| ())
        .map_err(into_error)
}

pub(crate) fn is_onboarding_done(connection: &Connection) -> Result<bool, PersistenceError> {
    let value: Option<i64> = connection
        .query_row(
            "SELECT value FROM meta WHERE key = ?1",
            params![ONBOARDING_DONE],
            |row| row.get(0),
        )
        .optional()
        .map_err(into_error)?;
    Ok(value.unwrap_or(0) != 0)
}

pub(crate) fn mark_onboarding_done(connection: &Connection) -> Result<(), PersistenceError> {
    connection
        .execute(
            "INSERT INTO meta (key, value) VALUES (?1, 1)
             ON CONFLICT(key) DO UPDATE SET value = 1",
            params![ONBOARDING_DONE],
        )
        .map(|_| ())
        .map_err(into_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        create_table(&connection).unwrap();
        connection
    }

    #[test]
    fn a_fresh_meta_table_reports_onboarding_not_done() {
        assert!(!is_onboarding_done(&store()).unwrap());
    }

    #[test]
    fn marking_onboarding_done_persists_and_is_idempotent() {
        let connection = store();
        mark_onboarding_done(&connection).unwrap();
        mark_onboarding_done(&connection).unwrap();
        assert!(is_onboarding_done(&connection).unwrap());
    }
}
