use crate::{lock, refusal, AppState};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use breeze_domain::{AppId, AppStatus, AppStatuses, CommandError, StatusChange};
use serde::Serialize;
use std::collections::HashMap;
use tauri::State;

#[derive(Serialize)]
pub struct InstalledAppDto {
    pub bundle_id: String,
    pub name: String,
    // Data URL PNG déjà rendu ; None = l'UI retombe sur la lettre.
    pub icon_data_url: Option<String>,
    // Statut CHOISI ; `applies_next_cycle` dit s'il attend encore le cycle suivant.
    pub status: &'static str,
    // Application de la liste de sécurité : toujours "Spared", non éditable.
    pub locked: bool,
    pub applies_next_cycle: bool,
}

#[derive(Serialize)]
pub struct AppStatusChangeDto {
    pub applies_next_cycle: bool,
}

fn status_label(status: AppStatus) -> &'static str {
    match status {
        AppStatus::Blocked => "Blocked",
        AppStatus::Spared => "Spared",
        AppStatus::Ignored => "Ignored",
    }
}

fn parse_status(name: &str) -> Option<AppStatus> {
    match name {
        "Blocked" => Some(AppStatus::Blocked),
        "Spared" => Some(AppStatus::Spared),
        "Ignored" => Some(AppStatus::Ignored),
        _ => None,
    }
}

fn to_data_url(png: &[u8]) -> String {
    format!("data:image/png;base64,{}", STANDARD.encode(png))
}

#[tauri::command]
pub async fn list_installed_apps(
    state: State<'_, AppState>,
) -> Result<Vec<InstalledAppDto>, String> {
    // Énumération lente (sips par app) tenue hors du fil UI.
    let port = state.installed_apps.clone();
    let apps = tauri::async_runtime::spawn_blocking(move || port.installed_apps())
        .await
        .map_err(|_| "enumeration-cancelled".to_owned())?
        .map_err(|error| {
            eprintln!("breeze: could not enumerate installed apps: {}", error.0);
            "enumeration-failed".to_owned()
        })?;
    let statuses = lock(&state.scheduler).app_statuses().clone();
    Ok(apps
        .into_iter()
        .map(|app| InstalledAppDto {
            status: status_label(statuses.chosen_status(&app.id)),
            locked: statuses.is_locked(&app.id),
            applies_next_cycle: statuses.applies_next_cycle(&app.id),
            bundle_id: app.id.as_str().to_owned(),
            name: app.name,
            icon_data_url: app.icon_png.as_deref().map(to_data_url),
        })
        .collect())
}

// Le disque d'abord, sur une copie : le cycle ne change que si l'écriture a réussi. Le
// verrou du scheduler reste tenu du début à la fin, pour que la copie et le cycle voient
// les mêmes statuts et que l'application donne exactement le résultat écrit. Le refus
// « pause due » (§10.3) tombe avant toute écriture, comme pour le rythme et la sévérité.
fn choose_and_persist(
    state: &AppState,
    choices: &[(AppId, AppStatus)],
    what: &'static str,
) -> Result<Vec<StatusChange>, String> {
    let mut scheduler = lock(&state.scheduler);
    if scheduler.break_is_due() {
        return Err(refusal(CommandError::BreakDue));
    }
    let mut next = scheduler.app_statuses().clone();
    let changes = choices
        .iter()
        .map(|(id, status)| next.choose(id.clone(), *status))
        .collect::<Result<Vec<_>, _>>()
        .map_err(refusal)?;
    if let Err(error) = state
        .persistence
        .replace_app_statuses(&next.chosen().pairs())
    {
        eprintln!("breeze: could not persist {what}: {}", error.0);
        return Err("persistence-failed".to_owned());
    }
    for (id, status) in choices {
        scheduler
            .set_app_status(id.clone(), *status)
            .map_err(refusal)?;
    }
    Ok(changes)
}

#[tauri::command]
pub fn set_app_status(
    state: State<'_, AppState>,
    bundle_id: String,
    status: String,
) -> Result<AppStatusChangeDto, String> {
    let id = AppId::parse(&bundle_id).map_err(|_| "invalid-app-id".to_owned())?;
    let status = parse_status(&status).ok_or_else(|| "unknown-status".to_owned())?;
    let changes = choose_and_persist(&state, &[(id, status)], "app status")?;
    Ok(AppStatusChangeDto {
        applies_next_cycle: changes.contains(&StatusChange::AppliesNextCycle),
    })
}

// Les choix de l'accueil : un id invalide ou une app de la liste de sécurité est ignoré
// (geste large, jamais fatal) ; une app absente de la carte garde son statut.
fn spared_choices(
    statuses: &AppStatuses,
    choices: HashMap<String, bool>,
) -> Vec<(AppId, AppStatus)> {
    choices
        .into_iter()
        .filter_map(|(raw_id, is_spared)| {
            let id = AppId::parse(&raw_id).ok()?;
            let status = if is_spared {
                AppStatus::Spared
            } else {
                AppStatus::Blocked
            };
            (!statuses.is_locked(&id)).then_some((id, status))
        })
        .collect()
}

#[tauri::command]
pub fn set_spared_apps(
    state: State<'_, AppState>,
    spared: HashMap<String, bool>,
) -> Result<(), String> {
    let statuses = lock(&state.scheduler).app_statuses().clone();
    let choices = spared_choices(&statuses, spared);
    choose_and_persist(&state, &choices, "spared apps").map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use breeze_domain::{SafetyList, SparedApps};

    fn app(raw: &str) -> AppId {
        AppId::parse(raw).unwrap()
    }

    fn statuses() -> AppStatuses {
        AppStatuses::new(
            SparedApps::new(),
            SafetyList::new([app("com.apple.finder")]),
        )
    }

    #[test]
    fn only_the_apps_sent_are_chosen() {
        let choices = spared_choices(
            &statuses(),
            HashMap::from([("com.apple.Music".to_owned(), false)]),
        );
        assert_eq!(choices, vec![(app("com.apple.Music"), AppStatus::Blocked)]);
    }

    #[test]
    fn an_invalid_id_is_skipped_without_failing() {
        let choices = spared_choices(
            &statuses(),
            HashMap::from([
                ("not a bundle id".to_owned(), true),
                ("com.apple.Notes".to_owned(), true),
            ]),
        );
        assert_eq!(choices, vec![(app("com.apple.Notes"), AppStatus::Spared)]);
    }

    #[test]
    fn a_safety_listed_app_sent_by_the_onboarding_is_left_alone() {
        let choices = spared_choices(
            &statuses(),
            HashMap::from([("com.apple.finder".to_owned(), false)]),
        );
        assert!(choices.is_empty());
    }
}
