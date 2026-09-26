pub mod monitor_cache;
#[cfg(target_os = "macos")]
pub mod native_presentation;
pub mod native_window;
#[cfg(target_os = "macos")]
pub mod presentation_options;
pub mod screen_surface;
pub mod tauri_displays;
pub mod tauri_overlay;
#[cfg(target_os = "macos")]
pub mod tauri_presentation_lock;
pub mod window_veil;

pub use monitor_cache::MonitorCache;
pub use tauri_displays::TauriDisplays;
pub use tauri_overlay::TauriOverlay;
#[cfg(target_os = "macos")]
pub use tauri_presentation_lock::TauriPresentationLock;

use breeze_ports::PresentationLockPort;

// Verrou de présentation d'une pause Hardcore : barre de menus, Dock et ⌘Tab (§8.5).
#[cfg(target_os = "macos")]
pub fn presentation_lock(app: &tauri::AppHandle) -> Box<dyn PresentationLockPort> {
    Box::new(TauriPresentationLock::new(app.clone()))
}

// Hors macOS, rien à verrouiller : l'overlay revient devant, il ne bloque pas (§5.3).
#[cfg(not(target_os = "macos"))]
pub fn presentation_lock(_app: &tauri::AppHandle) -> Box<dyn PresentationLockPort> {
    Box::new(breeze_bridge_null::NullPresentationLock)
}
