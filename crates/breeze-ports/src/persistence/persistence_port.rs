use crate::persistence::{PersistedState, PersistenceError};

pub trait PersistencePort {
    fn load(&self) -> Result<Option<PersistedState>, PersistenceError>;
    fn save(&self, state: PersistedState) -> Result<(), PersistenceError>;
}
