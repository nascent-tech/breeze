use crate::dto::remaining_of;
use crate::local_time::{local_now, LocalNow};
use crate::{bridge, lock, AppState, TRAY_ID};
use breeze_app::{CyclePhase, CycleSnapshot, Observation, Scheduler};
use breeze_domain::constants::SECONDS_PER_MINUTE;
use breeze_domain::{Absence, Instant, WallClock};
use breeze_ports::{ForegroundAppPort, SessionSignalsPort, WindowFramesPort};
use core::time::Duration;
use std::sync::atomic::Ordering;
use std::thread;
use tauri::{AppHandle, Manager};

const TICK: Duration = Duration::from_millis(250);
// L'horloge monotone de macOS ne compte pas la veille : quand le temps mural avance de
// plus que cela au-delà du monotone entre deux ticks, la machine a dormi.
const SLEEP_THRESHOLD: Duration = Duration::from_secs(60);

#[cfg(target_os = "macos")]
fn session_signals() -> Box<dyn SessionSignalsPort> {
    Box::new(breeze_bridge_macos::MacSessionSignals::new())
}

#[cfg(not(target_os = "macos"))]
fn session_signals() -> Box<dyn SessionSignalsPort> {
    Box::new(breeze_bridge_null::NullSessionSignals)
}

// Premier plan et cadres des fenêtres : lus à chaque tick, jamais sous le verrou.
#[cfg(target_os = "macos")]
fn desktop() -> (Box<dyn ForegroundAppPort>, Box<dyn WindowFramesPort>) {
    (
        Box::new(breeze_bridge_macos::MacForegroundApp::new()),
        Box::new(breeze_bridge_macos::MacWindowFrames::new()),
    )
}

// Sans adaptateur : identité inconnue (tout compte comme travail), cadres inobservables
// (pause Simple en voile plein écran), §10.5.
#[cfg(not(target_os = "macos"))]
fn desktop() -> (Box<dyn ForegroundAppPort>, Box<dyn WindowFramesPort>) {
    (
        Box::new(breeze_bridge_null::NullForegroundApp),
        Box::new(breeze_bridge_null::NullWindowFrames),
    )
}

// Titre à côté de l'icône : le décompte des phases qui en ont un, rien sinon (ni
// « 0:00 » hors des heures actives, ni pendant une suspension).
pub fn tray_title(snapshot: &CycleSnapshot, now: Instant, show_countdown: bool) -> String {
    let counts_down = matches!(
        snapshot.phase,
        CyclePhase::Working | CyclePhase::Notice | CyclePhase::Break
    );
    if !show_countdown || !counts_down {
        return String::new();
    }
    let secs = remaining_of(snapshot, now).as_secs();
    format!(
        "{}:{:02}",
        secs / SECONDS_PER_MINUTE,
        secs % SECONDS_PER_MINUTE
    )
}

fn set_tray_title(app: &AppHandle, title: &str) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return;
    };
    if let Err(error) = tray.set_title(Some(title)) {
        eprintln!("breeze: could not update the tray title: {error}");
    }
}

// Carillon discret aux bornes de la pause. Non bloquant ; l'échec est journalisé.
#[cfg(target_os = "macos")]
fn chime_for_transition(previous: CyclePhase, current: CyclePhase) {
    let sound = if current == CyclePhase::Break {
        "/System/Library/Sounds/Glass.aiff"
    } else if previous == CyclePhase::Break {
        "/System/Library/Sounds/Ping.aiff"
    } else {
        return;
    };
    match std::process::Command::new("/usr/bin/afplay")
        .arg(sound)
        .spawn()
    {
        // Attendu hors du ticker : le processus fini est récolté, jamais laissé zombie.
        Ok(mut child) => {
            thread::spawn(move || {
                if let Err(error) = child.wait() {
                    eprintln!("breeze: could not reap the chime: {error}");
                }
            });
        }
        Err(error) => eprintln!("breeze: could not play the chime: {error}"),
    }
}

#[cfg(not(target_os = "macos"))]
fn chime_for_transition(_previous: CyclePhase, _current: CyclePhase) {}

