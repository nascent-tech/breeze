mod dto;

use breeze_app::Scheduler;
use breeze_bridge_null::{NullDisplays, NullOverlay};
use breeze_domain::constants::{MINUTES_PER_DAY, SECONDS_PER_MINUTE};
use breeze_domain::{ActiveDays, CommandError, Cycle, Instant, Minutes, Rhythm, Severity};
use core::time::Duration;
use dto::{to_dto, SnapshotDto};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant as SystemInstant;

const TICK: Duration = Duration::from_millis(250);

struct AppState {
    scheduler: Arc<Mutex<Scheduler>>,
    started: SystemInstant,
    rhythm: Rhythm,
}

fn now_since(started: SystemInstant) -> Instant {
    let millis = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    Instant::at_millis(millis)
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
    }
    .to_owned()
}

fn lock(scheduler: &Mutex<Scheduler>) -> std::sync::MutexGuard<'_, Scheduler> {
    scheduler
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[tauri::command]
fn get_snapshot(state: tauri::State<'_, AppState>) -> SnapshotDto {
    let now = now_since(state.started);
    to_dto(lock(&state.scheduler).snapshot(), now, &state.rhythm)
}

#[tauri::command]
fn suspend(state: tauri::State<'_, AppState>, minutes: u64) -> Result<(), String> {
    let now = now_since(state.started);
    let capped = minutes.clamp(1, u64::from(MINUTES_PER_DAY));
    let resume_at = now.plus(Duration::from_secs(
        capped.saturating_mul(SECONDS_PER_MINUTE),
    ));
    lock(&state.scheduler)
        .suspend(now, resume_at)
        .map_err(refusal)
}

#[tauri::command]
fn resume(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let now = now_since(state.started);
    lock(&state.scheduler).resume(now).map_err(refusal)
}

#[tauri::command]
fn set_severity(state: tauri::State<'_, AppState>, severity: String) -> Result<(), String> {
    let parsed = parse_severity(&severity).ok_or_else(|| "unknown-severity".to_owned())?;
    lock(&state.scheduler)
        .change_severity(parsed)
        .map_err(refusal)
}

#[tauri::command]
fn quit(app: tauri::AppHandle) {
    app.exit(0);
}

fn spawn_ticker(scheduler: Arc<Mutex<Scheduler>>, started: SystemInstant) {
    thread::spawn(move || {
        let mut overlay = NullOverlay;
        let displays = NullDisplays;
        loop {
            thread::sleep(TICK);
            let now = now_since(started);
            lock(&scheduler).poll(now, &mut overlay, &displays);
        }
    });
}

pub fn run() {
    let rhythm = Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday())
        .expect("the default rhythm is within bounds");
    let started = SystemInstant::now();
    let scheduler = Arc::new(Mutex::new(Scheduler::new(Cycle::start(
        rhythm,
        Severity::Simple,
        Instant::EPOCH,
    ))));
    spawn_ticker(Arc::clone(&scheduler), started);

    tauri::Builder::default()
        .manage(AppState {
            scheduler,
            started,
            rhythm,
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            suspend,
            resume,
            set_severity,
            quit
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Breeze desktop host");
}
