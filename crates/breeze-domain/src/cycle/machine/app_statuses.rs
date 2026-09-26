use super::Cycle;
use crate::command_error::CommandError;
use crate::constants::WINDOW_VEIL_CAP;
use crate::cycle::break_mode::BreakMode;
use crate::cycle::degraded_reason::DegradedReason;
use crate::cycle::state::CycleState;
use crate::settings::{AppId, AppStatus, AppStatuses, Severity, StatusChange};

// Statuts d'applications et capacité « cadres observables » : ce que le cycle retient
// pour décider du voile et du gel, sans rien savoir d'une fenêtre réelle.
impl Cycle {
    pub fn with_app_statuses(mut self, apps: AppStatuses) -> Self {
        self.apps = apps;
        self
    }

    pub fn app_statuses(&self) -> &AppStatuses {
        &self.apps
    }

    // Règle du sens (§10.3) : affaiblir attend le cycle suivant, renforcer vaut tout de
    // suite. Comme le rythme et la sévérité, aucun statut ne change pendant qu'une pause
    // est due (préavis, pause, retour). La liste de sécurité (§10.6) refuse toute
    // modification.
    pub fn set_app_status(
        &mut self,
        id: AppId,
        status: AppStatus,
    ) -> Result<StatusChange, CommandError> {
        if self.break_is_due() {
            return Err(CommandError::BreakDue);
        }
        self.apps.choose(id, status)
    }

    pub fn reset_app_statuses(&mut self) {
        self.apps.reset();
    }

    // Pendant une pause, une application est-elle à voiler ? Le Mode Hardcore couvre
    // l'écran entier, quel que soit le statut (§8.3).
    pub fn is_veiled(&self, id: Option<&AppId>) -> bool {
        id.is_none_or(|id| self.apps.effective_status(id) == AppStatus::Blocked)
    }

    // Relevé de la capacité, lu au premier instant de [PAUSE ACTIVE] et jamais après.
    pub fn observe_frames(&mut self, observable: bool) {
        self.frames_observable = observable;
    }

    // Trop de fenêtres bloquées pour les voiler une à une : la pause Simple en cours passe
    // en plein écran, une seule fois, et y reste jusqu'à [RETOUR].
    pub fn observe_blocked_windows(&mut self, count: usize) {
        let CycleState::BreakActive {
            deadline,
            severity: Severity::Simple,
            mode: BreakMode::Nominal,
        } = self.state
        else {
            return;
        };
        if count <= WINDOW_VEIL_CAP {
            return;
        }
        self.state = CycleState::BreakActive {
            deadline,
            severity: Severity::Simple,
            mode: BreakMode::Degraded(DegradedReason::TooManyWindows),
        };
    }
}
