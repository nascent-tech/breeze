use crate::dto::severity_name;
use crate::startup::{default_rhythm, flag_or};
use crate::{lock, persistence_failed, refusal, AppState, PANEL_SHORTCUT_LABEL};
use crate::{FLAG_MENUBAR_TEXT, FLAG_SOUNDS};
use breeze_domain::{ActiveDays, Rhythm, Severity, SparedApps, TimeRange};
use breeze_ports::PersistedState;
use serde::Serialize;
use std::sync::atomic::Ordering;
use std::sync::PoisonError;
use tauri::State;
use tauri_plugin_autostart::ManagerExt;

const DEFAULT_START: u16 = 9 * 60;
const DEFAULT_END: u16 = 18 * 60 + 30;

#[derive(Serialize)]
pub struct SettingsDto {
    pub work_minutes: u16,
    pub pause_minutes: u16,
    pub severity: &'static str,
    pub chosen_severity: &'static str,
    pub severity_pending: bool,
    pub active_days: u8,
    pub schedule_enabled: bool,
    pub schedule_start: u16,
    pub schedule_end: u16,
    pub update_check: bool,
    pub launch_at_login: bool,
    pub sounds: bool,
    pub menubar_text: bool,
    pub shortcut: &'static str,
}

fn update_check_enabled(state: &AppState) -> bool {
    state
        .persistence
        .is_update_check_enabled()
        .unwrap_or_else(|error| {
            eprintln!("breeze: could not read update-check flag: {}", error.0);
            true
        })
}

fn launch_at_login_enabled(app: &tauri::AppHandle) -> bool {
    app.autolaunch().is_enabled().unwrap_or_else(|error| {
        eprintln!("breeze: could not read launch-at-login: {error}");
        false
    })
}

// Plage coupée : on rend les dernières bornes saisies, pour la rallumer telle quelle.
fn schedule_bounds(state: &AppState, active: Option<TimeRange>) -> (u16, u16) {
    if let Some(range) = active {
        return (range.start(), range.end());
    }
    match state.persistence.remembered_schedule() {
        Ok(remembered) => remembered.unwrap_or((DEFAULT_START, DEFAULT_END)),
        Err(error) => {
            eprintln!("breeze: could not read remembered schedule: {}", error.0);
            (DEFAULT_START, DEFAULT_END)
        }
    }
}

#[tauri::command]
pub fn get_settings(app: tauri::AppHandle, state: State<'_, AppState>) -> SettingsDto {
    let (rhythm, snapshot) = {
        let scheduler = lock(&state.scheduler);
        (scheduler.configured_rhythm(), scheduler.snapshot())
    };
    let (start, end) = schedule_bounds(&state, rhythm.schedule());
    let chosen = severity_name(snapshot.chosen_severity);
    SettingsDto {
        work_minutes: rhythm.work().count(),
        pause_minutes: rhythm.pause().count(),
        severity: chosen,
        chosen_severity: chosen,
        severity_pending: snapshot.chosen_severity != snapshot.severity,
        active_days: rhythm.active_days().mask(),
        schedule_enabled: rhythm.schedule().is_some(),
        schedule_start: start,
        schedule_end: end,
        update_check: update_check_enabled(&state),
        launch_at_login: launch_at_login_enabled(&app),
        sounds: flag_or(&state.persistence, FLAG_SOUNDS, true),
        menubar_text: flag_or(&state.persistence, FLAG_MENUBAR_TEXT, true),
        shortcut: PANEL_SHORTCUT_LABEL,
    }
}