// Relevé d'un tick : les deux horloges et le calendrier local, lus ensemble.
#[derive(Clone, Copy)]
struct TickMark {
    wall: WallClock,
    monotonic: Instant,
    local: LocalNow,
}

// Durée murale d'une veille entre deux relevés, s'il y en a eu une.
fn slept_between(
    previous_wall: WallClock,
    previous: Instant,
    wall: WallClock,
    now: Instant,
) -> Option<Duration> {
    let wall_elapsed = wall.saturating_duration_since(previous_wall);
    let awake_elapsed = now.elapsed_since(previous);
    (wall_elapsed > awake_elapsed.saturating_add(SLEEP_THRESHOLD)).then_some(wall_elapsed)
}

// Ce qui s'est passé depuis le relevé précédent : minuit franchi (la dette de posture
// s'efface, §9.2) et veille (verdict d'absence, puis heures actives quittées ou non).
fn catch_up(scheduler: &mut Scheduler, previous: TickMark, current: TickMark) {
    if previous.local.date != current.local.date {
        scheduler.clear_debt();
    }
    let Some(lasted) = slept_between(
        previous.wall,
        previous.monotonic,
        current.wall,
        current.monotonic,
    ) else {
        return;
    };
    let absence = Absence {
        began_at: previous.monotonic,
        lasted,
    };
    let asleep_at = (previous.local.weekday, previous.local.minute_of_day);
    scheduler.return_from_sleep(absence, current.monotonic, asleep_at);
}

struct Ticker {
    app: AppHandle,
    monitors: bridge::MonitorCache,
    overlay: bridge::TauriOverlay,
    displays: bridge::TauriDisplays,
    signals: Box<dyn SessionSignalsPort>,
    foreground: Box<dyn ForegroundAppPort>,
    frames: Box<dyn WindowFramesPort>,
    last_phase: CyclePhase,
    last_title: Option<String>,
    last_mark: Option<TickMark>,
}

impl Ticker {
    fn new(app: AppHandle) -> Self {
        let monitors = bridge::MonitorCache::default();
        let (foreground, frames) = desktop();
        Ticker {
            overlay: bridge::TauriOverlay::new(app.clone()),
            displays: bridge::TauriDisplays::new(monitors.clone()),
            monitors,
            signals: session_signals(),
            foreground,
            frames,
            last_phase: CyclePhase::Inactive,
            last_title: None,
            last_mark: None,
            app,
        }
    }

    // Invariant : aucun appel Tauri bloquant (getter, set_title) tant que le scheduler
    // est verrouillé — une commande synchrone l'attend peut-être sur le thread principal.
    fn advance_cycle(&mut self, state: &AppState, now: Instant) -> CycleSnapshot {
        let mark = TickMark {
            wall: state.clock.wall(),
            monotonic: now,
            local: local_now(),
        };
        let observation = self.observe(now);
        let previous = self.last_mark.replace(mark);
        let mut scheduler = lock(&state.scheduler);
        // Une pause qui démarre au réveil (§10.4) lit la capacité de cet instant-ci.
        scheduler.observe_frames(observation.frames_observable());
        if let Some(previous) = previous {
            catch_up(&mut scheduler, previous, mark);
        }
        let in_hours = scheduler.in_hours(mark.local.weekday, mark.local.minute_of_day);
        scheduler.poll(
            now,
            in_hours,
            &mut self.overlay,
            &self.displays,
            &observation,
        )
    }

    // Relevé du système, fait avant de prendre le verrou du scheduler.
    fn observe(&mut self, now: Instant) -> Observation {
        Observation {
            signals: self.signals.poll(now),
            foreground: self.foreground.foreground_app(),
            windows: self.frames.visible_windows(),
        }
    }

    fn refresh_title(&mut self, title: String) {
        if self.last_title.as_deref() == Some(title.as_str()) {
            return;
        }
        set_tray_title(&self.app, &title);
        self.last_title = Some(title);
    }

