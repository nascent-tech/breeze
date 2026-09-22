mod apps;
mod bridge;
mod dto;
mod settings;

use breeze_app::{CyclePhase, Scheduler};
use breeze_bridge_common::{SqliteStore, SystemClock};
use breeze_bridge_null::NullSessionSignals;
#[cfg(not(target_os = "macos"))]
use breeze_bridge_null::{NullAccessibility, NullInstalledApps};
use breeze_domain::constants::{MINUTES_PER_DAY, SECONDS_PER_MINUTE};
use breeze_domain::{
    ActiveDays, CommandError, Cycle, InterruptionDoor, Minutes, PostureDebt, Rhythm, Severity,
    SparedApps,
};
use breeze_ports::ClockPort;
use breeze_ports::SessionSignalsPort;
use breeze_ports::{AccessibilityPermissionPort, InstalledAppsPort};
use core::time::Duration;
use dto::{to_dto, SnapshotDto};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const TICK: Duration = Duration::from_millis(250);
const DEFAULT_WORK: u16 = 50;
const DEFAULT_PAUSE: u16 = 10;
const TRAY_ID: &str = "breeze-tray";
pub(crate) const FLAG_SOUNDS: &str = "sounds";
pub(crate) const FLAG_MENUBAR_TEXT: &str = "menubar_text";
const PANEL_SHORTCUT: &str = "Alt+Cmd+B";

type Persistence = Arc<dyn breeze_ports::PersistencePort + Send + Sync>;
type Clock = Arc<dyn ClockPort + Send + Sync>;
type InstalledApps = Arc<dyn InstalledAppsPort>;
type Accessibility = Arc<dyn AccessibilityPermissionPort>;
type Toggle = Arc<AtomicBool>;

pub(crate) struct AppState {
    pub(crate) scheduler: Arc<Mutex<Scheduler>>,
    pub(crate) clock: Clock,
    pub(crate) persistence: Persistence,
    pub(crate) installed_apps: InstalledApps,
    pub(crate) accessibility: Accessibility,
    pub(crate) spared: Arc<Mutex<SparedApps>>,
    pub(crate) sounds: Toggle,
    pub(crate) menubar_text: Toggle,
}

fn parse_severity(name: &str) -> Option<Severity> {
    match name {
        "Simple" => Some(Severity::Simple),
        "Hardcore" => Some(Severity::Hardcore),
        _ => None,
    }
}

pub(crate) fn refusal(error: CommandError) -> String {
    match error {
        CommandError::BreakDue => "break-due",
        CommandError::NotSuspendable => "not-suspendable",
        CommandError::NotSuspended => "not-suspended",
        CommandError::NotInterruptible => "not-interruptible",
    }
    .to_owned()
}

pub(crate) fn lock(scheduler: &Mutex<Scheduler>) -> std::sync::MutexGuard<'_, Scheduler> {
    scheduler
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub(crate) fn save_state(persistence: &Persistence, scheduler: &Mutex<Scheduler>, clock: &Clock) {
    let (rhythm, severity, served, debt) = {
        let scheduler = lock(scheduler);
        (
            scheduler.configured_rhythm(),
            scheduler.chosen_severity(),
            scheduler.snapshot().served_breaks,
            scheduler.debt(),
        )
    };
    let schedule = rhythm.schedule();
    let state = breeze_ports::PersistedState {
        work_minutes: rhythm.work().count(),
        pause_minutes: rhythm.pause().count(),
        severity,
        served_breaks: served,
        debt_seconds: u32::try_from(debt.total().as_secs()).unwrap_or(u32::MAX),
        debt_recorded_at_unix: clock.wall().as_unix_secs(),
        active_days: rhythm.active_days().mask(),
        schedule_start: schedule.map(breeze_domain::TimeRange::start),
        schedule_end: schedule.map(breeze_domain::TimeRange::end),
    };
    if let Err(error) = persistence.save(state) {
        eprintln!("breeze: could not persist state: {}", error.0);
    }
}

#[tauri::command]
fn get_snapshot(state: tauri::State<'_, AppState>) -> SnapshotDto {
    let now = state.clock.monotonic();
    let (snapshot, active, configured) = {
        let scheduler = lock(&state.scheduler);
        (
            scheduler.snapshot(),
            scheduler.active_rhythm(),
            scheduler.configured_rhythm(),
        )
    };
    to_dto(snapshot, now, &active, &configured)
}

