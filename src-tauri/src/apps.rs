use crate::AppState;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use breeze_domain::{AppId, AppStatus};
use breeze_ports::PermissionStatus;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

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

#[tauri::command]
pub fn open_settings(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_focus();
        return Ok(());
    }
    tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("Breeze — Réglages")
    .inner_size(820.0, 660.0)
    .resizable(false)
    .build()
    .map(|_| ())
    .map_err(|error| error.to_string())
}
