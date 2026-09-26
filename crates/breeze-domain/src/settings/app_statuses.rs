use crate::command_error::CommandError;
use crate::settings::app_id::AppId;
use crate::settings::app_status::AppStatus;
use crate::settings::safety_list::SafetyList;
use crate::settings::spared_apps::SparedApps;
use crate::settings::status_change::StatusChange;

// Statuts d'applications d'un cycle : ceux que l'utilisateur a CHOISIS, ceux qui sont
// EFFECTIFS pour le cycle en cours (§10.3), et la liste de sécurité qui prime sur les deux.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct AppStatuses {
    chosen: SparedApps,
    effective: SparedApps,
    safety: SafetyList,
}

impl AppStatuses {
    // Un lancement ouvre un cycle neuf : les choix persistés y valent d'emblée.
    pub fn new(chosen: SparedApps, safety: SafetyList) -> Self {
        AppStatuses {
            effective: chosen.clone(),
            chosen,
            safety,
        }
    }

    pub fn is_locked(&self, id: &AppId) -> bool {
        self.safety.contains(id)
    }

    pub fn safety_list(&self) -> &SafetyList {
        &self.safety
    }

    pub fn chosen_status(&self, id: &AppId) -> AppStatus {
        if self.is_locked(id) {
            return AppStatus::Spared;
        }
        self.chosen.status_of(id)
    }

    pub fn effective_status(&self, id: &AppId) -> AppStatus {
        if self.is_locked(id) {
            return AppStatus::Spared;
        }
        self.effective.status_of(id)
    }

    // Le statut choisi attend-il encore le cycle suivant (affaiblissement en attente) ?
    pub fn applies_next_cycle(&self, id: &AppId) -> bool {
        self.chosen_status(id) != self.effective_status(id)
    }

    // Les choix tels qu'ils se persistent (écarts au défaut Bloquée, hors liste de sécurité).
    pub fn chosen(&self) -> &SparedApps {
        &self.chosen
    }

    pub fn choose(&mut self, id: AppId, status: AppStatus) -> Result<StatusChange, CommandError> {
        if self.is_locked(&id) {
            return Err(CommandError::LockedApp);
        }
        self.chosen.set(id.clone(), status);
        if status.weakens(self.effective.status_of(&id)) {
            return Ok(StatusChange::AppliesNextCycle);
        }
        self.effective.set(id, status);
        Ok(StatusChange::AppliesNow)
    }

    // Retour d'usine : tout redevient Bloquée, ce qui renforce toujours — donc tout de suite.
    pub fn reset(&mut self) {
        self.chosen = SparedApps::new();
        self.effective = SparedApps::new();
    }

    // Passage au travail suivant : les choix en attente deviennent effectifs.
    pub fn roll_over(&mut self) {
        self.effective = self.chosen.clone();
    }
}
