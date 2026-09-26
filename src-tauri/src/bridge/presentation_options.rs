// Les options de présentation d'une pause Hardcore (§8.5) et les règles de combinaison
// d'AppKit. Pur : aucun appel au système, testable sans fenêtre.

use objc2_app_kit::NSApplicationPresentationOptions as Options;

// Barre de menus et Dock masqués, ⌘Tab et « Masquer Breeze » désactivés. Forcer à quitter
// et la fermeture de session restent possibles (§8.5) : jamais DisableForceQuit ni
// DisableSessionTermination.
pub fn hardcore_options() -> Options {
    Options::HideDock
        | Options::HideMenuBar
        | Options::DisableProcessSwitching
        | Options::DisableHideApplication
}

// Si l'une des options de gauche est posée, l'une de celles de droite doit l'être aussi.
fn requirements() -> [(Options, Options); 4] {
    [
        (Options::HideMenuBar, Options::HideDock),
        (
            Options::AutoHideMenuBar
                | Options::DisableProcessSwitching
                | Options::DisableForceQuit
                | Options::DisableSessionTermination,
            Options::HideDock | Options::AutoHideDock,
        ),
        (
            Options::DisableAppleMenu,
            Options::HideMenuBar | Options::AutoHideMenuBar,
        ),
        (Options::AutoHideToolbar, Options::FullScreen),
    ]
}

// Jamais les deux options d'une paire ensemble.
fn exclusions() -> [Options; 2] {
    [
        Options::HideDock | Options::AutoHideDock,
        Options::HideMenuBar | Options::AutoHideMenuBar,
    ]
}

// `setPresentationOptions:` lève une exception sur une combinaison refusée, et une exception
// Objective-C qui traverse Rust arrête le processus : on vérifie avant d'appliquer.
pub fn is_legal(options: Options) -> bool {
    let required = requirements()
        .into_iter()
        .all(|(when, needs)| !options.intersects(when) || options.intersects(needs));
    let exclusive = exclusions().into_iter().all(|pair| !options.contains(pair));
    required && exclusive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hardcore_break_hides_the_dock_and_menu_bar_and_disables_switching_and_hiding() {
        let options = hardcore_options();
        assert!(options.contains(Options::HideDock));
        assert!(options.contains(Options::HideMenuBar));
        assert!(options.contains(Options::DisableProcessSwitching));
        assert!(options.contains(Options::DisableHideApplication));
    }

    #[test]
    fn a_hardcore_break_never_disables_force_quit_nor_session_termination() {
        let options = hardcore_options();
        assert!(!options.contains(Options::DisableForceQuit));
        assert!(!options.contains(Options::DisableSessionTermination));
    }

    #[test]
    fn the_hardcore_options_are_a_combination_appkit_accepts() {
        assert!(is_legal(hardcore_options()));
        assert!(is_legal(Options::Default));
    }

    #[test]
    fn hiding_the_menu_bar_without_hiding_the_dock_is_refused() {
        assert!(!is_legal(Options::HideMenuBar));
        assert!(!is_legal(Options::HideMenuBar | Options::AutoHideDock));
    }

    #[test]
    fn disabling_process_switching_needs_a_hidden_dock() {
        assert!(!is_legal(Options::DisableProcessSwitching));
        assert!(is_legal(
            Options::DisableProcessSwitching | Options::AutoHideDock
        ));
    }

    #[test]
    fn the_dock_cannot_be_both_hidden_and_auto_hidden() {
        assert!(!is_legal(Options::HideDock | Options::AutoHideDock));
    }

    #[test]
    fn the_menu_bar_cannot_be_both_hidden_and_auto_hidden() {
        assert!(!is_legal(
            Options::HideDock | Options::HideMenuBar | Options::AutoHideMenuBar
        ));
    }

    #[test]
    fn disabling_the_apple_menu_needs_a_hidden_menu_bar() {
        assert!(!is_legal(Options::DisableAppleMenu | Options::HideDock));
        assert!(is_legal(
            Options::DisableAppleMenu | Options::HideDock | Options::HideMenuBar
        ));
    }
}
