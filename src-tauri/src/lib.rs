mod apps;
mod bridge;
mod cycle;
mod dto;
mod journal;
mod ledger;
mod local_time;
mod panel;
mod settings;
mod startup;
mod ticker;
mod tray;
mod windows;

use breeze_app::Scheduler;
use breeze_domain::{CommandError, InterruptionDoor, TimeRange};
use breeze_ports::{
    ClockPort, InstalledAppsPort, PersistedState, PersistenceError, PersistencePort,
};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub(crate) const TRAY_ID: &str = "breeze-tray";
pub(crate) const FLAG_SOUNDS: &str = "sounds";
pub(crate) const FLAG_MENUBAR_TEXT: &str = "menubar_text";
pub(crate) const PANEL_SHORTCUT_LABEL: &str = "⌥⌘B";
const PANEL_SHORTCUT: &str = "Alt+Cmd+B";

type Persistence = Arc<dyn PersistencePort + Send + Sync>;
type Clock = Arc<dyn ClockPort + Send + Sync>;
type InstalledApps = Arc<dyn InstalledAppsPort>;
type Toggle = Arc<AtomicBool>;

// Ordre des verrous, toujours : persist_lock → scheduler. Aucun chemin ne prend le
// scheduler puis persist_lock (le scheduler n'est tenu que le temps d'une lecture ou d'une
// commande, jamais pendant l'appel à `save_state`). Les statuts d'applications vivent dans
// le cycle, sous le verrou du scheduler.
pub(crate) struct AppState {
    pub(crate) scheduler: Arc<Mutex<Scheduler>>,
    // Sérialise lecture + écriture de l'état : deux sauvegardes concurrentes ne peuvent
    // pas écrire dans le désordre un état plus ancien par-dessus un plus récent.
    pub(crate) persist_lock: Mutex<()>,
    pub(crate) journal: Mutex<journal::Journal>,
    pub(crate) clock: Clock,
    pub(crate) persistence: Persistence,
    pub(crate) installed_apps: InstalledApps,
    pub(crate) sounds: Toggle,
    pub(crate) menubar_text: Toggle,
}

pub(crate) fn refusal(error: CommandError) -> String {
    match error {
        CommandError::BreakDue => "break-due",
        CommandError::NotSuspendable => "not-suspendable",
        CommandError::NotSuspended => "not-suspended",
        CommandError::NotInterruptible => "not-interruptible",
        CommandError::LockedApp => "locked-app",
    }
    .to_owned()
}

pub(crate) fn lock(scheduler: &Mutex<Scheduler>) -> MutexGuard<'_, Scheduler> {
    scheduler.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(crate) fn persistence_failed(what: &'static str) -> impl Fn(PersistenceError) -> String {
    move |error| {
        eprintln!("breeze: could not persist {what}: {}", error.0);
        "persistence-failed".to_owned()
    }
}

impl AppState {
    // L'état tel qu'il s'écrit : rythme et sévérité CHOISIS, compteurs, dette datée.
    pub(crate) fn persisted_state(&self, scheduler: &Scheduler) -> PersistedState {
        let rhythm = scheduler.configured_rhythm();
        let schedule = rhythm.schedule();
        PersistedState {
            work_minutes: rhythm.work().count(),
            pause_minutes: rhythm.pause().count(),
            severity: scheduler.chosen_severity(),
            served_breaks: scheduler.snapshot().served_breaks,
            debt_seconds: u32::try_from(scheduler.debt().total().as_secs()).unwrap_or(u32::MAX),
            debt_recorded_at_unix: self.clock.wall().as_unix_secs(),
            active_days: rhythm.active_days().mask(),
            schedule_start: schedule.map(TimeRange::start),
            schedule_end: schedule.map(TimeRange::end),
        }
    }

    pub(crate) fn save_state(&self) -> Result<(), String> {
        let _serialized = self
            .persist_lock
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let state = self.persisted_state(&lock(&self.scheduler));
        self.persistence
            .save(state)
            .map_err(persistence_failed("state"))
    }

    // Après toute commande : le journal d'abord (les sorts nés de la commande), l'état
    // ensuite. L'état mémoire reste appliqué même si l'écriture échoue ; l'appelant
    // reçoit "persistence-failed" pour prévenir l'UI.
    pub(crate) fn persist(&self) -> Result<(), String> {
        self.record_outcomes();
        self.save_state()
    }

    // Sortie propre : l'app quitte avant le tick suivant. Terminer et prélever se font
    // sous le même verrou, pour que le sort de la pause interrompue soit écrit ici.
    pub(crate) fn shut_down(&self, door: InterruptionDoor) {
        let now = self.clock.monotonic();
        let outcomes = lock(&self.scheduler).terminate_and_take(now, door);
        self.journal_outcomes(outcomes);
        // Déjà journalisé par save_state ; rien de plus à faire en sortant.
        _ = self.save_state();
    }
}

pub(crate) fn graceful_quit(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<AppState>() {
        state.shut_down(InterruptionDoor::TrayMenu);
    }
    app.exit(0);
}

fn register_panel_shortcut(app: &tauri::App) {
    let outcome = app
        .global_shortcut()
        .on_shortcut(PANEL_SHORTCUT, |app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                panel::toggle(app, panel::tray_rect(app));
            }
        });
    if let Err(error) = outcome {
        eprintln!("breeze: could not register panel shortcut: {error}");
    }
}

pub fn run() {
    tauri::Builder::default()
        // Enregistré en premier : une seconde ouverture ne démarre pas un second cycle,
        // elle montre le panneau de l'instance déjà lancée.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            panel::show_panel(app, panel::tray_rect(app));
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // App de barre de menus : ni icône dans le Dock, ni entrée dans ⌘⇥.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            app.manage(startup::app_state(app));
            app.manage(panel::PanelFocus::default());
            tray::build(app)?;
            panel::hide_on_blur(app.handle());
            register_panel_shortcut(app);
            ticker::spawn(app.handle().clone());
            startup::first_scene(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cycle::get_snapshot,
            cycle::suspend,
            cycle::resume,
            cycle::interrupt_break,
            cycle::set_severity,
            cycle::set_rhythm,
            cycle::quit,
            apps::list_installed_apps,
            apps::set_app_status,
            apps::set_spared_apps,
            windows::open_settings,
            windows::finish_onboarding,
            windows::reopen_onboarding,
            windows::get_app_info,
            windows::open_url,
            panel::hide_panel,
            ledger::get_stats,
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
                    state.shut_down(InterruptionDoor::Quit);
                }
                // ⌘Q en pause Hardcore : barre de menus et Dock rendus avant la mort du processus.
                #[cfg(target_os = "macos")]
                bridge::native_presentation::restore_default();
            }
        });
}