#[tauri::command]
fn suspend(state: tauri::State<'_, AppState>, minutes: u64) -> Result<(), String> {
    let now = state.clock.monotonic();
    let capped = minutes.clamp(1, u64::from(MINUTES_PER_DAY));
    let resume_at = now.plus(Duration::from_secs(
        capped.saturating_mul(SECONDS_PER_MINUTE),
    ));
    lock(&state.scheduler)
        .suspend(now, resume_at)
        .map_err(refusal)?;
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}

#[tauri::command]
fn resume(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let now = state.clock.monotonic();
    lock(&state.scheduler).resume(now).map_err(refusal)?;
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}

#[tauri::command]
fn interrupt_break(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let now = state.clock.monotonic();
    lock(&state.scheduler)
        .interrupt_break(now)
        .map_err(refusal)?;
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}

#[tauri::command]
fn set_severity(state: tauri::State<'_, AppState>, severity: String) -> Result<(), String> {
    let parsed = parse_severity(&severity).ok_or_else(|| "unknown-severity".to_owned())?;
    lock(&state.scheduler)
        .change_severity(parsed)
        .map_err(refusal)?;
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}

#[tauri::command]
fn set_rhythm(
    state: tauri::State<'_, AppState>,
    work_minutes: u16,
    pause_minutes: u16,
) -> Result<(), String> {
    let current = lock(&state.scheduler).configured_rhythm();
    let rhythm = Rhythm::new(
        Minutes(work_minutes),
        Minutes(pause_minutes),
        current.schedule(),
        current.active_days(),
    )
    .map_err(|_| "invalid-rhythm".to_owned())?;
    lock(&state.scheduler)
        .change_rhythm(rhythm)
        .map_err(refusal)?;
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}

#[tauri::command]
fn quit(app: tauri::AppHandle, state: tauri::State<'_, AppState>) {
    let now = state.clock.monotonic();
    lock(&state.scheduler).terminate(now, InterruptionDoor::TrayMenu);
    save_state(&state.persistence, &state.scheduler, &state.clock);
    app.exit(0);
}

struct TickerToggles {
    sounds: Toggle,
    menubar_text: Toggle,
}

fn spawn_ticker(
    app: tauri::AppHandle,
    scheduler: Arc<Mutex<Scheduler>>,
    clock: Clock,
    persistence: Persistence,
    toggles: TickerToggles,
) {
    thread::spawn(move || {
        let monitors = bridge::MonitorCache::default();
        let mut overlay = bridge::TauriOverlay::new(app.clone());
        let displays = bridge::TauriDisplays::new(monitors.clone());
        let mut signals = NullSessionSignals;
        let mut last_served = 0;
        let mut last_phase = CyclePhase::Inactive;
        loop {
            thread::sleep(TICK);
            // Invariant : aucun getter Tauri bloquant tant que `scheduler` est verrouillé
            // (sinon interblocage avec une commande synchrone sur le thread principal).
            monitors.refresh_from_main_thread(&app);
            let now = clock.monotonic();
            let reading = signals.poll(now);
            let snapshot = {
                let mut scheduler = lock(&scheduler);
                scheduler.poll(now, &mut overlay, &displays, reading);
                scheduler.snapshot()
            };
            if snapshot.served_breaks != last_served {
                last_served = snapshot.served_breaks;
                save_state(&persistence, &scheduler, &clock);
            }
            update_tray_title(
                &app,
                &snapshot,
                now,
                toggles.menubar_text.load(Ordering::Relaxed),
            );
            if snapshot.phase != last_phase {
                if toggles.sounds.load(Ordering::Relaxed) {
                    chime_for_transition(last_phase, snapshot.phase);
                }
                last_phase = snapshot.phase;
            }
        }
    });
}

fn update_tray_title(
    app: &tauri::AppHandle,
    snapshot: &breeze_app::CycleSnapshot,
    now: breeze_domain::Instant,
    show_countdown: bool,
) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return;
    };
    let title = if show_countdown {
        countdown_title(snapshot, now)
    } else {
        String::new()
    };
    let _ = tray.set_title(Some(title));
}

