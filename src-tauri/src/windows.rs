use crate::{lock, panel, AppState};
use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime, State, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const SETTINGS_LABEL: &str = "settings";
const SETTINGS_WIDTH: f64 = 820.0;
const SETTINGS_HEIGHT: f64 = 640.0;
const ONBOARDING_LABEL: &str = "onboarding";
const ONBOARDING_WIDTH: f64 = 520.0;
const ONBOARDING_HEIGHT: f64 = 760.0;

#[derive(Serialize)]
pub struct AppInfoDto {
    pub version: String,
    pub name: String,
}

fn window_failed(error: tauri::Error) -> String {
    eprintln!("breeze: could not open window: {error}");
    "window-failed".to_owned()
}

// Fenêtre native : feux tricolores, ombre et coins du système, titre masqué ; le
// contenu passe sous la barre de titre et l'UI y pose sa zone `data-tauri-drag-region`.
#[cfg(target_os = "macos")]
fn native_frame<'a, R: Runtime, M: Manager<R>>(
    builder: WebviewWindowBuilder<'a, R, M>,
) -> WebviewWindowBuilder<'a, R, M> {
    builder
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .hidden_title(true)
}

#[cfg(not(target_os = "macos"))]
fn native_frame<'a, R: Runtime, M: Manager<R>>(
    builder: WebviewWindowBuilder<'a, R, M>,
) -> WebviewWindowBuilder<'a, R, M> {
    builder
}

fn bring_forward(window: &WebviewWindow) {
    let shown = window
        .show()
        .and_then(|()| window.unminimize())
        .and_then(|()| window.set_focus());
    if let Err(error) = shown {
        eprintln!(
            "breeze: could not bring {} forward: {error}",
            window.label()
        );
    }
}

fn open_window(
    app: &AppHandle,
    label: &str,
    page: &str,
    title: &str,
    (width, height): (f64, f64),
) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(label) {
        bring_forward(&window);
        return Ok(());
    }
    let builder = WebviewWindowBuilder::new(app, label, WebviewUrl::App(page.into()))
        .title(title)
        .inner_size(width, height)
        .resizable(false)
        .maximizable(false)
        .center()
        .focused(true);
    let window = native_frame(builder).build()?;
    bring_forward(&window);
    Ok(())
}

pub fn show_settings(app: &AppHandle) {
    if let Err(error) = open_settings_window(app) {
        eprintln!("breeze: could not open settings: {error}");
    }
}

fn open_settings_window(app: &AppHandle) -> tauri::Result<()> {
    panel::hide(app);
    open_window(
        app,
        SETTINGS_LABEL,
        "settings.html",
        "Breeze — Réglages",
        (SETTINGS_WIDTH, SETTINGS_HEIGHT),
    )
}

pub fn open_onboarding(app: &AppHandle) -> tauri::Result<()> {
    open_window(
        app,
        ONBOARDING_LABEL,
        "onboarding.html",
        "Bienvenue dans Breeze",
        (ONBOARDING_WIDTH, ONBOARDING_HEIGHT),
    )
}

#[tauri::command]
pub fn open_settings(app: AppHandle) -> Result<(), String> {
    open_settings_window(&app).map_err(window_failed)
}

#[tauri::command]
pub fn reopen_onboarding(app: AppHandle) -> Result<(), String> {
    open_onboarding(&app).map_err(window_failed)
}

// `destroy` et non `close` : l'UI intercepte la demande de fermeture pour appliquer
// ses choix puis appeler cette commande — un `close` relancerait l'interception.
#[tauri::command]
pub fn finish_onboarding(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let first_run = !state.persistence.is_onboarding_done().unwrap_or(true);
    if let Err(error) = state.persistence.mark_onboarding_done() {
        eprintln!("breeze: could not mark onboarding done: {}", error.0);
        return Err("persistence-failed".to_owned());
    }
    if let Some(onboarding) = app.get_webview_window(ONBOARDING_LABEL) {
        if let Err(error) = onboarding.destroy() {
            eprintln!("breeze: could not close onboarding: {error}");
        }
    }
    if first_run {
        start_first_cycle(&state);
    }
    panel::show_panel(&app, panel::tray_rect(&app));
    Ok(())
}

// Le cycle a démarré en 50/10 avant l'accueil : les choix faits y attendraient le cycle
// suivant. Au premier accueil, le premier cycle part directement avec eux.
fn start_first_cycle(state: &AppState) {
    let now = state.clock.monotonic();
    if lock(&state.scheduler).start_over(now).is_err() {
        return;
    }
    if let Err(error) = state.save_state() {
        eprintln!("breeze: first cycle not persisted: {error}");
    }
}

#[tauri::command]
pub fn get_app_info(app: AppHandle) -> AppInfoDto {
    let info = app.package_info();
    AppInfoDto {
        version: info.version.to_string(),
        name: info.name.clone(),
    }
}

fn web_address(raw: &str) -> Option<tauri::Url> {
    let url = tauri::Url::parse(raw).ok()?;
    (url.scheme() == "https" && url.host_str().is_some()).then_some(url)
}

#[tauri::command]
pub fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    let address = web_address(&url).ok_or_else(|| "invalid-url".to_owned())?;
    app.opener()
        .open_url(address.as_str(), None::<&str>)
        .map_err(|error| {
            eprintln!("breeze: could not open {address}: {error}");
            "open-failed".to_owned()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_secure_web_address_is_accepted() {
        assert!(web_address("https://github.com/nascent-tech/breeze/issues").is_some());
    }

    #[test]
    fn every_other_scheme_is_refused() {
        assert!(web_address(concat!("http", "://github.com")).is_none());
        assert!(web_address("file:///etc/passwd").is_none());
        assert!(web_address("javascript:alert(1)").is_none());
        assert!(web_address("not a url").is_none());
    }
}
