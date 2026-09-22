use breeze_ports::{AppEnumerationError, InstalledApp, InstalledAppsPort};

// Aucun catalogue d'apps : sur un OS sans adaptateur dédié, la liste est vide et
// l'UI n'affiche aucune ligne d'application. Sert aussi de double de test isolé.
pub struct NullInstalledApps;

impl InstalledAppsPort for NullInstalledApps {
    fn installed_apps(&self) -> Result<Vec<InstalledApp>, AppEnumerationError> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reports_an_empty_catalogue() {
        assert_eq!(NullInstalledApps.installed_apps(), Ok(Vec::new()));
    }
}
