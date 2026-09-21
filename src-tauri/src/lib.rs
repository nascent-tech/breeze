use breeze_app::{CyclePhase, CycleSnapshot, Scheduler};
use breeze_bridge_null::{NullDisplays, NullOverlay};
use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{ActiveDays, Cycle, Instant, Minutes, Rhythm, Severity};
use core::time::Duration;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant as SystemInstant;

const TICK: Duration = Duration::from_millis(250);

struct AppState {
    scheduler: Arc<Mutex<Scheduler>>,
    started: SystemInstant,
    rhythm: Rhythm,
    severity: Severity,
}

#[derive(Serialize)]
struct SnapshotDto {
    phase: String,
    remaining_secs: u64,
    total_secs: u64,
    break_in_secs: u64,
    severity: String,
    served_breaks: u32,
}

fn now_since(started: SystemInstant) -> Instant {
    let millis = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    Instant::at_millis(millis)
}

fn phase_name(phase: CyclePhase) -> &'static str {
    match phase {
        CyclePhase::Inactive => "Inactive",
        CyclePhase::Working => "Working",
        CyclePhase::Notice => "Notice",
        CyclePhase::Break => "Break",
        CyclePhase::Returning => "Returning",
    }
}

fn phase_total(phase: CyclePhase, rhythm: &Rhythm) -> Duration {
    match phase {
        CyclePhase::Working => rhythm.work().as_duration(),
        CyclePhase::Notice => NOTICE,
        CyclePhase::Break => rhythm.pause().as_duration(),
        CyclePhase::Returning => RETURN_HOLD,
        CyclePhase::Inactive => Duration::ZERO,
    }
}

fn severity_name(severity: Severity) -> String {
    match severity {
        Severity::Simple => "Simple".to_owned(),
        Severity::Hardcore => "Hardcore".to_owned(),
    }
}

fn seconds_until_break(phase: CyclePhase, remaining: u64) -> u64 {
    match phase {
        CyclePhase::Working => remaining.saturating_add(NOTICE.as_secs()),
        CyclePhase::Notice => remaining,
        _ => 0,
    }
}

fn to_dto(
    snapshot: CycleSnapshot,
    now: Instant,
    rhythm: &Rhythm,
    severity: Severity,
) -> SnapshotDto {
    let remaining = snapshot
        .deadline
        .map_or(0, |deadline| deadline.elapsed_since(now).as_secs());
    SnapshotDto {
        phase: phase_name(snapshot.phase).to_owned(),
        remaining_secs: remaining,
        total_secs: phase_total(snapshot.phase, rhythm).as_secs(),
        break_in_secs: seconds_until_break(snapshot.phase, remaining),
        severity: severity_name(severity),
        served_breaks: snapshot.served_breaks,
    }
}

#[tauri::command]
fn get_snapshot(state: tauri::State<'_, AppState>) -> SnapshotDto {
    let now = now_since(state.started);
    let scheduler = state
        .scheduler
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    to_dto(scheduler.snapshot(), now, &state.rhythm, state.severity)
}

fn spawn_ticker(scheduler: Arc<Mutex<Scheduler>>, started: SystemInstant) {
    thread::spawn(move || {
        let mut overlay = NullOverlay;
        let displays = NullDisplays;
        loop {
            thread::sleep(TICK);
            let now = now_since(started);
            let mut guard = scheduler
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            guard.poll(now, &mut overlay, &displays);
        }
    });
}

pub fn run() {
    let rhythm = Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday())
        .expect("the default rhythm is within bounds");
    let severity = Severity::Simple;
    let started = SystemInstant::now();
    let scheduler = Arc::new(Mutex::new(Scheduler::new(Cycle::start(
        rhythm,
        severity,
        Instant::EPOCH,
    ))));
    spawn_ticker(Arc::clone(&scheduler), started);

    let state = AppState {
        scheduler,
        started,
        rhythm,
        severity,
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running the Breeze desktop host");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rhythm() -> Rhythm {
        Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
    }

    #[test]
    fn a_fresh_working_cycle_reports_break_after_work_plus_notice() {
        let r = rhythm();
        let snapshot = CycleSnapshot::of(&Cycle::start(r, Severity::Simple, Instant::EPOCH));
        let dto = to_dto(snapshot, Instant::EPOCH, &r, Severity::Simple);
        assert_eq!(dto.phase, "Working");
        assert_eq!(dto.total_secs, r.work().as_duration().as_secs());
        assert_eq!(
            dto.break_in_secs,
            r.work().as_duration().as_secs() + NOTICE.as_secs()
        );
        assert_eq!(dto.severity, "Simple");
    }

    #[test]
    fn the_reported_severity_is_the_configured_one_even_while_working() {
        let r = rhythm();
        let snapshot = CycleSnapshot::of(&Cycle::start(r, Severity::Hardcore, Instant::EPOCH));
        let dto = to_dto(snapshot, Instant::EPOCH, &r, Severity::Hardcore);
        assert_eq!(dto.severity, "Hardcore");
    }
}
