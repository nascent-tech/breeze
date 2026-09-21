use crate::apps::app_enumeration_error::AppEnumerationError;
use crate::apps::installed_app::InstalledApp;

pub trait InstalledAppsPort: Send + Sync {
    // Lecture locale, potentiellement lente (~1 s pour ~75 apps) : l'hôte l'appelle
    // hors du fil UI. Aucune sortie réseau.
    fn installed_apps(&self) -> Result<Vec<InstalledApp>, AppEnumerationError>;
}
