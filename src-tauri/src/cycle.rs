use crate::dto::{to_dto, DayContext, SnapshotDto};
use crate::local_time::{local_now, next_start_label, LocalNow};
use crate::settings::rebuild_rhythm;
use crate::{lock, refusal, AppState};
use breeze_app::CyclePhase;
use breeze_domain::constants::{MINUTES_PER_DAY, SECONDS_PER_MINUTE};
use breeze_domain::{InterruptionDoor, Minutes, OffHours, Rhythm, Severity};
use core::time::Duration;
use tauri::{AppHandle, State};

fn parse_severity(name: &str) -> Option<Severity> {
    match name {
        "Simple" => Some(Severity::Simple),
        "Hardcore" => Some(Severity::Hardcore),
        _ => None,
    }
}

fn reason_name(off_hours: OffHours) -> &'static str {
    match off_hours {
        OffHours::InactiveDay => "day",
        OffHours::OutsideSchedule => "schedule",
    }
}

fn day_context(state: &AppState, phase: CyclePhase, rhythm: Rhythm, local: LocalNow) -> DayContext {
    let served_today = state.served_on(local.date);
    if phase != CyclePhase::Inactive {
        return DayContext {
            served_today,
            ..DayContext::default()
        };
    }
    let inactive_reason = rhythm
        .off_hours_at(local.weekday, local.minute_of_day)
        .map(reason_name);
    let next_start = rhythm.next_start_after(local.weekday, local.minute_of_day);
    DayContext {
        served_today,
        inactive_reason,
        next_start_label: inactive_reason.map(|_| next_start_label(next_start)),
    }
}

#[tauri::command]
pub fn get_snapshot(state: State<'_, AppState>) -> SnapshotDto {
    let now = state.clock.monotonic();
    let local = local_now();
    let (snapshot, active, configured) = {
        let scheduler = lock(&state.scheduler);
        (
            scheduler.snapshot(),
            scheduler.active_rhythm(),
            scheduler.configured_rhythm(),
        )
    };
    let day = day_context(&state, snapshot.phase, configured, local);
    to_dto(snapshot, now, &active, &configured, day)
}

#[tauri::command]
pub fn suspend(state: State<'_, AppState>, minutes: u64) -> Result<(), String> {
    let now = state.clock.monotonic();
    let capped = minutes.clamp(1, u64::from(MINUTES_PER_DAY));
    let resume_at = now.plus(Duration::from_secs(
        capped.saturating_mul(SECONDS_PER_MINUTE),
    ));
    lock(&state.scheduler)
        .suspend(now, resume_at)
        .map_err(refusal)?;
    state.persist()
}

#[tauri::command]
pub fn resume(state: State<'_, AppState>) -> Result<(), String> {
    let now = state.clock.monotonic();
    lock(&state.scheduler).resume(now).map_err(refusal)?;
    state.persist()
}

#[tauri::command]
pub fn interrupt_break(state: State<'_, AppState>) -> Result<(), String> {
    let now = state.clock.monotonic();
    lock(&state.scheduler)
        .interrupt_break(now)
        .map_err(refusal)?;
    // La pause est interrompue quoi qu'il arrive au disque : un échec d'écriture est
    // journalisé, pas renvoyé à la surface de pause qui se referme.
    _ = state.persist();
    Ok(())
}

#[tauri::command]
pub fn set_severity(state: State<'_, AppState>, severity: String) -> Result<(), String> {
    let parsed = parse_severity(&severity).ok_or_else(|| "unknown-severity".to_owned())?;
    lock(&state.scheduler)
        .change_severity(parsed)
        .map_err(refusal)?;
    state.persist()
}

#[tauri::command]
pub fn set_rhythm(
    state: State<'_, AppState>,
    work_minutes: u16,
    pause_minutes: u16,
) -> Result<(), String> {
    rebuild_rhythm(&state, |current| {
        Rhythm::new(
            Minutes(work_minutes),
            Minutes(pause_minutes),
            current.schedule(),
            current.active_days(),
        )
        .map_err(|_| "invalid-rhythm".to_owned())
    })
}

#[tauri::command]
pub fn quit(app: AppHandle, state: State<'_, AppState>) {
    state.shut_down(InterruptionDoor::TrayMenu);
    app.exit(0);
}