fn countdown_title(snapshot: &breeze_app::CycleSnapshot, now: breeze_domain::Instant) -> String {
    match snapshot.phase {
        CyclePhase::Working | CyclePhase::Notice | CyclePhase::Break => snapshot
            .deadline
            .map(|deadline| {
                let secs = deadline.elapsed_since(now).as_secs();
                format!("{}:{:02}", secs / 60, secs % 60)
            })
            .unwrap_or_default(),
        _ => String::new(),
    }
}

// Carillon discret aux bornes de la pause. Non bloquant ; l'échec est ignoré.
#[cfg(target_os = "macos")]
fn chime_for_transition(previous: CyclePhase, current: CyclePhase) {
    let sound = if current == CyclePhase::Break {
        Some("/System/Library/Sounds/Glass.aiff")
    } else if previous == CyclePhase::Break {
        Some("/System/Library/Sounds/Ping.aiff")
    } else {
        None
    };
    if let Some(path) = sound {
        let _ = std::process::Command::new("/usr/bin/afplay")
            .arg(path)
            .spawn();
    }
}

#[cfg(not(target_os = "macos"))]
fn chime_for_transition(_previous: CyclePhase, _current: CyclePhase) {}

fn default_rhythm() -> Rhythm {
    Rhythm::new(
        Minutes(DEFAULT_WORK),
        Minutes(DEFAULT_PAUSE),
        None,
        ActiveDays::everyday(),
    )
    .expect("the default rhythm is within bounds")
}

fn restore(saved: Option<breeze_ports::PersistedState>) -> (Rhythm, Severity, PostureDebt) {
    let Some(saved) = saved else {
        return (default_rhythm(), Severity::Simple, PostureDebt::none());
    };
    let days = ActiveDays::from_mask(saved.active_days).unwrap_or_else(|_| ActiveDays::everyday());
    let schedule = match (saved.schedule_start, saved.schedule_end) {
        (Some(start), Some(end)) => breeze_domain::TimeRange::from_minutes(start, end).ok(),
        _ => None,
    };
    let rhythm = Rhythm::new(
        Minutes(saved.work_minutes),
        Minutes(saved.pause_minutes),
        schedule,
        days,
    )
    .unwrap_or_else(|_| default_rhythm());
    let debt = PostureDebt::restore(Duration::from_secs(u64::from(saved.debt_seconds)));
    (rhythm, saved.severity, debt)
}

fn open_store(app: &tauri::App) -> SqliteStore {
    let dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir());
    let _ = std::fs::create_dir_all(&dir);
    match SqliteStore::open(&dir.join("breeze.sqlite3")) {
        Ok(store) => store,
        Err(error) => {
            eprintln!("breeze: falling back to an in-memory store: {}", error.0);
            SqliteStore::in_memory().expect("an in-memory store always opens")
        }
    }
}

fn load_saved(store: &SqliteStore) -> Option<breeze_ports::PersistedState> {
    use breeze_ports::PersistencePort;
    match store.load() {
        Ok(saved) => saved,
        Err(error) => {
            eprintln!("breeze: could not read stored state: {}", error.0);
            None
        }
    }
}

fn load_app_statuses(
    persistence: &Persistence,
) -> Vec<(breeze_domain::AppId, breeze_domain::AppStatus)> {
    match persistence.load_app_statuses() {
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("breeze: could not read app statuses: {}", error.0);
            Vec::new()
        }
    }
}

fn graceful_quit(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<AppState>() {
        let now = state.clock.monotonic();
        lock(&state.scheduler).terminate(now, InterruptionDoor::TrayMenu);
        save_state(&state.persistence, &state.scheduler, &state.clock);
    }
    app.exit(0);
}

fn show_panel(app: &tauri::AppHandle) {
    if let Some(panel) = app.get_webview_window("panel") {
        let _ = panel.show();
        let _ = panel.set_focus();
    }
}

