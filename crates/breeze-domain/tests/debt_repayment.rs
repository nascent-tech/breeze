use breeze_domain::{
    Absence, ActiveDays, BreakOutcome, Cycle, CycleState, Instant, InterruptionDoor, Minutes,
    PostureDebt, Rhythm, Severity,
};
use core::time::Duration;

fn rhythm(work: u16, pause: u16) -> Rhythm {
    Rhythm::new(Minutes(work), Minutes(pause), None, ActiveDays::everyday()).unwrap()
}

fn at(n: u64) -> Instant {
    Instant::at_secs(n)
}

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

// 50/10 : travail 3000 s, pause 600 s ; préavis 60 s → pause à 3060.
fn hardcore_break_with_debt(debt: Duration) -> Cycle {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Hardcore, Instant::EPOCH)
        .with_debt(PostureDebt::restore(debt));
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    cycle
}

#[test]
fn interrupting_two_minutes_into_a_ten_minute_break_credits_eight() {
    let mut cycle = hardcore_break_with_debt(secs(0));
    cycle.interrupt_break(at(3180)).unwrap(); // 3060 + 120 s
    assert_eq!(cycle.debt().total(), secs(480));
}

#[test]
fn a_debt_extends_the_next_break_and_a_served_break_repays_it() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH)
        .with_debt(PostureDebt::restore(secs(480)));
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    // Pause allongée de 480 s : échéance 3060 + 600 + 480 = 4140.
    match cycle.state() {
        CycleState::BreakActive { deadline, .. } => assert_eq!(deadline, at(4140)),
        other => panic!("expected a lengthened break, got {other:?}"),
    }
    cycle.tick(at(4140)); // servie jusqu'au terme → remboursée
    assert_eq!(cycle.debt().total(), secs(0));
    assert!(matches!(cycle.state(), CycleState::Returning { .. }));
}

#[test]
fn the_extension_never_exceeds_the_work_duration() {
    // 25/10 : travail 1500 s, pause 600 s. Plafond = travail = 1500 s → allongement 900 s max.
    let mut cycle = Cycle::start(rhythm(25, 10), Severity::Simple, Instant::EPOCH)
        .with_debt(PostureDebt::restore(secs(1800)));
    cycle.tick(at(1500));
    cycle.tick(at(1560)); // préavis 60 s
    cycle.tick(at(1560 + 1500)); // pause plafonnée à 1500 s, servie
    assert_eq!(cycle.debt().total(), secs(900));
}

#[test]
fn the_extension_never_exceeds_the_pause_ceiling_of_sixty_minutes() {
    // 90/20 : travail 5400 s, pause 1200 s. Plafond = 3600 s → allongement 2400 s max.
    let mut cycle = Cycle::start(rhythm(90, 20), Severity::Simple, Instant::EPOCH)
        .with_debt(PostureDebt::restore(secs(3000)));
    cycle.tick(at(5400));
    cycle.tick(at(5460)); // préavis
    cycle.tick(at(5460 + 3600)); // pause plafonnée à 3600 s, servie
    assert_eq!(cycle.debt().total(), secs(600));
}

#[test]
fn interrupting_a_lengthened_break_credits_what_was_not_lived() {
    let mut cycle = hardcore_break_with_debt(secs(480)); // pause de 1080 s, échéance 4140
    cycle.interrupt_break(at(3780)).unwrap(); // 12 min vécues (3060 + 720)
    assert_eq!(cycle.debt().total(), secs(360)); // 8 anciennes − 2 vécues = 6 min
                                                 // La pause prévue compte l'allongement : 12 min tenues sur 18.
    assert_eq!(cycle.outcomes()[0].held(), secs(720));
}

#[test]
fn a_lengthened_break_served_to_its_end_counts_its_extension_as_held() {
    let mut cycle = hardcore_break_with_debt(secs(480)); // pause de 1080 s, échéance 4140
    cycle.tick(at(4140));
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Served {
            planned: secs(1080)
        }]
    );
}

#[test]
fn an_absence_that_serves_a_lengthened_break_freezes_the_debt() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH)
        .with_debt(PostureDebt::restore(secs(480)));
    cycle.tick(at(3000));
    cycle.tick(at(3060)); // pause allongée, échéance 4140, restant 1080 s

    let absence = Absence {
        began_at: at(3060),
        lasted: secs(2000), // > 1080 : la pause est validée servie
    };
    cycle.return_from_absence(absence, at(9000));

    assert_eq!(cycle.debt().total(), secs(480)); // gelée : ni remboursée ni recréditée
                                                 // Servie par l'absence, la pause n'a tenu que sa durée réglée : l'allongement reste dû.
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Served { planned: secs(600) }]
    );
}

#[test]
fn a_cycle_validated_by_absence_leaves_the_debt_intact() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH)
        .with_debt(PostureDebt::restore(secs(300)));
    // Absence plus longue que la pause, pendant le travail → cycle validé.
    let absence = Absence {
        began_at: Instant::EPOCH,
        lasted: secs(700),
    };
    cycle.return_from_absence(absence, at(9000));
    assert_eq!(cycle.debt().total(), secs(300));
}

#[test]
fn terminating_during_notice_credits_the_whole_configured_pause() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    cycle.tick(at(3000)); // → préavis, échéance 3060
    cycle.terminate(at(3030), InterruptionDoor::TrayMenu);
    assert_eq!(cycle.debt().total(), secs(600));
    assert!(matches!(cycle.state(), CycleState::Working { .. }));
}

#[test]
fn terminating_during_work_credits_nothing_and_is_idempotent() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    cycle.terminate(at(10), InterruptionDoor::Quit);
    assert_eq!(cycle.debt().total(), secs(0));
    assert!(cycle.outcomes().is_empty());
}

#[test]
fn terminating_twice_credits_once() {
    let mut cycle = hardcore_break_with_debt(secs(0));
    cycle.terminate(at(3300), InterruptionDoor::TrayMenu);
    cycle.terminate(at(3400), InterruptionDoor::Quit);
    assert_eq!(cycle.outcomes().len(), 1);
}

#[test]
fn a_crash_freezes_the_debt_without_crediting() {
    let mut cycle = hardcore_break_with_debt(secs(480)); // absorbed 480, owed 0
    cycle.terminate(at(3300), InterruptionDoor::Crash);
    // La chute gèle l'absorbé (§9.2, décision 15) sans créditer le restant non vécu.
    assert_eq!(cycle.debt().total(), secs(480));
}

#[test]
fn clearing_the_debt_keeps_the_current_break_deadline() {
    let mut cycle = hardcore_break_with_debt(secs(480));
    let deadline = match cycle.state() {
        CycleState::BreakActive { deadline, .. } => deadline,
        other => panic!("expected a break, got {other:?}"),
    };
    cycle.clear_debt();
    assert_eq!(cycle.debt().total(), secs(0));
    match cycle.state() {
        CycleState::BreakActive { deadline: kept, .. } => assert_eq!(kept, deadline),
        other => panic!("expected the break to keep its deadline, got {other:?}"),
    }
}
