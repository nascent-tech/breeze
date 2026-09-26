use super::Cycle;
use crate::clock::Instant;
use crate::constants::IDLE_FREEZE;
use crate::cycle::countdown::Countdown;
use crate::cycle::freeze_reason::FreezeReason;
use crate::cycle::state::CycleState;
use crate::settings::{AppId, AppStatus};

// Deux causes gèlent un décompte de travail qui n'est pas encore échu : l'inactivité
// (§10.1) et une application Ignorée au premier plan (§8.3). Il ne repart que quand
// aucune des deux ne tient plus.
impl Cycle {
    pub fn observe_activity(&mut self, at: Instant) {
        self.last_activity = at;
        self.idle = false;
        self.thaw_if_free(at);
    }

    pub fn freeze_if_idle(&mut self, now: Instant) {
        let freeze_at = self.last_activity.plus(IDLE_FREEZE);
        if now < freeze_at {
            return;
        }
        if self.freeze_running(now, freeze_at) || self.is_frozen() {
            self.idle = true;
        }
    }

    // Relevé de l'application au premier plan, à chaque poll. `None` = identité inconnue :
    // tout usage compte alors comme du travail (§10.5).
    pub fn observe_foreground(&mut self, now: Instant, app: Option<&AppId>) {
        self.ignored_in_front =
            app.is_some_and(|id| self.apps.effective_status(id) == AppStatus::Ignored);
        if self.ignored_in_front {
            self.freeze_running(now, now);
        } else {
            self.thaw_if_free(now);
        }
    }

    pub fn freeze_reason(&self) -> Option<FreezeReason> {
        if !self.is_frozen() {
            return None;
        }
        if self.ignored_in_front {
            return Some(FreezeReason::IgnoredApp);
        }
        Some(FreezeReason::Idle)
    }

    // Gèle un décompte qui court et n'est pas échu ; le restant se compte depuis `from`.
    fn freeze_running(&mut self, now: Instant, from: Instant) -> bool {
        let CycleState::Working {
            countdown: Countdown::Running { deadline },
        } = self.state
        else {
            return false;
        };
        if now.has_reached(deadline) {
            return false;
        }
        self.state = CycleState::Working {
            countdown: Countdown::Frozen {
                remaining: deadline.elapsed_since(from),
            },
        };
        true
    }

    fn is_frozen(&self) -> bool {
        matches!(
            self.state,
            CycleState::Working {
                countdown: Countdown::Frozen { .. }
            }
        )
    }

    fn thaw_if_free(&mut self, at: Instant) {
        if self.idle || self.ignored_in_front {
            return;
        }
        let CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } = self.state
        else {
            return;
        };
        let Some(deadline) = at.checked_plus(remaining) else {
            return;
        };
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
    }

    // Un décompte neuf (travail suivant, reprise) repart de l'instant présent : l'horloge
    // d'inactivité avec lui ; l'app au premier plan sera relue au poll suivant.
    pub(super) fn reset_freezing(&mut self, from: Instant) {
        self.last_activity = from;
        self.idle = false;
    }
}