// Icône d'état : ouvre le panneau, les réglages, ou quitte proprement. Ce n'est pas
// un port (aucune décision du domaine) — pur adaptateur de l'enveloppe Tauri.
fn build_tray(app: &tauri::App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Ouvrir Breeze", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Réglages…", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quitter Breeze", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &settings, &quit])?;
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => show_panel(app),
            "settings" => {
                if let Err(error) = apps::open_settings(app.clone()) {
                    eprintln!("breeze: could not open settings: {error}");
                }
            }
            "quit" => graceful_quit(app),
            _ => {}
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

fn open_onboarding_if_first_run(app: &tauri::App, persistence: &Persistence) {
    match persistence.is_onboarding_done() {
        Ok(true) => {}
        Ok(false) => spawn_onboarding_window(app.handle()),
        Err(error) => eprintln!("breeze: could not read onboarding flag: {}", error.0),
    }
}

fn spawn_onboarding_window(app: &tauri::AppHandle) {
    let built = tauri::WebviewWindowBuilder::new(
        app,
        "onboarding",
        tauri::WebviewUrl::App("onboarding.html".into()),
    )
    .title("Bienvenue dans Breeze")
    .inner_size(680.0, 820.0)
    .resizable(false)
    .center()
    .focused(true)
    .build();
    if let Err(error) = built {
        eprintln!("breeze: could not open onboarding: {error}");
    }
}

#[cfg(target_os = "macos")]
fn app_adapters() -> (InstalledApps, Accessibility) {
    (
        Arc::new(breeze_bridge_macos::MacInstalledApps::new()),
        Arc::new(breeze_bridge_macos::MacAccessibility),
    )
}

#[cfg(not(target_os = "macos"))]
fn app_adapters() -> (InstalledApps, Accessibility) {
    // Hors macOS : pas de catalogue ni de permission réelle tant que l'adaptateur
    // de la plateforme n'existe pas. Le produit dégrade honnêtement.
    (Arc::new(NullInstalledApps), Arc::new(NullAccessibility))
}

fn flag_or(persistence: &Persistence, key: &str, default: bool) -> bool {
    persistence.flag(key).ok().flatten().unwrap_or(default)
}

fn register_panel_shortcut(app: &tauri::App) {
    let outcome = app
        .global_shortcut()
        .on_shortcut(PANEL_SHORTCUT, |app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                show_panel(app);
            }
        });
    if let Err(error) = outcome {
        eprintln!("breeze: could not register panel shortcut: {error}");
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let store = open_store(app);
            let (rhythm, severity, debt) = restore(load_saved(&store));
            let clock: Clock = Arc::new(SystemClock::new());
            let scheduler = Arc::new(Mutex::new(Scheduler::new(
                Cycle::start(rhythm, severity, clock.monotonic()).with_debt(debt),
            )));
            let persistence: Persistence = Arc::new(store);
            let spared = Arc::new(Mutex::new(SparedApps::from_pairs(load_app_statuses(
                &persistence,
            ))));
            let sounds: Toggle =
                Arc::new(AtomicBool::new(flag_or(&persistence, FLAG_SOUNDS, true)));
            let menubar_text: Toggle = Arc::new(AtomicBool::new(flag_or(
                &persistence,
                FLAG_MENUBAR_TEXT,
                true,
            )));
            let (installed_apps, accessibility) = app_adapters();
            spawn_ticker(
                app.handle().clone(),
                Arc::clone(&scheduler),
                Arc::clone(&clock),
                Arc::clone(&persistence),
                TickerToggles {
                    sounds: Arc::clone(&sounds),
                    menubar_text: Arc::clone(&menubar_text),
                },
            );
            build_tray(app)?;
            register_panel_shortcut(app);
            open_onboarding_if_first_run(app, &persistence);
            app.manage(AppState {
                scheduler,
                clock,
                persistence,
                installed_apps,
                accessibility,
                spared,
                sounds,
                menubar_text,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            suspend,
            resume,
            interrupt_break,
            set_severity,
            set_rhythm,
            quit,
            apps::list_installed_apps,
            apps::set_app_status,
            apps::set_spared_apps,
            apps::request_accessibility,
            apps::accessibility_status,
            apps::open_settings,
            apps::finish_onboarding,
            settings::get_settings,
            settings::set_active_days,
            settings::set_schedule,
            settings::set_update_check,
            settings::reset_settings,
            settings::set_launch_at_login,
            settings::set_sounds,
            settings::set_menubar_mode
        ])
        .build(tauri::generate_context!())
        .expect("error while building the Breeze desktop host")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                if let Some(state) = app.try_state::<AppState>() {
                    let now = state.clock.monotonic();
                    lock(&state.scheduler).terminate(now, InterruptionDoor::Quit);
                    save_state(&state.persistence, &state.scheduler, &state.clock);
                }
            }
        });
}
