// Le seul code non sûr de l'adaptateur vit dans `cf_containers`, qui l'autorise
// localement et en documente l'invariant ; tout le reste le refuse.
#![deny(unsafe_code)]

// Tout l'adaptateur est macOS-only ; sur un autre OS ce crate compile à vide et
// l'hôte câble les doubles null de breeze-bridge-null.
#[cfg(target_os = "macos")]
mod cf_containers;
#[cfg(target_os = "macos")]
mod icon;
#[cfg(target_os = "macos")]
mod mac_foreground_app;
#[cfg(target_os = "macos")]
mod mac_installed_apps;
#[cfg(target_os = "macos")]
mod mac_safety_list;
#[cfg(target_os = "macos")]
mod mac_session_signals;
#[cfg(target_os = "macos")]
mod mac_window_frames;
#[cfg(target_os = "macos")]
mod running_apps;
#[cfg(target_os = "macos")]
mod window_description;

#[cfg(target_os = "macos")]
pub use mac_foreground_app::MacForegroundApp;
#[cfg(target_os = "macos")]
pub use mac_installed_apps::MacInstalledApps;
#[cfg(target_os = "macos")]
pub use mac_safety_list::MacSafetyList;
#[cfg(target_os = "macos")]
pub use mac_session_signals::MacSessionSignals;
#[cfg(target_os = "macos")]
pub use mac_window_frames::MacWindowFrames;
