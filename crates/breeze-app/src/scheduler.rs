use crate::enforcer::Enforcer;
use crate::snapshot::CycleSnapshot;
use breeze_domain::{CommandError, Countdown, Cycle, CycleState, Instant, Rhythm, Severity};
use breeze_ports::{DisplayEnumerationPort, OverlaySurfacesPort};

pub struct Scheduler {
    cycle: Cycle,
    enforcer: Enforcer,
}

impl Scheduler {
    pub fn new(cycle: Cycle) -> Self {
        Scheduler {
            cycle,
            enforcer: Enforcer::default(),
        }
    }

    pub fn poll<O, D>(&mut self, now: Instant, overlay: &mut O, displays: &D) -> CycleSnapshot
    where
        O: OverlaySurfacesPort,
        D: DisplayEnumerationPort,
    {
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
