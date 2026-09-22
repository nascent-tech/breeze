use crate::sqlite_error::into_error;
use breeze_domain::{AppId, AppStatus};
use breeze_ports::PersistenceError;
use rusqlite::{params, Connection};

// Bloquée = absence de ligne : la table ne tient que les écarts explicites au défaut.
pub(crate) fn create_table(connection: &Connection) -> Result<(), PersistenceError> {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS app_statuses (
                bundle_id TEXT PRIMARY KEY,
                status    TEXT NOT NULL CHECK (status IN ('Spared', 'Ignored'))
            )",
            [],
        )
        .map(|_| ())
        .map_err(into_error)
}

pub(crate) fn load(connection: &Connection) -> Result<Vec<(AppId, AppStatus)>, PersistenceError> {
    let mut statement = connection
        .prepare("SELECT bundle_id, status FROM app_statuses")
        .map_err(into_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(into_error)?;
    let mut pairs = Vec::new();
    for row in rows {
        let (raw_id, raw_status) = row.map_err(into_error)?;
        // Une ligne corrompue (id invalide, statut inconnu) est ignorée, jamais fatale.
        if let (Ok(id), Some(status)) = (AppId::parse(&raw_id), status_from(&raw_status)) {
            pairs.push((id, status));
        }
    }
    Ok(pairs)
}

pub(crate) fn replace(
    connection: &mut Connection,
    statuses: &[(AppId, AppStatus)],
) -> Result<(), PersistenceError> {
    let transaction = connection.transaction().map_err(into_error)?;
    transaction
        .execute("DELETE FROM app_statuses", [])
        .map_err(into_error)?;
    {
        let mut insert = transaction
            .prepare("INSERT INTO app_statuses (bundle_id, status) VALUES (?1, ?2)")
            .map_err(into_error)?;
        for (id, status) in statuses {
            if let Some(name) = stored_name(*status) {
                insert
                    .execute(params![id.as_str(), name])
                    .map_err(into_error)?;
            }
        }
    }
    transaction.commit().map_err(into_error)
}

fn stored_name(status: AppStatus) -> Option<&'static str> {
    match status {
        AppStatus::Blocked => None,
        AppStatus::Spared => Some("Spared"),
        AppStatus::Ignored => Some("Ignored"),
    }
}

fn status_from(name: &str) -> Option<AppStatus> {
    match name {
        "Spared" => Some(AppStatus::Spared),
        "Ignored" => Some(AppStatus::Ignored),
        _ => None,
    }
}
