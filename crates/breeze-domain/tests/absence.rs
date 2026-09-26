use breeze_domain::constants::RETURN_HOLD;
use breeze_domain::{
    absence_verdict, Absence, AbsenceVerdict, ActiveDays, BreakMode, BreakOutcome, Countdown,
    Cycle, CycleState, Instant, Minutes, Rhythm, Severity,
};
use core::time::Duration;

fn rhythm(work: u16, pause: u16) -> Rhythm {
    Rhythm::new(Minutes(work), Minutes(pause), None, ActiveDays::everyday()).unwrap()
}

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

fn at(n: u64) -> Instant {
    Instant::at_secs(n)
}

fn absence(began_at: Instant, lasted: u64) -> Absence {
    Absence {
        began_at,
        lasted: secs(lasted),
    }
}

fn verdict(state: CycleState, lasted: u64) -> AbsenceVerdict {
    absence_verdict(state, rhythm(50, 10), absence(Instant::EPOCH, lasted))
}

fn working_running(deadline: u64) -> CycleState {
    CycleState::Working {
        countdown: Countdown::Running {
            deadline: at(deadline),
        },
    }
}

// Rythme 50/10 : travail 3000 s, pause 600 s.

#[test]
fn inactive_never_yields_a_verdict() {
    assert_eq!(
        verdict(CycleState::Inactive, 100_000),
        AbsenceVerdict::Nothing
    );
}

#[test]
fn a_suspension_never_yields_a_verdict() {
    let state = CycleState::Suspended {
        resume_at: at(9000),
        frozen: secs(1200),
    };
    assert_eq!(verdict(state, 100_000), AbsenceVerdict::Nothing);
}

#[test]
fn a_frozen_work_only_validates_the_cycle_beyond_the_pause_length() {
    let state = CycleState::Working {
        countdown: Countdown::Frozen {
            remaining: secs(2820),
        },
    };
    assert_eq!(verdict(state, 700), AbsenceVerdict::CycleValidated);
}

#[test]
fn a_short_absence_leaves_a_frozen_work_untouched() {
    let state = CycleState::Working {
        countdown: Countdown::Frozen {
            remaining: secs(2820),
        },
    };
    assert_eq!(verdict(state, 600), AbsenceVerdict::Nothing);
}

#[test]
fn a_work_absence_longer_than_the_pause_validates_the_cycle() {
    assert_eq!(
        verdict(working_running(3000), 700),
        AbsenceVerdict::CycleValidated
    );
}

#[test]
fn a_work_absence_past_the_deadline_but_within_the_pause_skips_the_notice() {
    // Échéance proche (100 s) : l'écart de 200 s la franchit sans dépasser la pause.
    assert_eq!(
        verdict(working_running(100), 200),
        AbsenceVerdict::BreakStartsAtWake
    );
}

#[test]
fn a_work_absence_short_of_the_deadline_keeps_the_wall_clock_running() {
    assert_eq!(
        verdict(working_running(3000), 500),
        AbsenceVerdict::PhaseContinues {
            remaining: secs(2500)
        }
    );
}

#[test]
fn a_work_absence_exactly_at_the_deadline_continues_with_zero() {
    assert_eq!(
        verdict(working_running(100), 100),
        AbsenceVerdict::PhaseContinues { remaining: secs(0) }
    );
}

#[test]
fn a_due_break_starts_at_wake() {
    let state = CycleState::Working {
        countdown: Countdown::Due {
            since: Instant::EPOCH,
        },
    };
    assert_eq!(verdict(state, 200), AbsenceVerdict::BreakStartsAtWake);
}

#[test]
fn a_due_break_is_validated_beyond_the_pause_length() {
    let state = CycleState::Working {
        countdown: Countdown::Due {
            since: Instant::EPOCH,
        },
    };
    assert_eq!(verdict(state, 700), AbsenceVerdict::CycleValidated);
}

#[test]
fn a_notice_absence_past_its_end_starts_the_break_at_wake() {
    let state = CycleState::Notice { deadline: at(30) };
    assert_eq!(verdict(state, 40), AbsenceVerdict::BreakStartsAtWake);
}

#[test]
fn a_notice_absence_short_of_its_end_continues_the_notice() {
    let state = CycleState::Notice { deadline: at(30) };
    assert_eq!(
        verdict(state, 20),
        AbsenceVerdict::PhaseContinues {
            remaining: secs(10)
        }
    );
}

#[test]
fn a_notice_absence_longer_than_the_pause_validates_the_cycle() {
    let state = CycleState::Notice { deadline: at(30) };
    assert_eq!(verdict(state, 700), AbsenceVerdict::CycleValidated);
}

#[test]
fn a_break_absence_longer_than_the_remaining_serves_it() {
    let state = CycleState::BreakActive {
        deadline: at(400),
        severity: Severity::Simple,
        mode: BreakMode::Nominal,
    };
    assert_eq!(verdict(state, 500), AbsenceVerdict::BreakServed);
}

#[test]
fn a_break_absence_within_the_remaining_pushes_the_deadline_by_the_whole_gap() {
    let state = CycleState::BreakActive {
        deadline: at(400),
        severity: Severity::Simple,
        mode: BreakMode::Nominal,
    };
    assert_eq!(
        verdict(state, 300),
        AbsenceVerdict::PhaseContinues {
            remaining: secs(400)
        }
    );
}

