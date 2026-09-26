use breeze_app::{CyclePhase, Scheduler};
use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{
    Absence, ActiveDays, BreakOutcome, Cycle, Instant, InterruptionDoor, Minutes, Rhythm, Severity,
    TimeRange, Weekday,
};
use breeze_ports::{
    Display, DisplayEnumerationPort, DisplayId, OverlayCapability, OverlaySurfacesPort, Rect,
    SessionSignals, SurfaceId, SurfaceKind,
};
use core::time::Duration;

const ONE_SEC: Duration = Duration::from_secs(1);

const SCREEN: Rect = Rect {
    x: 0,
    y: 0,
    width: 100,
    height: 100,
};

// L'utilisateur est actif à `now` : jamais de gel.
fn active(now: Instant) -> SessionSignals {
    SessionSignals { last_input: now }
}

// La dernière saisie remonte à `last`.
fn idle_since(last: Instant) -> SessionSignals {
    SessionSignals { last_input: last }
}

#[derive(Default)]
struct SpyOverlay {
    covers: u32,
    dismisses: u32,
    last_kind: Option<SurfaceKind>,
}

impl OverlaySurfacesPort for SpyOverlay {
    fn cover_display(&mut self, _display: DisplayId, kind: SurfaceKind) -> SurfaceId {
        self.covers += 1;
        self.last_kind = Some(kind);
        SurfaceId(u64::from(self.covers))
    }

    fn dismiss_all(&mut self) {
        self.dismisses += 1;
    }

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::Layered
    }
}

struct TwoDisplays;

impl DisplayEnumerationPort for TwoDisplays {
    fn displays(&self) -> Vec<Display> {
        vec![
            Display {
                id: DisplayId(1),
                bounds: SCREEN,
            },
            Display {
                id: DisplayId(2),
                bounds: SCREEN,
            },
        ]
    }
}

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

#[test]
fn the_scheduler_projects_the_working_phase_at_start_without_covering() {
    let mut sched = Scheduler::new(Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;
    let snap = sched.poll(
        Instant::EPOCH,
        true,
        &mut overlay,
        &displays,
        active(Instant::EPOCH),
    );
    assert_eq!(snap.phase, CyclePhase::Working);
    assert_eq!(overlay.covers, 0);
}

#[test]
fn overlays_cover_every_display_on_a_hardcore_break_then_lift_on_return() {
    let r = rhythm();
    let pause = r.pause().as_duration();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let return_at = break_at.plus(pause);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Hardcore, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    let snap = sched.poll(break_at, true, &mut overlay, &displays, active(break_at));
    assert_eq!(snap.phase, CyclePhase::Break);
    assert_eq!(overlay.covers, 2, "one surface per display");
    assert_eq!(overlay.last_kind, Some(SurfaceKind::Hardcore));
    assert_eq!(overlay.dismisses, 0);

    let mid = break_at.plus(ONE_SEC);
    sched.poll(mid, true, &mut overlay, &displays, active(mid));
    let late = break_at.plus(pause - ONE_SEC);
    sched.poll(late, true, &mut overlay, &displays, active(late));
    assert_eq!(
        overlay.covers, 2,
        "polling again while the break holds poses no new surface"
    );
    assert_eq!(overlay.dismisses, 0);

    sched.poll(return_at, true, &mut overlay, &displays, active(return_at));
    assert_eq!(overlay.dismisses, 1, "overlays lift when the break ends");
}

#[test]
fn a_simple_break_veils_rather_than_shielding() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    sched.poll(break_at, true, &mut overlay, &displays, active(break_at));
    assert_eq!(overlay.last_kind, Some(SurfaceKind::Veil));
}

#[test]
fn a_working_countdown_freezes_once_the_session_signals_report_enough_idle() {
    use breeze_domain::constants::IDLE_FREEZE;

    let mut sched = Scheduler::new(Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;
    // Dernière saisie à l'EPOCH ; le poll a lieu après le seuil, sans activité entre-temps.
    let snap = sched.poll(
        Instant::EPOCH.plus(IDLE_FREEZE),
        true,
        &mut overlay,
        &displays,
        idle_since(Instant::EPOCH),
    );

    // Un décompte gelé n'a plus d'échéance mais garde son restant pour l'UI.
    let work = rhythm().work().as_duration();
    assert_eq!(snap.phase, CyclePhase::Working);
    assert_eq!(snap.deadline, None);
    assert_eq!(snap.frozen_remaining, Some(work - IDLE_FREEZE));
    assert_eq!(overlay.covers, 0);
}

#[test]
fn fresh_input_thaws_a_frozen_countdown_at_the_next_poll() {
    use breeze_domain::constants::IDLE_FREEZE;

    let mut sched = Scheduler::new(Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    let frozen = sched.poll(
        Instant::EPOCH.plus(IDLE_FREEZE),
        true,
        &mut overlay,
        &displays,
        idle_since(Instant::EPOCH),
    );
    assert_eq!(frozen.deadline, None);

    // Une saisie fraîche (last_input avancé) : le décompte reprend une échéance.
    let woke_at = Instant::EPOCH.plus(IDLE_FREEZE);
    let running = sched.poll(woke_at, true, &mut overlay, &displays, active(woke_at));
    assert_eq!(running.phase, CyclePhase::Working);
    assert!(running.deadline.is_some());
}

#[test]
fn the_null_bridge_runs_the_cycle_without_ever_covering() {
    use breeze_bridge_null::{NullDisplays, NullOverlay};
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Hardcore, Instant::EPOCH));
    let mut overlay = NullOverlay;
    let displays = NullDisplays;
    let snap = sched.poll(break_at, true, &mut overlay, &displays, active(break_at));
    assert_eq!(snap.phase, CyclePhase::Break);
}

