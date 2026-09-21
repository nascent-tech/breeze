use crate::settings::app_id::AppId;
use crate::settings::app_status::AppStatus;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SparedApps {
    // Seuls les choix explicites sont tenus. Une app absente est Bloquée d'office
    // (brief §8.2 : toute nouvelle application est bloquée jusqu'à choix contraire).
    overrides: BTreeMap<AppId, AppStatus>,
}

impl SparedApps {
    pub fn new() -> Self {
        SparedApps {
            overrides: BTreeMap::new(),
        }
    }

    pub fn from_pairs(pairs: impl IntoIterator<Item = (AppId, AppStatus)>) -> Self {
        let mut apps = SparedApps::new();
        for (id, status) in pairs {
            apps.set(id, status);
        }
        apps
    }

    pub fn status_of(&self, id: &AppId) -> AppStatus {
        self.overrides
            .get(id)
            .copied()
            .unwrap_or(AppStatus::Blocked)
    }

    pub fn set(&mut self, id: AppId, status: AppStatus) {
        // Revenir à Bloquée = retirer la ligne : la collection ne garde que les écarts au défaut.
        match status {
            AppStatus::Blocked => {
                self.overrides.remove(&id);
            }
            AppStatus::Spared | AppStatus::Ignored => {
                self.overrides.insert(id, status);
            }
        }
    }

    pub fn pairs(&self) -> Vec<(AppId, AppStatus)> {
        self.overrides
            .iter()
            .map(|(id, status)| (id.clone(), *status))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(raw: &str) -> AppId {
        AppId::parse(raw).unwrap()
    }

    #[test]
    fn an_unknown_app_is_blocked_by_default() {
        let apps = SparedApps::new();
        assert_eq!(apps.status_of(&id("com.unknown.App")), AppStatus::Blocked);
    }

    #[test]
    fn a_spared_app_keeps_its_status() {
        let mut apps = SparedApps::new();
        apps.set(id("com.apple.Music"), AppStatus::Spared);
        assert_eq!(apps.status_of(&id("com.apple.Music")), AppStatus::Spared);
    }

    #[test]
    fn returning_an_app_to_blocked_drops_it_from_the_kept_overrides() {
        let mut apps = SparedApps::new();
        apps.set(id("com.tinyspeck.slackmacgap"), AppStatus::Ignored);
        apps.set(id("com.tinyspeck.slackmacgap"), AppStatus::Blocked);
        assert!(apps.pairs().is_empty());
        assert_eq!(
            apps.status_of(&id("com.tinyspeck.slackmacgap")),
            AppStatus::Blocked
        );
    }

    #[test]
    fn it_is_built_from_persisted_pairs() {
        let apps = SparedApps::from_pairs([
            (id("a.b.c"), AppStatus::Spared),
            (id("d.e.f"), AppStatus::Ignored),
        ]);
        assert_eq!(apps.status_of(&id("a.b.c")), AppStatus::Spared);
        assert_eq!(apps.status_of(&id("d.e.f")), AppStatus::Ignored);
    }
}
