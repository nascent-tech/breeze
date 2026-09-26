use breeze_app::{CyclePhase, Enforcer, Observation, Scheduler};
use breeze_domain::constants::NOTICE;
use breeze_domain::{
    ActiveDays, BreakMode, Cycle, CycleState, DegradedReason, Instant, InterruptionDoor, Minutes,
    Rhythm, Severity,
};
use breeze_ports::{
    Display, DisplayEnumerationPort, DisplayId, FramesUnobservable, OverlayCapability,
    OverlaySurfacesPort, PresentationLockPort, Rect, SessionSignals, SurfaceId, SurfaceKind,
    WindowId,
};
use core::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

const ONE_SEC: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Call {
    Cover(SurfaceKind),
    DismissAll,
    Lock,
    Hold,
    Release,
}

type Log = Rc<RefCell<Vec<Call>>>;

// Doubles manuels qui écrivent dans le même registre : l'ordre entre surfaces et verrou
// se lit tel qu'il est expédié.
struct RecordingOverlay(Log);

impl OverlaySurfacesPort for RecordingOverlay {
    fn cover_display(&mut self, _display: DisplayId, kind: SurfaceKind) -> SurfaceId {
        self.0.borrow_mut().push(Call::Cover(kind));
        SurfaceId(1)
    }

    fn cover_window(&mut self, _window: WindowId, _frame: Rect) -> SurfaceId {
        SurfaceId(2)
    }

    fn reframe(&mut self, _surface: SurfaceId, _frame: Rect) {}

    fn dismiss(&mut self, _surface: SurfaceId) {}

    fn dismiss_all(&mut self) {
        self.0.borrow_mut().push(Call::DismissAll);
    }

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::Layered
    }
}

struct RecordingLock(Log);

impl PresentationLockPort for RecordingLock {
    fn lock(&mut self) {
        self.0.borrow_mut().push(Call::Lock);
    }

    fn hold(&mut self) {
        self.0.borrow_mut().push(Call::Hold);
    }

    fn release(&mut self) {
        self.0.borrow_mut().push(Call::Release);
    }
}

struct OneDisplay;

impl DisplayEnumerationPort for OneDisplay {
    fn displays(&self) -> Vec<Display> {
        vec![Display {
            id: DisplayId(1),
            bounds: Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            },
        }]
    }
}

struct Harness {
    log: Log,
    overlay: RecordingOverlay,
    lock: RecordingLock,
}

impl Harness {
    fn new() -> Self {
        let log = Log::default();
        Harness {
            overlay: RecordingOverlay(log.clone()),
            lock: RecordingLock(log.clone()),
            log,
        }
    }

    fn reconcile(&mut self, enforcer: &mut Enforcer, state: CycleState) {
        enforcer.reconcile(
            state,
            Some(&[]),
            &mut self.overlay,
            &mut self.lock,
            &OneDisplay,
        );
    }

    fn poll(&mut self, scheduler: &mut Scheduler, now: Instant) -> CyclePhase {
        let observation = Observation {
            signals: SessionSignals { last_input: now },
            foreground: None,
            windows: Err(FramesUnobservable),
        };
        scheduler
            .poll(
                now,
                true,
                &mut self.overlay,
                &mut self.lock,
                &OneDisplay,
                &observation,
            )
            .phase
    }

    fn take(&self) -> Vec<Call> {
        core::mem::take(&mut *self.log.borrow_mut())
    }

    fn lock_calls(&self) -> Vec<Call> {
        self.take()
            .into_iter()
            .filter(|call| matches!(call, Call::Lock | Call::Hold | Call::Release))
            .collect()
    }
}

fn at(secs: u64) -> Instant {
    Instant::EPOCH.plus(Duration::from_secs(secs))
}

fn hardcore_break() -> CycleState {
    CycleState::BreakActive {
        deadline: at(600),
        severity: Severity::Hardcore,
        mode: BreakMode::Nominal,
    }
}

fn simple_break(mode: BreakMode) -> CycleState {
    CycleState::BreakActive {
        deadline: at(600),
        severity: Severity::Simple,
        mode,
    }
}

fn returning() -> CycleState {
    CycleState::Returning { deadline: at(603) }
}

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn break_at() -> Instant {
    Instant::EPOCH
        .plus(rhythm().work().as_duration())
        .plus(NOTICE)
}

fn scheduler(severity: Severity) -> Scheduler {
    Scheduler::new(Cycle::start(rhythm(), severity, Instant::EPOCH))
}

#[test]
fn a_hardcore_break_locks_the_presentation_once_its_surfaces_are_requested() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();

    harness.reconcile(&mut enforcer, hardcore_break());

    assert_eq!(
        harness.take(),
        vec![Call::Cover(SurfaceKind::Hardcore), Call::Lock]
    );
}

