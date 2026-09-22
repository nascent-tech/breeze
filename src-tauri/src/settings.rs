use crate::{lock, refusal, save_state, AppState};
use breeze_domain::{ActiveDays, Minutes, Rhythm, Severity, TimeRange};
use serde::Serialize;
use tauri::State;

const DEFAULT_START: u16 = 9 * 60;
const DEFAULT_END: u16 = 18 * 60 + 30;
const RESET_WORK: u16 = 50;
const RESET_PAUSE: u16 = 10;

#[derive(Serialize)]
pub struct SettingsDto {
    pub work_minutes: u16,
    pub pause_minutes: u16,
    pub severity: &'static str,
    pub active_days: u8,
    pub schedule_enabled: bool,
    pub schedule_start: u16,
    pub schedule_end: u16,
    pub update_check: bool,
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Simple => "Simple",
        Severity::Hardcore => "Hardcore",
    }
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> SettingsDto {
    let (rhythm, severity) = {
        let scheduler = lock(&state.scheduler);
        (scheduler.configured_rhythm(), scheduler.chosen_severity())
    };
    let schedule = rhythm.schedule();
    SettingsDto {
        work_minutes: rhythm.work().count(),
        pause_minutes: rhythm.pause().count(),
        severity: severity_label(severity),
        active_days: rhythm.active_days().mask(),
        schedule_enabled: schedule.is_some(),
        schedule_start: schedule.map_or(DEFAULT_START, TimeRange::start),
        schedule_end: schedule.map_or(DEFAULT_END, TimeRange::end),
        update_check: state.persistence.is_update_check_enabled().unwrap_or(true),
    }
}

fn apply_rhythm(state: &State<'_, AppState>, rhythm: Rhythm) -> Result<(), String> {
    lock(&state.scheduler)
        .change_rhythm(rhythm)
        .map_err(refusal)?;
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}

#[tauri::command]
pub fn set_active_days(state: State<'_, AppState>, mask: u8) -> Result<(), String> {
    let days = ActiveDays::from_mask(mask).map_err(|_| "invalid-active-days".to_owned())?;
    let current = lock(&state.scheduler).configured_rhythm();
    let rhythm = Rhythm::new(current.work(), current.pause(), current.schedule(), days)
        .map_err(|_| "invalid-rhythm".to_owned())?;
    apply_rhythm(&state, rhythm)
}

#[tauri::command]
pub fn set_schedule(
    state: State<'_, AppState>,
    enabled: bool,
    start: u16,
    end: u16,
) -> Result<(), String> {
    let schedule = if enabled {
        Some(TimeRange::from_minutes(start, end).map_err(|_| "invalid-schedule".to_owned())?)
    } else {
        None
    };
    let current = lock(&state.scheduler).configured_rhythm();
    let rhythm = Rhythm::new(
        current.work(),
        current.pause(),
        schedule,
        current.active_days(),
    )
    .map_err(|_| "invalid-rhythm".to_owned())?;
    apply_rhythm(&state, rhythm)
}

#[tauri::command]
pub fn set_update_check(state: State<'_, AppState>, enabled: bool) -> Result<(), String> {
    state
        .persistence
        .set_update_check(enabled)
        .map_err(|error| {
            eprintln!("breeze: could not persist update-check flag: {}", error.0);
            "persistence-failed".to_owned()
        })
}

#[tauri::command]
pub fn reset_settings(state: State<'_, AppState>) -> Result<(), String> {
    let rhythm = Rhythm::new(
        Minutes(RESET_WORK),
        Minutes(RESET_PAUSE),
        None,
        ActiveDays::everyday(),
    )
    .map_err(|_| "invalid-rhythm".to_owned())?;
    {
        let mut scheduler = lock(&state.scheduler);
        scheduler.change_rhythm(rhythm).map_err(refusal)?;
        let _ = scheduler.change_severity(Severity::Simple);
    }
    let mut spared = state.spared.lock().unwrap_or_else(|e| e.into_inner());
    let snapshot = spared.clone();
    *spared = breeze_domain::SparedApps::new();
    if let Err(error) = state.persistence.replace_app_statuses(&[]) {
        *spared = snapshot;
        eprintln!("breeze: could not clear app statuses: {}", error.0);
        return Err("persistence-failed".to_owned());
    }
    let _ = state.persistence.set_update_check(true);
    save_state(&state.persistence, &state.scheduler, &state.clock);
    Ok(())
}
