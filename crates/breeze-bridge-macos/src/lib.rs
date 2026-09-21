#![forbid(unsafe_code)]

// Tout l'adaptateur est macOS-only ; sur un autre OS ce crate compile à vide et
// l'hôte câble les doubles null de breeze-bridge-null.
#[cfg(target_os = "macos")]
mod icon;
#[cfg(target_os = "macos")]
mod mac_accessibility;
#[cfg(target_os = "macos")]
mod mac_installed_apps;

#[cfg(target_os = "macos")]
pub use mac_accessibility::MacAccessibility;
#[cfg(target_os = "macos")]
pub use mac_installed_apps::MacInstalledApps;
