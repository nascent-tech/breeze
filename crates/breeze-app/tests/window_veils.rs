use breeze_app::{CyclePhase, Observation, Scheduler, VeilMode};
use breeze_domain::constants::{NOTICE, WINDOW_VEIL_CAP};
use breeze_domain::{
    ActiveDays, AppId, AppStatus, AppStatuses, Cycle, FreezeReason, Instant, Minutes, Rhythm,
    SafetyList, Severity, SparedApps,
};
use breeze_ports::{
    Display, DisplayEnumerationPort, DisplayId, FramesUnobservable, OverlayCapability,
    OverlaySurfacesPort, Rect, SessionSignals, SurfaceId, SurfaceKind, WindowFrame, WindowId,
};
use core::time::Duration;

const ONE_SEC: Duration = Duration::from_secs(1);
const SIDE: u32 = 400;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Call {
    CoverDisplay(DisplayId, SurfaceKind),
    CoverWindow(WindowId, Rect),
    Reframe(SurfaceId, Rect),
    Dismiss(SurfaceId),
    DismissAll,
}

// Double manuel : chaque appel est noté, chaque surface reçoit un numéro neuf.
#[derive(Default)]
struct RecordingOverlay {
    calls: Vec<Call>,
    next: u64,
}

impl RecordingOverlay {
    fn issue(&mut self) -> SurfaceId {
        self.next += 1;
        SurfaceId(self.next)
    }

    fn take(&mut self) -> Vec<Call> {
        core::mem::take(&mut self.calls)
    }
}

impl OverlaySurfacesPort for RecordingOverlay {
    fn cover_display(&mut self, display: DisplayId, kind: SurfaceKind) -> SurfaceId {
        self.calls.push(Call::CoverDisplay(display, kind));
        self.issue()
    }

    fn cover_window(&mut self, window: WindowId, frame: Rect) -> SurfaceId {
        self.calls.push(Call::CoverWindow(window, frame));
        self.issue()
    }

    fn reframe(&mut self, surface: SurfaceId, frame: Rect) {
        self.calls.push(Call::Reframe(surface, frame));
    }

    fn dismiss(&mut self, surface: SurfaceId) {
        self.calls.push(Call::Dismiss(surface));
    }

    fn dismiss_all(&mut self) {
        self.calls.push(Call::DismissAll);
    }

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::Layered
    }
}

struct OneDisplay;

impl DisplayEnumerationPort for OneDisplay {
    fn displays(&self) -> Vec<Display> {
        vec![Display {
            id: DisplayId(1),
            bounds: rect(0),
        }]
    }
}

fn id(raw: &str) -> AppId {
    AppId::parse(raw).unwrap()
}

fn rect(x: i32) -> Rect {
    Rect {
        x,
        y: 0,
        width: SIDE,
        height: SIDE,
    }
}

fn window(number: u32, owner: &str, x: i32) -> WindowFrame {
    WindowFrame {
        id: WindowId(number),
        owner: Some(id(owner)),
        frame: rect(x),
    }
}

fn seen(now: Instant, windows: Vec<WindowFrame>) -> Observation {
    Observation {
        signals: SessionSignals { last_input: now },
        foreground: None,
        windows: Ok(windows),
    }
}

fn unobservable(now: Instant) -> Observation {
    Observation {
        signals: SessionSignals { last_input: now },
        foreground: None,
        windows: Err(FramesUnobservable),
    }
}

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn break_at() -> Instant {
    Instant::EPOCH
        .plus(rhythm().work().as_duration())
        .plus(NOTICE)
}

// Music est épargnée, Terminal est sur la liste de sécurité, tout le reste est bloqué.
fn scheduler(severity: Severity) -> Scheduler {
    let chosen = SparedApps::from_pairs([(id("com.apple.Music"), AppStatus::Spared)]);
    let apps = AppStatuses::new(chosen, SafetyList::new([id("com.apple.Terminal")]));
    Scheduler::new(Cycle::start(rhythm(), severity, Instant::EPOCH).with_app_statuses(apps))
}

#[test]
fn a_simple_break_veils_only_the_windows_of_blocked_apps() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let windows = vec![
        window(1, "com.apple.Safari", 0),
        window(2, "com.apple.Music", 500),
        window(3, "com.apple.Terminal", 900),
    ];

    let snap = sched.poll(
        break_at(),
        true,
        &mut overlay,
        &OneDisplay,
        &seen(break_at(), windows),
    );

    assert_eq!(snap.veil_mode, Some(VeilMode::Windows));
    assert_eq!(
        overlay.take(),
        vec![Call::CoverWindow(WindowId(1), rect(0))]
    );
}

#[test]
fn a_window_without_identity_is_veiled_as_unknown() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let anonymous = WindowFrame {
        id: WindowId(7),
        owner: None,
        frame: rect(0),
    };

    sched.poll(
        break_at(),
        true,
        &mut overlay,
        &OneDisplay,
        &seen(break_at(), vec![anonymous]),
    );

    assert_eq!(
        overlay.take(),
        vec![Call::CoverWindow(WindowId(7), rect(0))]
    );
}

