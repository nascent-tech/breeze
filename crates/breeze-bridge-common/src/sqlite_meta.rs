use crate::sqlite_error::into_error;
use breeze_ports::PersistenceError;
use rusqlite::{params, Connection, OptionalExtension};

const ONBOARDING_DONE: &str = "onboarding_done";
const UPDATE_CHECK: &str = "update_check";
const SCHEDULE_START: &str = "schedule_start";
const SCHEDULE_END: &str = "schedule_end";

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

// Défaut : activé (seule sortie réseau, désactivable). Absent = jamais choisi = activé.
pub(crate) fn is_update_check_enabled(connection: &Connection) -> Result<bool, PersistenceError> {
    let value: Option<i64> = connection
        .query_row(
            "SELECT value FROM meta WHERE key = ?1",
            params![UPDATE_CHECK],
            |row| row.get(0),
        )
        .optional()
        .map_err(into_error)?;
    Ok(value.unwrap_or(1) != 0)
}

pub(crate) fn set_update_check(
    connection: &Connection,
    enabled: bool,
) -> Result<(), PersistenceError> {
    connection
        .execute(
            "INSERT INTO meta (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2",
            params![UPDATE_CHECK, i64::from(enabled)],
        )
        .map(|_| ())
        .map_err(into_error)
}

// Drapeau booléen générique (clé contrôlée par l'hôte, jamais par l'utilisateur).
// None = jamais choisi ; l'hôte décide du défaut.
pub(crate) fn flag(connection: &Connection, key: &str) -> Result<Option<bool>, PersistenceError> {
    let value: Option<i64> = connection
        .query_row(
            "SELECT value FROM meta WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()
        .map_err(into_error)?;
    Ok(value.map(|raw| raw != 0))
}

pub(crate) fn set_flag(
    connection: &Connection,
    key: &str,
    value: bool,
) -> Result<(), PersistenceError> {
    connection
        .execute(
            "INSERT INTO meta (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2",
            params![key, i64::from(value)],
        )
        .map(|_| ())
        .map_err(into_error)
}

fn number(connection: &Connection, key: &str) -> Result<Option<i64>, PersistenceError> {
    connection
        .query_row(
            "SELECT value FROM meta WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()
        .map_err(into_error)
}

pub(crate) fn remembered_schedule(
    connection: &Connection,
) -> Result<Option<(u16, u16)>, PersistenceError> {
    let start = number(connection, SCHEDULE_START)?.and_then(|raw| u16::try_from(raw).ok());
    let end = number(connection, SCHEDULE_END)?.and_then(|raw| u16::try_from(raw).ok());
    Ok(start.zip(end))
}

pub(crate) fn remember_schedule(
    connection: &Connection,
    start: u16,
    end: u16,
) -> Result<(), PersistenceError> {
    let upsert = "INSERT INTO meta (key, value) VALUES (?1, ?2)
                  ON CONFLICT(key) DO UPDATE SET value = ?2";
    let transaction = connection.unchecked_transaction().map_err(into_error)?;
    transaction
        .execute(upsert, params![SCHEDULE_START, i64::from(start)])
        .map_err(into_error)?;
    transaction
        .execute(upsert, params![SCHEDULE_END, i64::from(end)])
        .map_err(into_error)?;
    transaction.commit().map_err(into_error)
}

// Réglages d'usine : tout drapeau oublié retombe sur son défaut ; l'accueil accompli
// n'est pas une préférence et reste acquis.
pub(crate) fn forget_preferences(connection: &Connection) -> Result<(), PersistenceError> {
    connection
        .execute("DELETE FROM meta WHERE key <> ?1", params![ONBOARDING_DONE])
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
    fn schedule_bounds_are_remembered_and_forgotten_with_the_preferences() {
        let connection = store();
        assert_eq!(remembered_schedule(&connection).unwrap(), None);
        remember_schedule(&connection, 8 * 60, 17 * 60).unwrap();
        assert_eq!(
            remembered_schedule(&connection).unwrap(),
            Some((8 * 60, 17 * 60))
        );
        forget_preferences(&connection).unwrap();
        assert_eq!(remembered_schedule(&connection).unwrap(), None);
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
