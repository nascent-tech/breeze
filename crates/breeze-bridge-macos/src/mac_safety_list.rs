use breeze_domain::{AppId, SafetyList};
use breeze_ports::SafetyListPort;

// La liste de sécurité macOS (§10.6), par identité d'exécutable. Réglages Système garde
// l'identité `com.apple.systempreferences` depuis macOS 13 (relevé sur la machine :
// /System/Applications/System Settings.app) ; aucune app ne porte `com.apple.Settings`.
pub const SAFETY_BUNDLE_IDS: [&str; 7] = [
    "com.apple.systempreferences",
    "com.apple.ActivityMonitor",
    "com.apple.Terminal",
    "com.apple.finder",
    "com.apple.keychainaccess",
    "com.apple.loginwindow",
    "com.apple.VoiceOver",
];

// Celles qui vivent hors des dossiers d'applications parcourus par le catalogue : le
// catalogue les lit ici pour que l'utilisateur les voie, verrouillées.
pub const SAFETY_BUNDLE_PATHS: [&str; 4] = [
    "/System/Library/CoreServices/Finder.app",
    "/System/Library/CoreServices/Applications/Keychain Access.app",
    "/System/Library/CoreServices/loginwindow.app",
    "/System/Library/CoreServices/VoiceOver.app",
];

#[derive(Default)]
pub struct MacSafetyList;

impl SafetyListPort for MacSafetyList {
    fn safety_list(&self) -> SafetyList {
        SafetyList::new(
            SAFETY_BUNDLE_IDS
                .iter()
                .filter_map(|raw| AppId::parse(raw).ok()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_listed_identity_is_a_valid_app_id() {
        let list = MacSafetyList.safety_list();
        assert_eq!(list.ids().count(), SAFETY_BUNDLE_IDS.len());
    }

    #[test]
    fn the_terminal_is_on_the_list() {
        let terminal = AppId::parse("com.apple.Terminal").unwrap();
        assert!(MacSafetyList.safety_list().contains(&terminal));
    }
}
