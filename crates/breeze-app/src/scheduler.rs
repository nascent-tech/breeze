use crate::enforcer::Enforcer;
use crate::observation::Observation;
use crate::snapshot::CycleSnapshot;
use breeze_domain::{
    Absence, AbsenceVerdict, AppId, AppStatus, AppStatuses, BreakOutcome, CommandError, Cycle,
    Instant, InterruptionDoor, PostureDebt, Rhythm, Severity, StatusChange, Weekday,
};
use breeze_ports::{DisplayEnumerationPort, OverlaySurfacesPort, WindowFrame};

pub struct Scheduler {
    cycle: Cycle,
    enforcer: Enforcer,
    last_input: Instant,
    recorded_outcomes: usize,
}

impl Scheduler {
    pub fn new(cycle: Cycle) -> Self {
        Scheduler {
            cycle,
            enforcer: Enforcer::default(),
            last_input: Instant::EPOCH,
            recorded_outcomes: 0,
        }
    }

    // Le calendrier s'observe APRÈS l'avance du cycle : un décompte échu au moment où la
    // plage se ferme lance sa pause (préavis), il n'est pas avalé par [INACTIF]. La capacité
    // « cadres observables » est relevée AVANT : c'est elle que lit une pause qui commence.
    pub fn poll<O, D>(
        &mut self,
        now: Instant,
        in_hours: bool,
        overlay: &mut O,
        displays: &D,
        observation: &Observation,
    ) -> CycleSnapshot
    where
        O: OverlaySurfacesPort,
        D: DisplayEnumerationPort,
    {
        self.observe(now, observation);
        self.cycle.tick(now);
        self.cycle.observe_calendar(now, in_hours);
        let blocked = self.blocked_windows(observation);
        if let Some(blocked) = &blocked {
            self.cycle.observe_blocked_windows(blocked.len());
        }
        self.enforcer
            .reconcile(self.cycle.state(), blocked.as_deref(), overlay, displays);
        CycleSnapshot::of(&self.cycle)
    }

    fn observe(&mut self, now: Instant, observation: &Observation) {
        // `>` : la dernière saisie est monotone ; ne réagir qu'à une saisie plus récente
        // (une valeur qui recule ne doit jamais faire reculer l'horloge d'inactivité).
        let last_input = observation.signals.last_input;
        if last_input > self.last_input {
            self.last_input = last_input;
            self.cycle.observe_activity(last_input);
        }
        self.cycle
            .observe_foreground(now, observation.foreground.as_ref());
        self.cycle.freeze_if_idle(now);
        self.cycle.observe_frames(observation.frames_observable());
    }

    // Les fenêtres à voiler : celles des applications effectivement bloquées ce cycle-ci,
    // et celles dont le système ne donne pas l'identité (inconnues, donc bloquées).
    fn blocked_windows(&self, observation: &Observation) -> Option<Vec<WindowFrame>> {
        let windows = observation.windows.as_ref().ok()?;
        Some(
            windows
                .iter()
                .filter(|window| self.cycle.is_veiled(window.owner.as_ref()))
                .cloned()
                .collect(),
        )
    }

    // Relevé de capacité fait avant un réveil de veille : une pause qui démarre au réveil
    // (§10.4) lit ce relevé-là, pas celui d'avant la veille.
    pub fn observe_frames(&mut self, observable: bool) {
        self.cycle.observe_frames(observable);
    }

    pub fn app_statuses(&self) -> &AppStatuses {
        self.cycle.app_statuses()
    }

    pub fn set_app_status(
        &mut self,
        id: AppId,
        status: AppStatus,
    ) -> Result<StatusChange, CommandError> {
        self.cycle.set_app_status(id, status)
    }

    pub fn reset_app_statuses(&mut self) {
        self.cycle.reset_app_statuses();
    }

    // Les heures actives suivent le rythme CONFIGURÉ : une plage modifiée s'applique
    // sans attendre le cycle suivant.
    pub fn in_hours(&self, weekday: Weekday, minute_of_day: u16) -> bool {
        self.cycle
            .configured_rhythm()
            .is_active_at(weekday, minute_of_day)
    }

    pub fn return_from_sleep(
        &mut self,
        absence: Absence,
        now: Instant,
        asleep_at: (Weekday, u16),
    ) -> AbsenceVerdict {
        self.cycle.return_from_sleep(absence, now, asleep_at)
    }

    // La dette de posture s'efface à minuit, heure locale (§9.2).
    pub fn clear_debt(&mut self) {
        self.cycle.clear_debt();
    }

    // Sorts de pause apparus depuis le dernier appel, chacun rendu une seule fois.
    pub fn take_new_outcomes(&mut self) -> Vec<BreakOutcome> {
        let outcomes = self.cycle.outcomes();
        let fresh = outcomes
            .get(self.recorded_outcomes..)
            .map(<[BreakOutcome]>::to_vec)
            .unwrap_or_default();
        self.recorded_outcomes = outcomes.len();
        fresh
    }

    pub fn snapshot(&self) -> CycleSnapshot {
        CycleSnapshot::of(&self.cycle)
    }

    pub fn suspend(&mut self, now: Instant, resume_at: Instant) -> Result<(), CommandError> {
        self.cycle.suspend(now, resume_at)
    }

    pub fn resume(&mut self, now: Instant) -> Result<(), CommandError> {
        self.cycle.resume(now)
    }

    pub fn interrupt_break(&mut self, now: Instant) -> Result<(), CommandError> {
        self.cycle.interrupt_break(now)
    }

    // Terminer et prélever les sorts d'un seul geste : aucun autre fil ne peut prendre le
    // sort de la pause interrompue entre les deux et le perdre à la sortie.
    pub fn terminate_and_take(
        &mut self,
        now: Instant,
        door: InterruptionDoor,
    ) -> Vec<BreakOutcome> {
        self.cycle.terminate(now, door);
        self.take_new_outcomes()
    }

    pub fn debt(&self) -> PostureDebt {
        self.cycle.debt()
    }

    pub fn change_severity(&mut self, severity: Severity) -> Result<(), CommandError> {
        self.cycle.change_severity(severity)
    }

    pub fn chosen_severity(&self) -> Severity {
        self.cycle.chosen_severity()
    }

    pub fn change_rhythm(&mut self, rhythm: Rhythm) -> Result<(), CommandError> {
        self.cycle.change_rhythm(rhythm)
    }

    pub fn configured_rhythm(&self) -> Rhythm {
        self.cycle.configured_rhythm()
    }

    pub fn active_rhythm(&self) -> Rhythm {
        self.cycle.rhythm()
    }

    pub fn start_over(&mut self, now: Instant) -> Result<(), CommandError> {
        self.cycle.start_over(now)
    }

    pub fn break_is_due(&self) -> bool {
        self.cycle.break_is_due()
    }
}