fn office_hours() -> Rhythm {
    Rhythm::new(
        Minutes(50),
        Minutes(10),
        Some(TimeRange::from_minutes(9 * 60, 18 * 60).unwrap()),
        ActiveDays::from_mask(0b0001_1111).unwrap(),
    )
    .unwrap()
}

#[test]
fn the_calendar_puts_the_cycle_to_rest_outside_the_configured_hours() {
    let mut sched = Scheduler::new(Cycle::start(
        office_hours(),
        Severity::Simple,
        Instant::EPOCH,
    ));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    let saturday = sched.in_hours(Weekday::Saturday, 10 * 60);
    let snap = sched.poll(
        Instant::EPOCH,
        saturday,
        &mut overlay,
        &displays,
        active(Instant::EPOCH),
    );
    assert_eq!(snap.phase, CyclePhase::Inactive);

    let monday = Instant::EPOCH.plus(ONE_SEC);
    let in_hours = sched.in_hours(Weekday::Monday, 9 * 60);
    let snap = sched.poll(monday, in_hours, &mut overlay, &displays, active(monday));
    assert_eq!(snap.phase, CyclePhase::Working);
}

#[test]
fn a_countdown_that_ends_as_the_schedule_closes_still_starts_its_break() {
    let r = office_hours();
    let due = Instant::EPOCH.plus(r.work().as_duration());
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    let closed = sched.in_hours(Weekday::Tuesday, 18 * 60);
    let snap = sched.poll(due, closed, &mut overlay, &displays, active(due));

    assert!(!closed);
    assert_eq!(snap.phase, CyclePhase::Notice);
}

#[test]
fn waking_the_next_morning_starts_a_full_work_cycle() {
    let r = office_hours();
    let work = r.work().as_duration();
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;
    // Capot fermé mardi 17:50, rouvert mercredi 9:10 ; l'horloge monotone n'a pas bougé.
    let slept_at = Instant::EPOCH.plus(Duration::from_secs(2400));
    let wake = slept_at.plus(ONE_SEC);
    let absence = Absence {
        began_at: slept_at,
        lasted: Duration::from_secs(15 * 3600 + 20 * 60),
    };

    sched.return_from_sleep(absence, wake, (Weekday::Tuesday, 17 * 60 + 50));
    let in_hours = sched.in_hours(Weekday::Wednesday, 9 * 60 + 10);
    let snap = sched.poll(wake, in_hours, &mut overlay, &displays, active(wake));

    assert_eq!(snap.phase, CyclePhase::Working);
    assert_eq!(snap.deadline, Some(wake.plus(work)));
}

#[test]
fn each_break_outcome_is_handed_out_exactly_once() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let back_at = break_at.plus(r.pause().as_duration()).plus(RETURN_HOLD);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Hardcore, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    assert!(sched.take_new_outcomes().is_empty());
    sched.poll(back_at, true, &mut overlay, &displays, active(back_at));
    assert_eq!(
        sched.take_new_outcomes(),
        vec![BreakOutcome::Served {
            planned: r.pause().as_duration()
        }]
    );
    assert!(sched.take_new_outcomes().is_empty());

    let next_break = back_at.plus(r.work().as_duration()).plus(NOTICE);
    sched.poll(
        next_break,
        true,
        &mut overlay,
        &displays,
        active(next_break),
    );
    sched.interrupt_break(next_break).unwrap();
    assert_eq!(
        sched.terminate_and_take(next_break, InterruptionDoor::Quit),
        vec![BreakOutcome::Interrupted {
            planned: r.pause().as_duration(),
            unserved: r.pause().as_duration(),
            door: InterruptionDoor::HardcoreExitGesture,
        }]
    );
}

#[test]
fn quitting_during_a_break_hands_out_its_interruption_at_once() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    sched.poll(break_at, true, &mut overlay, &TwoDisplays, active(break_at));

    let taken = sched.terminate_and_take(break_at, InterruptionDoor::Quit);

    assert_eq!(taken.len(), 1);
    assert!(sched.take_new_outcomes().is_empty());
}
