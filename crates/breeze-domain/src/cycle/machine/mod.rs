mod app_statuses;
mod commands;
mod freezing;
mod transitions;
mod wake;

use crate::clock::Instant;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::debt::PostureDebt;
use crate::outcome::BreakOutcome;
use crate::settings::{AppStatuses, Rhythm, Severity};

#[derive(Clone, Debug)]
pub struct Cycle {
    state: CycleState,
    rhythm: Rhythm,
    pending_rhythm: Option<Rhythm>,
    severity: Severity,
    pending_severity: Option<Severity>,
    last_activity: Instant,
    outcomes: Vec<BreakOutcome>,
    debt: PostureDebt,
    apps: AppStatuses,
    // Relevé de la capacité « cadres observables », tenu pour le premier instant d'une pause.
    frames_observable: bool,
    // Le gel d'inactivité tient tant qu'aucune saisie fraîche n'arrive.
    idle: bool,
    // Une application Ignorée (effective) est au premier plan.
    ignored_in_front: bool,
}

impl Cycle {
    pub fn start(rhythm: Rhythm, severity: Severity, now: Instant) -> Self {
        let deadline = now.plus(rhythm.work().as_duration());
        Cycle {
            state: CycleState::Working {
                countdown: Countdown::Running { deadline },
            },
            rhythm,
            pending_rhythm: None,
            severity,
            pending_severity: None,
            last_activity: now,
            outcomes: Vec::new(),
            debt: PostureDebt::none(),
            apps: AppStatuses::default(),
            frames_observable: false,
            idle: false,
            ignored_in_front: false,
        }
    }

    pub fn with_debt(mut self, debt: PostureDebt) -> Self {
        self.debt = debt;
        self
    }

    pub fn debt(&self) -> PostureDebt {
        self.debt
    }

    pub fn clear_debt(&mut self) {
        self.debt.clear();
    }

    pub fn state(&self) -> CycleState {
        self.state
    }

    pub fn severity(&self) -> Severity {
        self.severity
    }

    pub fn chosen_severity(&self) -> Severity {
        self.pending_severity.unwrap_or(self.severity)
    }

    pub fn configured_rhythm(&self) -> Rhythm {
        self.pending_rhythm.unwrap_or(self.rhythm)
    }

    pub fn rhythm(&self) -> Rhythm {
        self.rhythm
    }

    pub fn rhythm_pending(&self) -> bool {
        self.pending_rhythm.is_some()
    }

    // Hors des heures actives, seul le travail s'éteint : une pause entamée (préavis,
    // pause, retour) va à son terme, une suspension reste une suspension.
    pub fn observe_calendar(&mut self, now: Instant, in_hours: bool) {
        match (self.state, in_hours) {
            (CycleState::Working { .. }, false) => self.state = CycleState::Inactive,
            (CycleState::Inactive, true) => _ = self.enter_next_work(now),
            _ => {}
        }
    }

    pub fn outcomes(&self) -> &[BreakOutcome] {
        &self.outcomes
    }
}
