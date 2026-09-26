use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{
    ActiveDays, AppId, AppStatus, AppStatuses, CommandError, Cycle, CycleState, Instant, Minutes,
    Rhythm, SafetyList, Severity, SparedApps, StatusChange,
};

fn id(raw: &str) -> AppId {
    AppId::parse(raw).unwrap()
}

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn terminal() -> AppId {
    id("com.apple.Terminal")
}

fn cycle_with(chosen: &[(&str, AppStatus)]) -> Cycle {
    let chosen = SparedApps::from_pairs(chosen.iter().map(|(raw, status)| (id(raw), *status)));
    let apps = AppStatuses::new(chosen, SafetyList::new([terminal()]));
    Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH).with_app_statuses(apps)
}

// Instant où le travail suivant commence : travail, préavis, pause, retour.
fn next_work_at() -> Instant {
    let r = rhythm();
    Instant::EPOCH
        .plus(r.work().as_duration())
        .plus(NOTICE)
        .plus(r.pause().as_duration())
        .plus(RETURN_HOLD)
}

#[test]
fn an_unknown_app_is_blocked_and_effective_from_the_first_cycle() {
    let cycle = cycle_with(&[]);
    let apps = cycle.app_statuses();
    assert_eq!(
        apps.effective_status(&id("com.unknown.App")),
        AppStatus::Blocked
    );
    assert!(!apps.applies_next_cycle(&id("com.unknown.App")));
}

#[test]
fn statuses_chosen_before_launch_are_effective_at_once() {
    let cycle = cycle_with(&[("com.apple.Music", AppStatus::Ignored)]);
    assert_eq!(
        cycle
            .app_statuses()
            .effective_status(&id("com.apple.Music")),
        AppStatus::Ignored
    );
}

#[test]
fn sparing_a_blocked_app_waits_for_the_next_cycle() {
    let mut cycle = cycle_with(&[]);
    let music = id("com.apple.Music");

    let change = cycle.set_app_status(music.clone(), AppStatus::Spared);

    assert_eq!(change, Ok(StatusChange::AppliesNextCycle));
    let apps = cycle.app_statuses();
    assert_eq!(apps.chosen_status(&music), AppStatus::Spared);
    assert_eq!(apps.effective_status(&music), AppStatus::Blocked);
    assert!(apps.applies_next_cycle(&music));
}

#[test]
fn ignoring_a_spared_app_waits_for_the_next_cycle() {
    let mut cycle = cycle_with(&[("com.apple.Music", AppStatus::Spared)]);
    let change = cycle.set_app_status(id("com.apple.Music"), AppStatus::Ignored);
    assert_eq!(change, Ok(StatusChange::AppliesNextCycle));
    assert_eq!(
        cycle
            .app_statuses()
            .effective_status(&id("com.apple.Music")),
        AppStatus::Spared
    );
}

#[test]
fn blocking_an_app_applies_immediately() {
    let mut cycle = cycle_with(&[("com.apple.Music", AppStatus::Ignored)]);
    let change = cycle.set_app_status(id("com.apple.Music"), AppStatus::Blocked);
    assert_eq!(change, Ok(StatusChange::AppliesNow));
    assert_eq!(
        cycle
            .app_statuses()
            .effective_status(&id("com.apple.Music")),
        AppStatus::Blocked
    );
}

#[test]
fn ceasing_to_ignore_an_app_applies_immediately() {
    let mut cycle = cycle_with(&[("com.apple.Music", AppStatus::Ignored)]);
    let change = cycle.set_app_status(id("com.apple.Music"), AppStatus::Spared);
    assert_eq!(change, Ok(StatusChange::AppliesNow));
    assert_eq!(
        cycle
            .app_statuses()
            .effective_status(&id("com.apple.Music")),
        AppStatus::Spared
    );
}

#[test]
fn going_back_on_a_pending_weakening_cancels_it_at_once() {
    let mut cycle = cycle_with(&[]);
    let music = id("com.apple.Music");
    cycle
        .set_app_status(music.clone(), AppStatus::Ignored)
        .unwrap();

    let change = cycle.set_app_status(music.clone(), AppStatus::Blocked);

    assert_eq!(change, Ok(StatusChange::AppliesNow));
    assert!(!cycle.app_statuses().applies_next_cycle(&music));
}

