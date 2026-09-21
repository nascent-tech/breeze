use breeze_ports::PersistenceError;

pub(crate) fn into_error(error: rusqlite::Error) -> PersistenceError {
    PersistenceError(error.to_string())
}