#[test]
fn every_poll_during_a_hardcore_break_holds_the_lock_without_locking_again() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();
    harness.reconcile(&mut enforcer, hardcore_break());
    harness.take();

    harness.reconcile(&mut enforcer, hardcore_break());
    harness.reconcile(&mut enforcer, hardcore_break());

    assert_eq!(harness.take(), vec![Call::Hold, Call::Hold]);
}

#[test]
fn the_return_releases_the_lock_once_after_the_surfaces_are_lifted() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();
    harness.reconcile(&mut enforcer, hardcore_break());
    harness.take();

    harness.reconcile(&mut enforcer, returning());
    harness.reconcile(&mut enforcer, returning());

    assert_eq!(harness.take(), vec![Call::DismissAll, Call::Release]);
}

#[test]
fn a_simple_break_never_locks_the_presentation() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();

    harness.reconcile(&mut enforcer, simple_break(BreakMode::Nominal));
    harness.reconcile(&mut enforcer, returning());
    harness.reconcile(
        &mut enforcer,
        simple_break(BreakMode::Degraded(DegradedReason::FramesUnobservable)),
    );
    harness.reconcile(&mut enforcer, returning());

    assert_eq!(harness.lock_calls(), vec![]);
}

#[test]
fn a_break_that_leaves_hardcore_coverage_releases_the_lock() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();
    harness.reconcile(&mut enforcer, hardcore_break());
    harness.take();

    harness.reconcile(
        &mut enforcer,
        simple_break(BreakMode::Degraded(DegradedReason::FramesUnobservable)),
    );
    harness.reconcile(
        &mut enforcer,
        simple_break(BreakMode::Degraded(DegradedReason::FramesUnobservable)),
    );

    assert_eq!(harness.lock_calls(), vec![Call::Release]);
}

#[test]
fn a_release_is_never_sent_without_a_lock_before_it() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();

    harness.reconcile(&mut enforcer, CycleState::Inactive);
    harness.reconcile(&mut enforcer, CycleState::Notice { deadline: at(60) });
    harness.reconcile(&mut enforcer, returning());

    assert_eq!(harness.lock_calls(), vec![]);
}

#[test]
fn each_hardcore_break_locks_and_releases_its_own_presentation() {
    let mut enforcer = Enforcer::default();
    let mut harness = Harness::new();

    harness.reconcile(&mut enforcer, hardcore_break());
    harness.reconcile(&mut enforcer, returning());
    harness.reconcile(&mut enforcer, hardcore_break());
    harness.reconcile(&mut enforcer, returning());

    assert_eq!(
        harness.lock_calls(),
        vec![Call::Lock, Call::Release, Call::Lock, Call::Release]
    );
}

#[test]
fn the_scheduler_releases_the_lock_when_the_break_is_served() {
    let mut sched = scheduler(Severity::Hardcore);
    let mut harness = Harness::new();
    let start = break_at();

    assert_eq!(harness.poll(&mut sched, start), CyclePhase::Break);
    harness.poll(&mut sched, start.plus(ONE_SEC));
    let end = start.plus(rhythm().pause().as_duration());
    assert_eq!(harness.poll(&mut sched, end), CyclePhase::Returning);
    harness.poll(&mut sched, end.plus(ONE_SEC));

    assert_eq!(
        harness.lock_calls(),
        vec![Call::Lock, Call::Hold, Call::Release]
    );
}

#[test]
fn the_escape_gesture_releases_the_lock_at_the_next_poll() {
    let mut sched = scheduler(Severity::Hardcore);
    let mut harness = Harness::new();
    let start = break_at();
    harness.poll(&mut sched, start);
    harness.take();

    let gesture = start.plus(ONE_SEC);
    sched.interrupt_break(gesture).unwrap();

    assert_eq!(harness.poll(&mut sched, gesture), CyclePhase::Working);
    harness.poll(&mut sched, gesture.plus(ONE_SEC));
    assert_eq!(harness.take(), vec![Call::DismissAll, Call::Release]);
}

#[test]
fn quitting_during_a_hardcore_break_releases_the_lock_if_a_poll_follows() {
    let mut sched = scheduler(Severity::Hardcore);
    let mut harness = Harness::new();
    let start = break_at();
    harness.poll(&mut sched, start);
    harness.take();

    let quit = start.plus(ONE_SEC);
    sched.terminate_and_take(quit, InterruptionDoor::Quit);
    harness.poll(&mut sched, quit);

    assert_eq!(harness.lock_calls(), vec![Call::Release]);
}

#[test]
fn a_simple_break_driven_by_the_scheduler_never_locks() {
    let mut sched = scheduler(Severity::Simple);
    let mut harness = Harness::new();
    let start = break_at();

    harness.poll(&mut sched, start);
    harness.poll(&mut sched, start.plus(ONE_SEC));

    assert_eq!(harness.lock_calls(), vec![]);
}