    fn tick(&mut self) {
        let app = self.app.clone();
        let Some(state) = app.try_state::<AppState>() else {
            return;
        };
        self.monitors.refresh_from_main_thread(&app);
        let now = state.clock.monotonic();
        let snapshot = self.advance_cycle(&state, now);
        self.overlay.keep_window_veils_above_targets();
        if state.record_outcomes() > 0 {
            // L'échec est déjà journalisé par save_state ; le tick suivant retentera.
            _ = state.save_state();
        }
        let show_countdown = state.menubar_text.load(Ordering::Relaxed);
        self.refresh_title(tray_title(&snapshot, now, show_countdown));
        let previous = core::mem::replace(&mut self.last_phase, snapshot.phase);
        if previous != snapshot.phase && state.sounds.load(Ordering::Relaxed) {
            chime_for_transition(previous, snapshot.phase);
        }
    }
}

pub fn spawn(app: AppHandle) {
    thread::spawn(move || {
        let mut ticker = Ticker::new(app);
        loop {
            thread::sleep(TICK);
            ticker.tick();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use breeze_domain::constants::IDLE_FREEZE;
    use breeze_domain::{ActiveDays, Cycle, Minutes, Rhythm, Severity};

    fn cycle() -> Cycle {
        let rhythm = Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap();
        Cycle::start(rhythm, Severity::Simple, Instant::EPOCH)
    }

    #[test]
    fn a_running_countdown_shows_minutes_and_seconds() {
        let snapshot = CycleSnapshot::of(&cycle());
        assert_eq!(tray_title(&snapshot, Instant::EPOCH, true), "50:00");
    }

    #[test]
    fn a_frozen_countdown_shows_its_frozen_remaining() {
        let mut frozen = cycle();
        frozen.freeze_if_idle(Instant::EPOCH.plus(IDLE_FREEZE));
        let snapshot = CycleSnapshot::of(&frozen);
        let later = Instant::EPOCH.plus(Duration::from_secs(3600));
        assert_eq!(tray_title(&snapshot, later, true), "47:00");
    }

    #[test]
    fn an_inactive_cycle_shows_nothing() {
        let mut resting = cycle();
        resting.observe_calendar(Instant::EPOCH, false);
        let snapshot = CycleSnapshot::of(&resting);
        assert_eq!(tray_title(&snapshot, Instant::EPOCH, true), "");
    }

    #[test]
    fn a_wall_clock_far_ahead_of_the_monotonic_one_means_the_machine_slept() {
        let before = WallClock::from_unix_secs(1_000);
        let after = WallClock::from_unix_secs(1_000 + 3_600);
        let now = Instant::EPOCH.plus(Duration::from_millis(250));
        assert_eq!(
            slept_between(before, Instant::EPOCH, after, now),
            Some(Duration::from_secs(3_600))
        );
    }

    #[test]
    fn clocks_that_advance_together_mean_no_sleep() {
        let before = WallClock::from_unix_secs(1_000);
        let after = WallClock::from_unix_secs(1_300);
        let now = Instant::EPOCH.plus(Duration::from_secs(300));
        assert_eq!(slept_between(before, Instant::EPOCH, after, now), None);
    }

    #[test]
    fn a_wall_clock_set_back_is_never_a_sleep() {
        let before = WallClock::from_unix_secs(10_000);
        let after = WallClock::from_unix_secs(1_000);
        let now = Instant::EPOCH.plus(Duration::from_secs(1));
        assert_eq!(slept_between(before, Instant::EPOCH, after, now), None);
    }

    #[test]
    fn a_break_shows_its_own_countdown_in_the_menu_bar() {
        let mut resting = cycle();
        let break_at = Instant::EPOCH
            .plus(Duration::from_secs(50 * 60))
            .plus(breeze_domain::constants::NOTICE);
        resting.tick(break_at);
        let snapshot = CycleSnapshot::of(&resting);
        assert_eq!(snapshot.phase, CyclePhase::Break);
        assert_eq!(tray_title(&snapshot, break_at, true), "10:00");
    }

    #[test]
    fn the_icon_only_mode_shows_nothing() {
        let snapshot = CycleSnapshot::of(&cycle());
        assert_eq!(tray_title(&snapshot, Instant::EPOCH, false), "");
    }
}
