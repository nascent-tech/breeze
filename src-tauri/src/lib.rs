mod bridge;
mod dto;

use breeze_app::Scheduler;
use breeze_bridge_common::{SqliteStore, SystemClock};
use breeze_domain::constants::{MINUTES_PER_DAY, SECONDS_PER_MINUTE};
use breeze_domain::{
    ActiveDays, CommandError, Cycle, InterruptionDoor, Minutes, PostureDebt, Rhythm, Severity,
};
use breeze_ports::ClockPort;
use core::time::Duration;
use dto::{to_dto, SnapshotDto};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::Manager;

const TICK: Duration = Duration::from_millis(250);
const DEFAULT_WORK: u16 = 50;
const DEFAULT_PAUSE: u16 = 10;

type Persistence = Arc<dyn breeze_ports::PersistencePort + Send + Sync>;
type Clock = Arc<dyn ClockPort + Send + Sync>;

struct AppState {
    scheduler: Arc<Mutex<Scheduler>>,
    clock: Clock,
    persistence: Persistence,
}

fn parse_severity(name: &str) -> Option<Severity> {
    match name {
        "Simple" => Some(Severity::Simple),
        "Hardcore" => Some(Severity::Hardcore),
        _ => None,
    }
}

fn refusal(error: CommandError) -> String {
    match error {
        CommandError::BreakDue => "break-due",
        CommandError::NotSuspendable => "not-suspendable",
        CommandError::NotSuspended => "not-suspended",
        CommandError::NotInterruptible => "not-interruptible",
    }
    .to_owned()
}

fn lock(scheduler: &Mutex<Scheduler>) -> std::sync::MutexGuard<'_, Scheduler> {
    scheduler
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn save_state(persistence: &Persistence, scheduler: &Mutex<Scheduler>, clock: &Clock) {
    let (rhythm, severity, served, debt) = {
        let scheduler = lock(scheduler);
        (
            scheduler.configured_rhythm(),
            scheduler.chosen_severity(),
            scheduler.snapshot().served_breaks,
            scheduler.debt(),
        )
    };
    let state = breeze_ports::PersistedState {
        work_minutes: rhythm.work().count(),
        pause_minutes: rhythm.pause().count(),
        severity,
        served_breaks: served,
        debt_seconds: u32::try_from(debt.total().as_secs()).unwrap_or(u32::MAX),
        debt_recorded_at_unix: clock.wall().as_unix_secs(),
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

fn spawn_ticker(
    app: tauri::AppHandle,
    scheduler: Arc<Mutex<Scheduler>>,
    clock: Clock,
    persistence: Persistence,
) {
    thread::spawn(move || {
        let monitors = bridge::MonitorCache::default();
        let mut overlay = bridge::TauriOverlay::new(app.clone());
        let displays = bridge::TauriDisplays::new(monitors.clone());
        let mut last_served = 0;
        loop {
            thread::sleep(TICK);
            // Invariant : aucun getter Tauri bloquant tant que `scheduler` est verrouillé
            // (sinon interblocage avec une commande synchrone sur le thread principal).
            monitors.refresh_from_main_thread(&app);
            let now = clock.monotonic();
            let served = lock(&scheduler)
                .poll(now, &mut overlay, &displays)
                .served_breaks;
            if served != last_served {
                last_served = served;
                save_state(&persistence, &scheduler, &clock);
            }
        }
    });
}

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
    let rhythm = Rhythm::new(
        Minutes(saved.work_minutes),
        Minutes(saved.pause_minutes),
        None,
        ActiveDays::everyday(),
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let store = open_store(app);
            let (rhythm, severity, debt) = restore(load_saved(&store));
            let clock: Clock = Arc::new(SystemClock::new());
            let scheduler = Arc::new(Mutex::new(Scheduler::new(
                Cycle::start(rhythm, severity, clock.monotonic()).with_debt(debt),
            )));
            let persistence: Persistence = Arc::new(store);
            spawn_ticker(
                app.handle().clone(),
                Arc::clone(&scheduler),
                Arc::clone(&clock),
                Arc::clone(&persistence),
            );
            app.manage(AppState {
                scheduler,
                clock,
                persistence,
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
            quit
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
