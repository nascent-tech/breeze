use breeze_domain::PostureDebt;
use core::time::Duration;

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

#[test]
fn credits_accumulate() {
    let mut debt = PostureDebt::none();
    debt.credit(secs(480));
    debt.credit(secs(120));
    assert_eq!(debt.total(), secs(600));
    assert!(!debt.is_none());
}

#[test]
fn an_absorption_is_bounded_by_what_is_owed() {
    let mut debt = PostureDebt::restore(secs(300));
    debt.absorb(secs(900));
    assert_eq!(debt.owed(), secs(0));
    assert_eq!(debt.total(), secs(300));
}

#[test]
fn settling_clears_the_absorbed_part() {
    let mut debt = PostureDebt::restore(secs(600));
    debt.absorb(secs(200));
    debt.settle();
    assert_eq!(debt.total(), secs(400));
}

#[test]
fn freezing_returns_the_absorbed_part_to_what_is_owed() {
    let mut debt = PostureDebt::restore(secs(600));
    debt.absorb(secs(200));
    debt.freeze();
    assert_eq!(debt.owed(), secs(600));
    assert_eq!(debt.total(), secs(600));
}

#[test]
fn displayed_minutes_round_up_and_never_hide_a_remainder() {
    assert_eq!(PostureDebt::restore(secs(450)).minutes(), 8);
    assert_eq!(PostureDebt::restore(secs(1)).minutes(), 1);
    assert_eq!(PostureDebt::none().minutes(), 0);
}

#[test]
fn a_debt_being_repaid_still_shows_on_the_icon() {
    // Pendant une pause allongée : tout est absorbé, rien n'est encore remboursé.
    let mut debt = PostureDebt::restore(secs(480));
    debt.absorb(secs(480));
    assert_eq!(debt.owed(), secs(0));
    assert_eq!(debt.minutes(), 8);
}
