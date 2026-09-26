use crate::sqlite_error::into_error;
use crate::{sqlite_app_statuses, sqlite_ledger, sqlite_meta};
use breeze_ports::PersistenceError;
use rusqlite::{params, Connection, TransactionBehavior};

const SCHEMA_VERSION: i64 = 6;

// Colonnes venues après la création de `state` : v2 la dette, v5 le calendrier.
// Définitions constantes, jamais issues d'une saisie.
const ADDED_STATE_COLUMNS: [(&str, &str); 5] = [
    ("debt_seconds", "INTEGER NOT NULL DEFAULT 0"),
    ("debt_recorded_at", "INTEGER NOT NULL DEFAULT 0"),
    ("active_days", "INTEGER NOT NULL DEFAULT 127"),
    ("schedule_start", "INTEGER"),
    ("schedule_end", "INTEGER"),
];

// Toute la mise à niveau tient dans UNE transaction qui porte aussi `user_version` : un
// échec laisse la base à sa version d'avant, intacte. Chaque étape est idempotente
// (colonne ajoutée seulement si absente, tables IF NOT EXISTS) : une base qu'une version
// antérieure a laissée à moitié migrée se rejoue sans erreur.
pub(crate) fn migrate(connection: &mut Connection) -> Result<(), PersistenceError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(into_error)?;
    let version: i64 = transaction
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(into_error)?;
    create_state_table(&transaction)?;
    for (name, definition) in ADDED_STATE_COLUMNS {
        add_state_column_if_missing(&transaction, name, definition)?;
    }
    sqlite_app_statuses::create_table(&transaction)?;
    sqlite_meta::create_table(&transaction)?;
    sqlite_ledger::create_table(&transaction)?;
    transaction
        .pragma_update(None, "user_version", version.max(SCHEMA_VERSION))
        .map_err(into_error)?;
    transaction.commit().map_err(into_error)
}

fn create_state_table(connection: &Connection) -> Result<(), PersistenceError> {
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
        .map(|_| ())
        .map_err(into_error)
}

fn add_state_column_if_missing(
    connection: &Connection,
    name: &str,
    definition: &str,
) -> Result<(), PersistenceError> {
    let present: bool = connection
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('state') WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )
        .map_err(into_error)?;
    if present {
        return Ok(());
    }
    connection
        .execute(
            &format!("ALTER TABLE state ADD COLUMN {name} {definition}"),
            [],
        )
        .map(|_| ())
        .map_err(into_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_version(connection: &Connection) -> i64 {
        connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap()
    }

    fn state_columns(connection: &Connection) -> Vec<String> {
        let mut statement = connection
            .prepare("SELECT name FROM pragma_table_info('state') ORDER BY cid")
            .unwrap();
        let names = statement.query_map([], |row| row.get(0)).unwrap();
        names.collect::<Result<_, _>>().unwrap()
    }

    // Base v4 qu'une migration non atomique a laissée à moitié : `active_days` ajoutée,
    // les colonnes de plage non, `user_version` resté à 4.
    fn half_migrated_v4() -> Connection {
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
                    debt_recorded_at INTEGER NOT NULL DEFAULT 0
                );
                INSERT INTO state VALUES (1, 50, 10, 'Simple', 2, 0, 0);
                ALTER TABLE state ADD COLUMN active_days INTEGER NOT NULL DEFAULT 127;
                CREATE TABLE app_statuses (bundle_id TEXT PRIMARY KEY, status TEXT NOT NULL);
                CREATE TABLE meta (key TEXT PRIMARY KEY, value INTEGER NOT NULL);
                PRAGMA user_version = 4;",
            )
            .unwrap();
        connection
    }

    #[test]
    fn a_half_migrated_v4_database_is_replayed_without_error() {
        let mut connection = half_migrated_v4();

        migrate(&mut connection).unwrap();

        assert_eq!(user_version(&connection), SCHEMA_VERSION);
        assert!(state_columns(&connection).ends_with(&[
            "active_days".to_owned(),
            "schedule_start".to_owned(),
            "schedule_end".to_owned(),
        ]));
        let served: i64 = connection
            .query_row("SELECT served_breaks FROM state", [], |row| row.get(0))
            .unwrap();
        assert_eq!(served, 2);
    }

    #[test]
    fn migrating_an_up_to_date_database_again_changes_nothing() {
        let mut connection = Connection::open_in_memory().unwrap();
        migrate(&mut connection).unwrap();
        let columns = state_columns(&connection);

        migrate(&mut connection).unwrap();

        assert_eq!(state_columns(&connection), columns);
        assert_eq!(user_version(&connection), SCHEMA_VERSION);
    }

    #[test]
    fn a_database_from_a_newer_version_keeps_its_version() {
        let mut connection = Connection::open_in_memory().unwrap();
        connection
            .pragma_update(None, "user_version", SCHEMA_VERSION + 1)
            .unwrap();

        migrate(&mut connection).unwrap();

        assert_eq!(user_version(&connection), SCHEMA_VERSION + 1);
    }
}
