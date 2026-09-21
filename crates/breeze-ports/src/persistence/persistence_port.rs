use crate::persistence::PersistedState;

pub trait PersistencePort {
    fn load(&self) -> Option<PersistedState>;
    fn save(&self, state: PersistedState);
}
