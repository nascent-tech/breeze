use crate::persistence::{PersistedState, PersistenceError};
use breeze_domain::{AppId, AppStatus};

pub trait PersistencePort {
    fn load(&self) -> Result<Option<PersistedState>, PersistenceError>;
    fn save(&self, state: PersistedState) -> Result<(), PersistenceError>;
    // Seuls les écarts au défaut (Spared/Ignored) sont tenus ; Bloquée = absence de ligne.
    fn load_app_statuses(&self) -> Result<Vec<(AppId, AppStatus)>, PersistenceError>;
    // Remplace tout l'ensemble en une transaction (BEGIN; DELETE; INSERT…; COMMIT).
    fn replace_app_statuses(&self, statuses: &[(AppId, AppStatus)])
        -> Result<(), PersistenceError>;
    // Le parcours de première utilisation a-t-il été mené à son terme ?
    fn is_onboarding_done(&self) -> Result<bool, PersistenceError>;
    fn mark_onboarding_done(&self) -> Result<(), PersistenceError>;
    // La vérification de mise à jour (seule sortie réseau) est-elle activée ? Défaut : oui.
    fn is_update_check_enabled(&self) -> Result<bool, PersistenceError>;
    fn set_update_check(&self, enabled: bool) -> Result<(), PersistenceError>;
    // Drapeau booléen de confort (clé contrôlée par l'hôte). None = jamais choisi.
    fn flag(&self, key: &str) -> Result<Option<bool>, PersistenceError>;
    fn set_flag(&self, key: &str, value: bool) -> Result<(), PersistenceError>;
}
