// Réglages AppKit que Tauri n'expose pas, posés sur le NSWindow d'une surface de pause.
// Sans effet hors de macOS. Toujours appelés sur le thread principal.

#[cfg(target_os = "macos")]
mod appkit {
    use breeze_ports::WindowId;
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior, NSWindowOrderingMode};
    use tauri::WebviewWindow;

    // Exécute `act` sur le NSWindow d'une fenêtre Tauri. Hors du thread principal, rien
    // n'est fait (journalisé) : AppKit n'y tolère aucun appel.
    fn with_ns_window(window: &WebviewWindow, act: impl FnOnce(&NSWindow)) -> tauri::Result<()> {
        if MainThreadMarker::new().is_none() {
            eprintln!("breeze: {} touched off the main thread", window.label());
            return Ok(());
        }
        let pointer = window.ns_window()?;
        // SAFETY: `ns_window()` rend le pointeur du NSWindow vivant de cette fenêtre Tauri
        // (retenu par Tauri tant que la fenêtre existe, autoreleased pour l'appel en cours) ;
        // nous sommes sur le thread principal (vérifié ci-dessus) et la référence ne sort
        // pas de cette fonction.
        let Some(ns_window) = (unsafe { pointer.cast::<NSWindow>().as_ref() }) else {
            return Ok(());
        };
        act(ns_window);
        Ok(())
    }

    // L'espace d'une app en plein écran natif accepte la surface en auxiliaire ; le reste du
    // comportement posé par Tauri (tous les bureaux ou non) est gardé tel quel. AppKit
    // n'admet qu'un seul des trois drapeaux plein écran : les deux autres sont retirés.
    pub fn join_full_screen_spaces(window: &WebviewWindow) -> tauri::Result<()> {
        with_ns_window(window, |ns_window| {
            let behavior = (ns_window.collectionBehavior()
                - NSWindowCollectionBehavior::FullScreenPrimary
                - NSWindowCollectionBehavior::FullScreenNone)
                | NSWindowCollectionBehavior::FullScreenAuxiliary;
            ns_window.setCollectionBehavior(behavior);
        })
    }

    // Range la surface juste au-dessus de la fenêtre cible, dont le numéro CGWindow
    // (kCGWindowNumber) est le `windowNumber` d'AppKit. `orderWindow:relativeTo:` n'active
    // pas l'app et ne rend pas la fenêtre clé : le focus reste où il est.
    pub fn order_above(window: &WebviewWindow, target: WindowId) -> tauri::Result<()> {
        let Ok(number) = isize::try_from(target.0) else {
            return Ok(());
        };
        with_ns_window(window, |ns_window| {
            ns_window.orderWindow_relativeTo(NSWindowOrderingMode::Above, number);
        })
    }
}

#[cfg(not(target_os = "macos"))]
mod appkit {
    use breeze_ports::WindowId;
    use tauri::WebviewWindow;

    pub fn join_full_screen_spaces(_window: &WebviewWindow) -> tauri::Result<()> {
        Ok(())
    }

    pub fn order_above(_window: &WebviewWindow, _target: WindowId) -> tauri::Result<()> {
        Ok(())
    }
}

pub use appkit::{join_full_screen_spaces, order_above};