#[test]
fn a_break_absence_exactly_the_remaining_is_still_frozen() {
    let state = CycleState::BreakActive {
        deadline: at(400),
        severity: Severity::Simple,
        mode: BreakMode::Nominal,
    };
    assert_eq!(
        verdict(state, 400),
        AbsenceVerdict::PhaseContinues {
            remaining: secs(400)
        }
    );
}

#[test]
fn a_return_absence_finishes_the_hold_at_wake() {
    let state = CycleState::Returning { deadline: at(3) };
    assert_eq!(
        verdict(state, 100_000),
        AbsenceVerdict::PhaseContinues { remaining: secs(3) }
    );
}

#[test]
fn a_zero_length_absence_yields_nothing() {
    assert_eq!(verdict(working_running(3000), 0), AbsenceVerdict::Nothing);
    let due = CycleState::Working {
        countdown: Countdown::Due {
            since: Instant::EPOCH,
        },
    };
    assert_eq!(verdict(due, 0), AbsenceVerdict::Nothing);
}

#[test]
fn a_work_absence_exactly_the_pause_length_does_not_validate_the_cycle() {
    assert_eq!(
        verdict(working_running(3000), 600),
        AbsenceVerdict::PhaseContinues {
            remaining: secs(2400)
        }
    );
}

#[test]
fn a_notice_absence_exactly_the_pause_length_only_skips_the_notice() {
    let state = CycleState::Notice { deadline: at(30) };
    assert_eq!(verdict(state, 600), AbsenceVerdict::BreakStartsAtWake);
}

#[test]
fn a_work_absence_past_the_pause_wins_over_a_crossed_deadline() {
    // Échéance proche (r=100 < pause) et écart > pause : le cas 1 prime sur le cas 3.
    assert_eq!(
        verdict(working_running(100), 700),
        AbsenceVerdict::CycleValidated
    );
}

#[test]
fn a_validated_cycle_credits_an_absence_outcome_and_applies_the_pending_rhythm() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    cycle.change_rhythm(rhythm(25, 5)).unwrap();

    let outcome = cycle.return_from_absence(absence(Instant::EPOCH, 700), at(10_000));

    assert_eq!(outcome, AbsenceVerdict::CycleValidated);
    assert_eq!(cycle.outcomes(), &[BreakOutcome::ValidatedByAbsence]);
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => assert_eq!(deadline, at(10_000).plus(secs(1500))),
        other => panic!("expected a fresh working cycle on the new rhythm, got {other:?}"),
    }
}

#[test]
fn a_notice_crossed_by_a_short_absence_opens_the_break_at_wake() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    cycle.tick(at(3000));
    assert!(matches!(cycle.state(), CycleState::Notice { .. }));

    let outcome = cycle.return_from_absence(absence(at(3000), 120), at(10_000));

    assert_eq!(outcome, AbsenceVerdict::BreakStartsAtWake);
    assert!(cycle.outcomes().is_empty());
    match cycle.state() {
        CycleState::BreakActive { deadline, .. } => {
            assert_eq!(deadline, at(10_000).plus(secs(600)));
        }
        other => panic!("expected a break opening at wake, got {other:?}"),
    }
}

#[test]
fn a_break_served_by_absence_credits_it_once_and_enters_returning() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    assert!(matches!(cycle.state(), CycleState::BreakActive { .. }));

    let outcome = cycle.return_from_absence(absence(at(3060), 700), at(10_000));

    assert_eq!(outcome, AbsenceVerdict::BreakServed);
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Served { planned: secs(600) }]
    );
    match cycle.state() {
        CycleState::Returning { deadline } => {
            assert_eq!(deadline, at(10_000).plus(RETURN_HOLD));
        }
        other => panic!("expected returning, got {other:?}"),
    }
}

#[test]
fn a_return_finishes_at_wake_without_a_second_credit() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    cycle.tick(at(3660));
    assert!(matches!(cycle.state(), CycleState::Returning { .. }));
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Served { planned: secs(600) }]
    );

    let outcome = cycle.return_from_absence(absence(at(3660), 1), at(10_000));

    assert_eq!(
        outcome,
        AbsenceVerdict::PhaseContinues {
            remaining: RETURN_HOLD
        }
    );
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Served { planned: secs(600) }]
    );
    match cycle.state() {
        CycleState::Returning { deadline } => {
            assert_eq!(deadline, at(10_000).plus(RETURN_HOLD));
        }
        other => panic!("expected returning re-anchored at wake, got {other:?}"),
    }
}

#[test]
fn a_short_work_absence_keeps_the_original_deadline() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);

    let outcome = cycle.return_from_absence(absence(at(1000), 500), at(1500));

    assert_eq!(
        outcome,
        AbsenceVerdict::PhaseContinues {
            remaining: secs(1500)
        }
    );
    assert_eq!(cycle.state(), working_running(3000));
}

#[test]
fn a_validated_cycle_applies_the_pending_severity() {
    let mut cycle = Cycle::start(rhythm(50, 10), Severity::Hardcore, Instant::EPOCH);
    cycle.change_severity(Severity::Simple).unwrap();
    assert_eq!(cycle.severity(), Severity::Hardcore);

    cycle.return_from_absence(absence(Instant::EPOCH, 700), at(10_000));

    assert_eq!(cycle.severity(), Severity::Simple);
}
