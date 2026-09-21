use crate::enforcer::Enforcer;
use crate::snapshot::CycleSnapshot;
use breeze_domain::{
    CommandError, Countdown, Cycle, CycleState, Instant, InterruptionDoor, PostureDebt, Rhythm,
    Severity,
};
use breeze_ports::{DisplayEnumerationPort, OverlaySurfacesPort, SessionSignals};

pub struct Scheduler {
    cycle: Cycle,
    enforcer: Enforcer,
    last_input: Instant,
}

impl Scheduler {
    pub fn new(cycle: Cycle) -> Self {
        Scheduler {
            cycle,
            enforcer: Enforcer::default(),
            last_input: Instant::EPOCH,
        }
    }

    pub fn poll<O, D>(
        &mut self,
        now: Instant,
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
        self.enforcer
            .reconcile(self.cycle.state(), overlay, displays);
        CycleSnapshot::of(&self.cycle)
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

    pub fn terminate(&mut self, now: Instant, door: InterruptionDoor) {
        self.cycle.terminate(now, door);
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

    pub fn next_wake(&self) -> Option<Instant> {
        deadline_of(self.cycle.state())
    }
}

fn deadline_of(state: CycleState) -> Option<Instant> {
    match state {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        }
        | CycleState::Notice { deadline }
        | CycleState::BreakActive { deadline, .. }
        | CycleState::Returning { deadline }
        | CycleState::Suspended {
            resume_at: deadline,
            ..
        } => Some(deadline),
        _ => None,
    }
}
