use crate::AppState;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use breeze_domain::{AppId, AppStatus, SparedApps};
use breeze_ports::PermissionStatus;
use serde::Serialize;
use std::collections::HashMap;
use tauri::State;

#[derive(Serialize)]
pub struct InstalledAppDto {
    pub bundle_id: String,
    pub name: String,
    // Data URL PNG déjà rendu ; None = l'UI retombe sur la lettre.
    pub icon_data_url: Option<String>,
    pub status: &'static str,
}

fn status_label(status: AppStatus) -> &'static str {
    match status {
        AppStatus::Blocked => "Blocked",
        AppStatus::Spared => "Spared",
        AppStatus::Ignored => "Ignored",
    }
}

fn parse_status(name: &str) -> Option<AppStatus> {
    match name {
        "Blocked" => Some(AppStatus::Blocked),
        "Spared" => Some(AppStatus::Spared),
        "Ignored" => Some(AppStatus::Ignored),
        _ => None,
    }
}

fn to_data_url(png: &[u8]) -> String {
    format!("data:image/png;base64,{}", STANDARD.encode(png))
}

fn permission_label(status: PermissionStatus) -> &'static str {
    match status {
        PermissionStatus::Granted => "Granted",
        PermissionStatus::Denied => "Denied",
        PermissionStatus::Unknown => "Unknown",
    }
}

#[tauri::command]
pub async fn list_installed_apps(
    state: State<'_, AppState>,
) -> Result<Vec<InstalledAppDto>, String> {
    // Énumération lente (sips par app) tenue hors du fil UI.
    let port = state.installed_apps.clone();
    let apps = tauri::async_runtime::spawn_blocking(move || port.installed_apps())
        .await
        .map_err(|_| "enumeration-cancelled".to_owned())?
        .map_err(|error| {
            eprintln!("breeze: could not enumerate installed apps: {}", error.0);
            "enumeration-failed".to_owned()
        })?;
    let spared = state.spared.lock().unwrap_or_else(|e| e.into_inner());
    Ok(apps
        .into_iter()
        .map(|app| InstalledAppDto {
            status: status_label(spared.status_of(&app.id)),
            bundle_id: app.id.as_str().to_owned(),
            name: app.name,
            icon_data_url: app.icon_png.as_deref().map(to_data_url),
        })
        .collect())
}

#[tauri::command]
pub fn set_app_status(
    state: State<'_, AppState>,
    bundle_id: String,
    status: String,
) -> Result<(), String> {
    let id = AppId::parse(&bundle_id).map_err(|_| "invalid-app-id".to_owned())?;
    let status = parse_status(&status).ok_or_else(|| "unknown-status".to_owned())?;
    // Verrou tenu pendant l'écriture : mémoire et disque ne divergent jamais. En cas
    // d'échec de persistance, l'ancien statut est restauré.
    let mut spared = state.spared.lock().unwrap_or_else(|e| e.into_inner());
    let previous = spared.status_of(&id);
    spared.set(id.clone(), status);
    if let Err(error) = state.persistence.replace_app_statuses(&spared.pairs()) {
        spared.set(id, previous);
        eprintln!("breeze: could not persist app status: {}", error.0);
        return Err("persistence-failed".to_owned());
    }
    Ok(())
}

#[tauri::command]
pub fn request_accessibility(state: State<'_, AppState>) -> bool {
    state.accessibility.request();
    matches!(state.accessibility.status(), PermissionStatus::Granted)
}

#[tauri::command]
pub fn accessibility_status(state: State<'_, AppState>) -> &'static str {
    permission_label(state.accessibility.status())
}

// N'écrit que les apps transmises : une app absente de la carte garde son statut
// (une app Ignorée ne redevient jamais Bloquée faute d'avoir été renvoyée).
fn apply_spared_choices(chosen: &mut SparedApps, choices: HashMap<String, bool>) {
    for (raw_id, is_spared) in choices {
        // Un id invalide est ignoré (onboarding = geste large, jamais fatal).
        let Ok(id) = AppId::parse(&raw_id) else {
            continue;
        };
        let status = if is_spared {
            AppStatus::Spared
        } else {
            AppStatus::Blocked
        };
        chosen.set(id, status);
    }
}

#[tauri::command]
pub fn set_spared_apps(
    state: State<'_, AppState>,
    spared: HashMap<String, bool>,
) -> Result<(), String> {
    let mut chosen = state.spared.lock().unwrap_or_else(|e| e.into_inner());
    let snapshot = chosen.clone();
    apply_spared_choices(&mut chosen, spared);
    if let Err(error) = state.persistence.replace_app_statuses(&chosen.pairs()) {
        *chosen = snapshot;
        eprintln!("breeze: could not persist spared apps: {}", error.0);
        return Err("persistence-failed".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(raw: &str) -> AppId {
        AppId::parse(raw).unwrap()
    }

    #[test]
    fn only_the_apps_sent_change_status() {
        let mut chosen = SparedApps::new();
        chosen.set(app("com.tinyspeck.slackmacgap"), AppStatus::Ignored);
        chosen.set(app("com.apple.Music"), AppStatus::Spared);
        let choices = HashMap::from([("com.apple.Music".to_owned(), false)]);

        apply_spared_choices(&mut chosen, choices);

        assert_eq!(
            chosen.status_of(&app("com.tinyspeck.slackmacgap")),
            AppStatus::Ignored
        );
        assert_eq!(
            chosen.status_of(&app("com.apple.Music")),
            AppStatus::Blocked
        );
    }

    #[test]
    fn an_invalid_id_is_skipped_without_failing() {
        let mut chosen = SparedApps::new();
        let choices = HashMap::from([
            ("not a bundle id".to_owned(), true),
            ("com.apple.Notes".to_owned(), true),
        ]);

        apply_spared_choices(&mut chosen, choices);

        assert_eq!(chosen.status_of(&app("com.apple.Notes")), AppStatus::Spared);
    }
}
