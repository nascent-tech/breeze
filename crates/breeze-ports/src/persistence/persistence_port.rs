use crate::persistence::{LedgerDay, LedgerEntry, PersistedState, PersistenceError};
use breeze_domain::{AppId, AppStatus};

pub trait PersistencePort {
    fn load(&self) -> Result<Option<PersistedState>, PersistenceError>;
    fn save(&self, state: PersistedState) -> Result<(), PersistenceError>;
    // Retour aux réglages d'usine en une seule transaction : état écrit, statuts d'apps
    // vidés, drapeaux de confort et vérification de mise à jour rendus à leur défaut.
    // L'accueil déjà accompli reste acquis.
    fn reset_preferences(&self, state: PersistedState) -> Result<(), PersistenceError>;
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
    // Dernières bornes de plage horaire saisies, gardées quand la plage est coupée pour
    // qu'on la retrouve telle quelle en la rallumant. None = jamais saisies.
    fn remembered_schedule(&self) -> Result<Option<(u16, u16)>, PersistenceError>;
    fn remember_schedule(&self, start: u16, end: u16) -> Result<(), PersistenceError>;
    // Journal des pauses : ajout seul, jamais réécrit.
    fn record_break(&self, entry: &LedgerEntry) -> Result<(), PersistenceError>;
    // Agrégats par jour local à partir de `from_date` inclus ("YYYY-MM-DD"), jours
    // croissants ; un jour sans pause n'a pas de ligne.
    fn ledger_days(&self, from_date: &str) -> Result<Vec<LedgerDay>, PersistenceError>;
}
