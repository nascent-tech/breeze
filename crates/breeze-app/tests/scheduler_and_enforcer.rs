use breeze_app::{CyclePhase, Scheduler};
use breeze_domain::constants::NOTICE;
use breeze_domain::{ActiveDays, Cycle, Instant, Minutes, Rhythm, Severity};
use core::time::Duration;

const ONE_SEC: Duration = Duration::from_secs(1);
use breeze_ports::{
    Display, DisplayEnumerationPort, DisplayId, OverlayCapability, OverlaySurfacesPort, Rect,
    SurfaceId, SurfaceKind,
};

const SCREEN: Rect = Rect {
    x: 0,
    y: 0,
    width: 100,
    height: 100,
};

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
    let snap = sched.poll(Instant::EPOCH, &mut overlay, &displays);
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

    let snap = sched.poll(break_at, &mut overlay, &displays);
    assert_eq!(snap.phase, CyclePhase::Break);
    assert_eq!(overlay.covers, 2, "one surface per display");
    assert_eq!(overlay.last_kind, Some(SurfaceKind::Hardcore));
    assert_eq!(overlay.dismisses, 0);

    sched.poll(break_at.plus(ONE_SEC), &mut overlay, &displays);
    sched.poll(break_at.plus(pause - ONE_SEC), &mut overlay, &displays);
    assert_eq!(
        overlay.covers, 2,
        "polling again while the break holds poses no new surface"
    );
    assert_eq!(overlay.dismisses, 0);

    sched.poll(return_at, &mut overlay, &displays);
    assert_eq!(overlay.dismisses, 1, "overlays lift when the break ends");
}

#[test]
fn a_simple_break_veils_rather_than_shielding() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Simple, Instant::EPOCH));
    let mut overlay = SpyOverlay::default();
    let displays = TwoDisplays;

    sched.poll(break_at, &mut overlay, &displays);
    assert_eq!(overlay.last_kind, Some(SurfaceKind::Veil));
}

#[test]
fn next_wake_points_at_the_current_deadline() {
    let r = rhythm();
    let work = r.work().as_duration();
    let sched = Scheduler::new(Cycle::start(r, Severity::Simple, Instant::EPOCH));
    assert_eq!(sched.next_wake(), Some(Instant::EPOCH.plus(work)));
}

#[test]
fn the_null_bridge_runs_the_cycle_without_ever_covering() {
    use breeze_bridge_null::{NullDisplays, NullOverlay};
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut sched = Scheduler::new(Cycle::start(r, Severity::Hardcore, Instant::EPOCH));
    let mut overlay = NullOverlay;
    let displays = NullDisplays;
    let snap = sched.poll(break_at, &mut overlay, &displays);
    assert_eq!(snap.phase, CyclePhase::Break);
}
