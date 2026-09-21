use crate::enforcer::Enforcer;
use crate::snapshot::CycleSnapshot;
use breeze_domain::{Countdown, Cycle, CycleState, Instant};
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
        | CycleState::Returning { deadline } => Some(deadline),
        _ => None,
    }
}