// Lit le rythme configuré et applique le rythme reconstruit SOUS UN SEUL VERROU :
// un réglage concurrent (fenêtre onboarding) ne peut pas écraser des champs périmés.
pub(crate) fn rebuild_rhythm<F>(state: &State<'_, AppState>, build: F) -> Result<(), String>
where
    F: FnOnce(Rhythm) -> Result<Rhythm, String>,
{
    {
        let mut scheduler = lock(&state.scheduler);
        let rhythm = build(scheduler.configured_rhythm())?;
        scheduler.change_rhythm(rhythm).map_err(refusal)?;
    }
    state.persist()
}

#[tauri::command]
pub fn set_active_days(state: State<'_, AppState>, mask: u8) -> Result<(), String> {
    let days = ActiveDays::from_mask(mask).map_err(|_| "invalid-active-days".to_owned())?;
    rebuild_rhythm(&state, |current| {
        Rhythm::new(current.work(), current.pause(), current.schedule(), days)
            .map_err(|_| "invalid-rhythm".to_owned())
    })
}

#[tauri::command]
pub fn set_schedule(
    state: State<'_, AppState>,
    enabled: bool,
    start: u16,
    end: u16,
) -> Result<(), String> {
    let bounds = TimeRange::from_minutes(start, end);
    if let Ok(range) = bounds {
        state
            .persistence
            .remember_schedule(range.start(), range.end())
            .map_err(persistence_failed("schedule bounds"))?;
    }
    let schedule = if enabled {
        Some(bounds.map_err(|_| "invalid-schedule".to_owned())?)
    } else {
        None
    };
    rebuild_rhythm(&state, |current| {
        Rhythm::new(
            current.work(),
            current.pause(),
            schedule,
            current.active_days(),
        )
        .map_err(|_| "invalid-rhythm".to_owned())
    })
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

// Atomique vis-à-vis du cycle ET du disque : le refus « pause due » tombe avant toute
// écriture ; l'état d'usine s'écrit en une transaction (état, statuts d'apps, drapeaux)
// pendant que le scheduler reste verrouillé, pour qu'aucune pause ne devienne due entre la
// vérification et l'application ; la mémoire ne change qu'une fois le disque d'accord.
// Verrous pris dans l'ordre documenté : persist_lock → scheduler → spared.
#[tauri::command]
pub fn reset_settings(state: State<'_, AppState>) -> Result<(), String> {
    let _serialized = state
        .persist_lock
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let mut scheduler = lock(&state.scheduler);
    if scheduler.break_is_due() {
        return Err(refusal(breeze_domain::CommandError::BreakDue));
    }
    let mut spared = state.spared.lock().unwrap_or_else(PoisonError::into_inner);
    let factory = PersistedState {
        severity: Severity::Simple,
        ..state.persisted_state(&scheduler)
    };
    let factory = with_rhythm(factory, default_rhythm());
    state
        .persistence
        .reset_preferences(factory)
        .map_err(persistence_failed("factory settings"))?;
    *spared = SparedApps::new();
    scheduler.change_rhythm(default_rhythm()).map_err(refusal)?;
    scheduler
        .change_severity(Severity::Simple)
        .map_err(refusal)?;
    state.sounds.store(true, Ordering::Relaxed);
    state.menubar_text.store(true, Ordering::Relaxed);
    Ok(())
}

fn with_rhythm(state: PersistedState, rhythm: Rhythm) -> PersistedState {
    let schedule = rhythm.schedule();
    PersistedState {
        work_minutes: rhythm.work().count(),
        pause_minutes: rhythm.pause().count(),
        active_days: rhythm.active_days().mask(),
        schedule_start: schedule.map(TimeRange::start),
        schedule_end: schedule.map(TimeRange::end),
        ..state
    }
}

#[tauri::command]
pub fn set_launch_at_login(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let manager = app.autolaunch();
    let outcome = if enabled {
        manager.enable()
    } else {
        manager.disable()
    };
    outcome.map_err(|error| {
        eprintln!("breeze: could not set launch-at-login: {error}");
        "autostart-failed".to_owned()
    })
}

#[tauri::command]
pub fn set_sounds(state: State<'_, AppState>, enabled: bool) -> Result<(), String> {
    state
        .persistence
        .set_flag(FLAG_SOUNDS, enabled)
        .map_err(|error| {
            eprintln!("breeze: could not persist sounds flag: {}", error.0);
            "persistence-failed".to_owned()
        })?;
    state.sounds.store(enabled, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_menubar_mode(state: State<'_, AppState>, text: bool) -> Result<(), String> {
    state
        .persistence
        .set_flag(FLAG_MENUBAR_TEXT, text)
        .map_err(|error| {
            eprintln!("breeze: could not persist menubar mode: {}", error.0);
            "persistence-failed".to_owned()
        })?;
    state.menubar_text.store(text, Ordering::Relaxed);
    Ok(())
}