#[test]
fn a_veil_follows_its_window_and_reuses_its_surface() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let at = break_at();
    sched.poll(
        at,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(at, vec![window(1, "a.b.c", 0)]),
    );
    overlay.take();

    let still = at.plus(ONE_SEC);
    sched.poll(
        still,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(still, vec![window(1, "a.b.c", 0)]),
    );
    assert!(overlay.take().is_empty(), "an unmoved window costs nothing");

    let moved = still.plus(ONE_SEC);
    sched.poll(
        moved,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(moved, vec![window(1, "a.b.c", 80)]),
    );
    assert_eq!(overlay.take(), vec![Call::Reframe(SurfaceId(1), rect(80))]);
}

#[test]
fn a_new_window_gets_a_veil_and_a_closed_one_loses_it() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let at = break_at();
    sched.poll(
        at,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(at, vec![window(1, "a.b.c", 0)]),
    );
    overlay.take();

    let later = at.plus(ONE_SEC);
    sched.poll(
        later,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(later, vec![window(2, "a.b.c", 600)]),
    );

    assert_eq!(
        overlay.take(),
        vec![
            Call::Dismiss(SurfaceId(1)),
            Call::CoverWindow(WindowId(2), rect(600))
        ]
    );
}

#[test]
fn veils_stay_put_while_the_frames_cannot_be_read() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let at = break_at();
    sched.poll(
        at,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(at, vec![window(1, "a.b.c", 0)]),
    );
    overlay.take();

    let later = at.plus(ONE_SEC);
    let snap = sched.poll(later, true, &mut overlay, &OneDisplay, &unobservable(later));

    assert!(overlay.take().is_empty());
    assert_eq!(
        snap.veil_mode,
        Some(VeilMode::Windows),
        "the mode never changes mid-break"
    );
}

#[test]
fn unobservable_frames_at_the_first_instant_give_a_full_screen_veil() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();

    let snap = sched.poll(
        break_at(),
        true,
        &mut overlay,
        &OneDisplay,
        &unobservable(break_at()),
    );

    assert_eq!(snap.veil_mode, Some(VeilMode::FullScreen));
    assert_eq!(
        overlay.take(),
        vec![Call::CoverDisplay(DisplayId(1), SurfaceKind::Veil)]
    );
}

#[test]
fn a_hardcore_break_shields_every_screen_whatever_the_statuses() {
    let mut sched = scheduler(Severity::Hardcore);
    let mut overlay = RecordingOverlay::default();
    let windows = vec![window(1, "com.apple.Safari", 0)];

    let snap = sched.poll(
        break_at(),
        true,
        &mut overlay,
        &OneDisplay,
        &seen(break_at(), windows),
    );

    assert_eq!(snap.veil_mode, None);
    assert_eq!(
        overlay.take(),
        vec![Call::CoverDisplay(DisplayId(1), SurfaceKind::Hardcore)]
    );
}

#[test]
fn beyond_the_cap_the_window_veils_give_way_to_a_full_screen_veil_once() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let at = break_at();
    sched.poll(
        at,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(at, vec![window(1, "a.b.c", 0)]),
    );
    overlay.take();

    let crowd: Vec<WindowFrame> = (1..=u32::try_from(WINDOW_VEIL_CAP).unwrap() + 1)
        .map(|n| window(n, "a.b.c", 0))
        .collect();
    let later = at.plus(ONE_SEC);
    let snap = sched.poll(later, true, &mut overlay, &OneDisplay, &seen(later, crowd));

    assert_eq!(snap.veil_mode, Some(VeilMode::FullScreen));
    assert_eq!(
        overlay.take(),
        vec![
            Call::Dismiss(SurfaceId(1)),
            Call::CoverDisplay(DisplayId(1), SurfaceKind::Veil)
        ]
    );

    let calm = later.plus(ONE_SEC);
    sched.poll(
        calm,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(calm, vec![window(1, "a.b.c", 0)]),
    );
    assert!(
        overlay.take().is_empty(),
        "no return to window veils during this break"
    );
}

#[test]
fn every_surface_lifts_when_the_break_ends() {
    let mut sched = scheduler(Severity::Simple);
    let mut overlay = RecordingOverlay::default();
    let at = break_at();
    sched.poll(
        at,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(at, vec![window(1, "a.b.c", 0)]),
    );
    overlay.take();

    let back = at.plus(rhythm().pause().as_duration());
    let snap = sched.poll(
        back,
        true,
        &mut overlay,
        &OneDisplay,
        &seen(back, vec![window(1, "a.b.c", 0)]),
    );

    assert_eq!(snap.phase, CyclePhase::Returning);
    assert_eq!(snap.veil_mode, None);
    assert_eq!(overlay.take(), vec![Call::DismissAll]);
}

#[test]
fn an_ignored_app_in_front_freezes_the_work_and_says_why() {
    let chosen = SparedApps::from_pairs([(id("com.apple.Music"), AppStatus::Ignored)]);
    let apps = AppStatuses::new(chosen, SafetyList::default());
    let mut sched = Scheduler::new(
        Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH).with_app_statuses(apps),
    );
    let mut overlay = RecordingOverlay::default();
    let at = Instant::EPOCH.plus(ONE_SEC);
    let observation = Observation {
        foreground: Some(id("com.apple.Music")),
        ..seen(at, Vec::new())
    };

    let snap = sched.poll(at, true, &mut overlay, &OneDisplay, &observation);

    assert_eq!(snap.frozen_reason, Some(FreezeReason::IgnoredApp));
    assert_eq!(snap.deadline, None);
}
