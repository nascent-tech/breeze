use crate::enforcer::Enforcer;
use crate::snapshot::CycleSnapshot;
use breeze_domain::{
    Absence, AbsenceVerdict, BreakOutcome, CommandError, Cycle, Instant, InterruptionDoor,
    PostureDebt, Rhythm, Severity, Weekday,
};
use breeze_ports::{DisplayEnumerationPort, OverlaySurfacesPort, SessionSignals};

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
    // plage se ferme lance sa pause (préavis), il n'est pas avalé par [INACTIF].
    pub fn poll<O, D>(
        &mut self,
        now: Instant,
        in_hours: bool,
        overlay: &mut O,
        displays: &D,
        signals: SessionSignals,
    ) -> CycleSnapshot
    where
        O: OverlaySurfacesPort,
        D: DisplayEnumerationPort,
    {
        // `>` : la dernière saisie est monotone ; ne réagir qu'à une saisie plus récente
        // (une valeur qui recule ne doit jamais faire reculer l'horloge d'inactivité).
        if signals.last_input > self.last_input {
            self.last_input = signals.last_input;
            self.cycle.observe_activity(signals.last_input);
        }
        self.cycle.freeze_if_idle(now);
        self.cycle.tick(now);
        self.cycle.observe_calendar(now, in_hours);
        self.enforcer
            .reconcile(self.cycle.state(), overlay, displays);
        CycleSnapshot::of(&self.cycle)
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