#[test]
fn a_pending_weakening_takes_effect_when_the_next_work_begins() {
    let mut cycle = cycle_with(&[]);
    let music = id("com.apple.Music");
    cycle
        .set_app_status(music.clone(), AppStatus::Ignored)
        .unwrap();

    let before = next_work_at().minus(core::time::Duration::from_secs(1));
    cycle.tick(before);
    assert_eq!(
        cycle.app_statuses().effective_status(&music),
        AppStatus::Blocked,
        "the break of the current cycle still sees the app blocked"
    );

    cycle.tick(next_work_at());
    assert_eq!(
        cycle.app_statuses().effective_status(&music),
        AppStatus::Ignored
    );
    assert!(!cycle.app_statuses().applies_next_cycle(&music));
}

#[test]
fn a_safety_listed_app_is_always_spared_and_refuses_any_change() {
    let mut cycle = cycle_with(&[]);
    let apps = cycle.app_statuses();
    assert!(apps.is_locked(&terminal()));
    assert_eq!(apps.effective_status(&terminal()), AppStatus::Spared);
    assert_eq!(apps.chosen_status(&terminal()), AppStatus::Spared);

    for status in [AppStatus::Blocked, AppStatus::Spared, AppStatus::Ignored] {
        assert_eq!(
            cycle.set_app_status(terminal(), status),
            Err(CommandError::LockedApp)
        );
    }
}

#[test]
fn the_safety_list_wins_over_a_stale_persisted_choice() {
    let cycle = cycle_with(&[("com.apple.Terminal", AppStatus::Ignored)]);
    assert_eq!(
        cycle.app_statuses().effective_status(&terminal()),
        AppStatus::Spared
    );
}

#[test]
fn a_reset_blocks_everything_at_once() {
    let mut cycle = cycle_with(&[("com.apple.Music", AppStatus::Spared)]);
    cycle
        .set_app_status(id("com.apple.Notes"), AppStatus::Ignored)
        .unwrap();

    cycle.reset_app_statuses();

    let apps = cycle.app_statuses();
    assert_eq!(
        apps.effective_status(&id("com.apple.Music")),
        AppStatus::Blocked
    );
    assert!(!apps.applies_next_cycle(&id("com.apple.Notes")));
    assert!(apps.chosen().pairs().is_empty());
}

#[test]
fn only_effectively_blocked_apps_are_veiled() {
    let mut cycle = cycle_with(&[("com.apple.Music", AppStatus::Spared)]);
    cycle
        .set_app_status(id("com.apple.Notes"), AppStatus::Spared)
        .unwrap();

    assert!(!cycle.is_veiled(Some(&id("com.apple.Music"))));
    assert!(!cycle.is_veiled(Some(&terminal())));
    assert!(
        cycle.is_veiled(Some(&id("com.apple.Notes"))),
        "a pending weakening does not lift the veil of this cycle"
    );
    assert!(
        cycle.is_veiled(None),
        "a window without identity is unknown, hence blocked"
    );
}

#[test]
fn no_status_changes_while_a_break_is_due() {
    let r = rhythm();
    let work_ends = Instant::EPOCH.plus(r.work().as_duration());
    let break_starts = work_ends.plus(NOTICE);
    let return_starts = break_starts.plus(r.pause().as_duration());
    let music = id("com.apple.Music");

    for (phase, at) in [
        ("notice", work_ends),
        ("break", break_starts),
        ("return", return_starts),
    ] {
        let mut cycle = cycle_with(&[]);
        cycle.tick(at);
        let in_phase = match phase {
            "notice" => matches!(cycle.state(), CycleState::Notice { .. }),
            "break" => matches!(cycle.state(), CycleState::BreakActive { .. }),
            _ => matches!(cycle.state(), CycleState::Returning { .. }),
        };
        assert!(in_phase, "{phase}: unexpected state {:?}", cycle.state());
        for status in [AppStatus::Blocked, AppStatus::Spared, AppStatus::Ignored] {
            assert_eq!(
                cycle.set_app_status(music.clone(), status),
                Err(CommandError::BreakDue),
                "{phase}: {status:?} must wait for the break to end"
            );
        }
        assert!(
            cycle.app_statuses().chosen().pairs().is_empty(),
            "{phase}: nothing was chosen"
        );
    }
}

#[test]
fn statuses_can_change_again_once_the_next_work_begins() {
    let mut cycle = cycle_with(&[]);
    cycle.tick(next_work_at());

    let change = cycle.set_app_status(id("com.apple.Music"), AppStatus::Blocked);

    assert_eq!(change, Ok(StatusChange::AppliesNow));
}
