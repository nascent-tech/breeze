use crate::journal::Journal;
use crate::local_time::{local_date_of, local_now};
use crate::{panel, windows, Accessibility, AppState, Clock, InstalledApps, Persistence};
use crate::{FLAG_MENUBAR_TEXT, FLAG_SOUNDS};
use breeze_app::Scheduler;
use breeze_bridge_common::{SqliteStore, SystemClock};
use breeze_domain::{ActiveDays, AppId, AppStatus, Cycle, Minutes, PostureDebt, Rhythm, Severity};
use breeze_domain::{SparedApps, TimeRange};
use breeze_ports::{PersistedState, PersistencePort};
use chrono::NaiveDate;
use core::time::Duration;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{App, AppHandle, Manager};

pub const DEFAULT_WORK: u16 = 50;
pub const DEFAULT_PAUSE: u16 = 10;
const TRAY_WAIT_STEP: Duration = Duration::from_millis(125);
const TRAY_WAIT_ATTEMPTS: u32 = 40;

pub fn default_rhythm() -> Rhythm {
    Rhythm::new(
        Minutes(DEFAULT_WORK),
        Minutes(DEFAULT_PAUSE),
        None,
        ActiveDays::everyday(),
    )
    .expect("the default rhythm is within bounds")
}

// La dette de posture s'efface à minuit, heure locale (§9.2) : une dette écrite un jour
// antérieur ne revient pas au lancement.
fn debt_of(saved: &PersistedState, today: NaiveDate) -> PostureDebt {
    if local_date_of(saved.debt_recorded_at_unix) != Some(today) {
        return PostureDebt::none();
    }
    PostureDebt::restore(Duration::from_secs(u64::from(saved.debt_seconds)))
}

fn restore(saved: Option<PersistedState>, today: NaiveDate) -> (Rhythm, Severity, PostureDebt) {
    let Some(saved) = saved else {
        return (default_rhythm(), Severity::Simple, PostureDebt::none());
    };
    let days = ActiveDays::from_mask(saved.active_days).unwrap_or_else(|_| ActiveDays::everyday());
    let schedule = match (saved.schedule_start, saved.schedule_end) {
        (Some(start), Some(end)) => TimeRange::from_minutes(start, end).ok(),
        _ => None,
    };
    let rhythm = Rhythm::new(
        Minutes(saved.work_minutes),
        Minutes(saved.pause_minutes),
        schedule,
        days,
    )
    .unwrap_or_else(|_| default_rhythm());
    (rhythm, saved.severity, debt_of(&saved, today))
}

fn open_store(app: &App) -> SqliteStore {
    let dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir());
    if let Err(error) = std::fs::create_dir_all(&dir) {
        eprintln!("breeze: could not create {}: {error}", dir.display());
    }
    match SqliteStore::open(&dir.join("breeze.sqlite3")) {
        Ok(store) => store,
        Err(error) => {
            eprintln!("breeze: falling back to an in-memory store: {}", error.0);
            SqliteStore::in_memory().expect("an in-memory store always opens")
        }
    }
}

fn load_saved(store: &SqliteStore) -> Option<PersistedState> {
    match store.load() {
        Ok(saved) => saved,
        Err(error) => {
            eprintln!("breeze: could not read stored state: {}", error.0);
            None
        }
    }
}

fn load_app_statuses(persistence: &Persistence) -> Vec<(AppId, AppStatus)> {
    match persistence.load_app_statuses() {
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("breeze: could not read app statuses: {}", error.0);
            Vec::new()
        }
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
    (
        Arc::new(breeze_bridge_null::NullInstalledApps),
        Arc::new(breeze_bridge_null::NullAccessibility),
    )
}

pub fn flag_or(persistence: &Persistence, key: &str, default: bool) -> bool {
    match persistence.flag(key) {
        Ok(value) => value.unwrap_or(default),
        Err(error) => {
            eprintln!("breeze: could not read flag {key}: {}", error.0);
            default
        }
    }
}

pub fn app_state(app: &App) -> AppState {
    let store = open_store(app);
    let (rhythm, severity, debt) = restore(load_saved(&store), local_now().date);
    let clock: Clock = Arc::new(SystemClock::new());
    let scheduler =
        Scheduler::new(Cycle::start(rhythm, severity, clock.monotonic()).with_debt(debt));
    let persistence: Persistence = Arc::new(store);
    let spared = SparedApps::from_pairs(load_app_statuses(&persistence));
    let (installed_apps, accessibility) = app_adapters();
    AppState {
        scheduler: Arc::new(Mutex::new(scheduler)),
        persist_lock: Mutex::new(()),
        journal: Mutex::new(Journal::default()),
        sounds: Arc::new(AtomicBool::new(flag_or(&persistence, FLAG_SOUNDS, true))),
        menubar_text: Arc::new(AtomicBool::new(flag_or(
            &persistence,
            FLAG_MENUBAR_TEXT,
            true,
        ))),
        spared: Arc::new(Mutex::new(spared)),
        clock,
        persistence,
        installed_apps,
        accessibility,
    }
}

fn has_area(rect: &tauri::Rect) -> bool {
    rect.size.to_physical::<f64>(1.0).width > 0.0
}

// L'icône d'état n'a de place dans la barre de menus qu'une fois la boucle d'événements
// lancée : on attend qu'elle en ait une pour ouvrir le panneau dessous, une seule fois.
fn reveal_panel_when_tray_is_placed(app: AppHandle) {
    thread::spawn(move || {
        for _ in 0..TRAY_WAIT_ATTEMPTS {
            thread::sleep(TRAY_WAIT_STEP);
            if let Some(rect) = panel::tray_rect(&app).filter(has_area) {
                panel::show_panel(&app, Some(rect));
                return;
            }
        }
        panel::show_panel(&app, None);
    });
}

// Premier lancement : l'accueil, seul. Ensuite : le panneau sous l'icône, une fois.
pub fn first_scene(app: &AppHandle) {
    let onboarded = match app.state::<AppState>().persistence.is_onboarding_done() {
        Ok(done) => done,
        Err(error) => {
            eprintln!("breeze: could not read onboarding flag: {}", error.0);
            true
        }
    };
    if onboarded {
        reveal_panel_when_tray_is_placed(app.clone());
    } else if let Err(error) = windows::open_onboarding(app) {
        eprintln!("breeze: could not open onboarding: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, TimeZone};

    fn saved_on(recorded_at: u64) -> PersistedState {
        PersistedState {
            work_minutes: DEFAULT_WORK,
            pause_minutes: DEFAULT_PAUSE,
            severity: Severity::Simple,
            served_breaks: 0,
            debt_seconds: 480,
            debt_recorded_at_unix: recorded_at,
            active_days: 0b0111_1111,
            schedule_start: None,
            schedule_end: None,
        }
    }

    fn noon(day: u32) -> u64 {
        let local = Local.with_ymd_and_hms(2026, 9, day, 12, 0, 0).unwrap();
        u64::try_from(local.timestamp()).unwrap()
    }

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, day).unwrap()
    }

    #[test]
    fn a_debt_recorded_today_comes_back_at_launch() {
        let (_, _, debt) = restore(Some(saved_on(noon(26))), date(26));
        assert_eq!(debt.total(), Duration::from_secs(480));
    }

    #[test]
    fn a_debt_recorded_before_midnight_is_gone_the_next_day() {
        let (_, _, debt) = restore(Some(saved_on(noon(25))), date(26));
        assert!(debt.is_none());
    }
}
