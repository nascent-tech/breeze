use breeze_app::{CyclePhase, CycleSnapshot, VeilMode};
use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{FreezeReason, Instant, Rhythm, Severity};
use core::time::Duration;
use serde::Serialize;

#[derive(Serialize)]
pub struct SnapshotDto {
    pub phase: String,
    pub remaining_secs: u64,
    pub total_secs: u64,
    pub break_in_secs: u64,
    pub frozen: bool,
    // Pourquoi le décompte est gelé : "idle" | "ignored_app" ; null s'il court.
    pub frozen_reason: Option<&'static str>,
    // Tenue de la pause Simple : "windows" | "fullscreen" ; null hors pause Simple.
    pub veil_mode: Option<&'static str>,
    pub severity: String,
    pub chosen_severity: String,
    pub rhythm_pending: bool,
    pub served_breaks: u32,
    pub served_today: u32,
    pub work_minutes: u16,
    pub pause_minutes: u16,
    pub debt_minutes: u16,
    pub inactive_reason: Option<&'static str>,
    pub next_start_label: Option<String>,
}

// Ce que l'instantané du cycle ne sait pas : le journal persisté et le calendrier local.
#[derive(Default)]
pub struct DayContext {
    pub served_today: u32,
    pub inactive_reason: Option<&'static str>,
    pub next_start_label: Option<String>,
}

fn phase_name(phase: CyclePhase) -> &'static str {
    match phase {
        CyclePhase::Inactive => "Inactive",
        CyclePhase::Working => "Working",
        CyclePhase::Notice => "Notice",
        CyclePhase::Break => "Break",
        CyclePhase::Returning => "Returning",
        CyclePhase::Suspended => "Suspended",
    }
}

fn phase_total(phase: CyclePhase, rhythm: &Rhythm) -> Duration {
    match phase {
        CyclePhase::Working => rhythm.work().as_duration(),
        CyclePhase::Notice => NOTICE,
        CyclePhase::Break => rhythm.pause().as_duration(),
        CyclePhase::Returning => RETURN_HOLD,
        CyclePhase::Inactive | CyclePhase::Suspended => Duration::ZERO,
    }
}

fn freeze_reason_name(reason: FreezeReason) -> &'static str {
    match reason {
        FreezeReason::Idle => "idle",
        FreezeReason::IgnoredApp => "ignored_app",
    }
}

fn veil_mode_name(mode: VeilMode) -> &'static str {
    match mode {
        VeilMode::Windows => "windows",
        VeilMode::FullScreen => "fullscreen",
    }
}

pub fn severity_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Simple => "Simple",
        Severity::Hardcore => "Hardcore",
    }
}

fn seconds_until_break(phase: CyclePhase, remaining: u64) -> u64 {
    match phase {
        CyclePhase::Working => remaining.saturating_add(NOTICE.as_secs()),
        CyclePhase::Notice => remaining,
        _ => 0,
    }
}

pub fn remaining_of(snapshot: &CycleSnapshot, now: Instant) -> Duration {
    match (snapshot.deadline, snapshot.frozen_remaining) {
        (Some(deadline), _) => deadline.elapsed_since(now),
        (None, Some(frozen)) => frozen,
        (None, None) => Duration::ZERO,
    }
}

pub fn to_dto(
    snapshot: CycleSnapshot,
    now: Instant,
    active: &Rhythm,
    configured: &Rhythm,
    day: DayContext,
) -> SnapshotDto {
    let remaining = remaining_of(&snapshot, now).as_secs();
    SnapshotDto {
        phase: phase_name(snapshot.phase).to_owned(),
        remaining_secs: remaining,
        total_secs: phase_total(snapshot.phase, active).as_secs(),
        break_in_secs: seconds_until_break(snapshot.phase, remaining),
        frozen: snapshot.frozen_remaining.is_some(),
        frozen_reason: snapshot.frozen_reason.map(freeze_reason_name),
        veil_mode: snapshot.veil_mode.map(veil_mode_name),
        severity: severity_name(snapshot.severity).to_owned(),
        chosen_severity: severity_name(snapshot.chosen_severity).to_owned(),
        rhythm_pending: snapshot.rhythm_pending,
        served_breaks: snapshot.served_breaks,
        served_today: day.served_today,
        work_minutes: configured.work().count(),
        pause_minutes: configured.pause().count(),
        debt_minutes: snapshot.debt_minutes,
        inactive_reason: day.inactive_reason,
        next_start_label: day.next_start_label,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use breeze_domain::constants::IDLE_FREEZE;
    use breeze_domain::{ActiveDays, Cycle, Minutes};

    fn rhythm() -> Rhythm {
        Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
    }

    fn dto_of(cycle: &Cycle, now: Instant, active: &Rhythm, configured: &Rhythm) -> SnapshotDto {
        to_dto(
            CycleSnapshot::of(cycle),
            now,
            active,
            configured,
            DayContext::default(),
        )
    }

    #[test]
    fn a_fresh_working_cycle_reports_break_after_work_plus_notice() {
        let r = rhythm();
        let dto = dto_of(
            &Cycle::start(r, Severity::Simple, Instant::EPOCH),
            Instant::EPOCH,
            &r,
            &r,
        );
        assert_eq!(dto.phase, "Working");
        assert_eq!(dto.total_secs, r.work().as_duration().as_secs());
        assert_eq!(
            dto.break_in_secs,
            r.work().as_duration().as_secs() + NOTICE.as_secs()
        );
        assert_eq!(dto.severity, "Simple");
        assert!(!dto.frozen);
        assert_eq!(dto.frozen_reason, None);
        assert_eq!(dto.veil_mode, None);
        assert_eq!(dto.work_minutes, 50);
        assert_eq!(dto.pause_minutes, 10);
    }

    #[test]
    fn the_reported_severity_is_the_configured_one_even_while_working() {
        let r = rhythm();
        let dto = dto_of(
            &Cycle::start(r, Severity::Hardcore, Instant::EPOCH),
            Instant::EPOCH,
            &r,
            &r,
        );
        assert_eq!(dto.severity, "Hardcore");
        assert_eq!(dto.chosen_severity, "Hardcore");
    }

    #[test]
    fn a_pending_return_to_simple_shows_both_severities() {
        let r = rhythm();
        let mut cycle = Cycle::start(r, Severity::Hardcore, Instant::EPOCH);
        cycle.change_severity(Severity::Simple).unwrap();
        let dto = dto_of(&cycle, Instant::EPOCH, &r, &r);
        assert_eq!(dto.severity, "Hardcore");
        assert_eq!(dto.chosen_severity, "Simple");
    }

    #[test]
    fn the_ring_follows_the_active_rhythm_while_settings_show_the_configured_one() {
        let active = rhythm();
        let configured =
            Rhythm::new(Minutes(25), Minutes(5), None, ActiveDays::everyday()).unwrap();
        let mut cycle = Cycle::start(active, Severity::Simple, Instant::EPOCH);
        cycle.change_rhythm(configured).unwrap();
        let dto = dto_of(&cycle, Instant::EPOCH, &active, &configured);
        assert_eq!(dto.total_secs, active.work().as_duration().as_secs());
        assert_eq!(dto.work_minutes, 25);
        assert_eq!(dto.pause_minutes, 5);
        assert!(dto.rhythm_pending);
    }

    #[test]
    fn a_frozen_countdown_reports_its_frozen_remaining_not_zero() {
        let r = rhythm();
        let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
        let froze_at = Instant::EPOCH.plus(IDLE_FREEZE);
        cycle.freeze_if_idle(froze_at);
        let much_later = froze_at.plus(Duration::from_secs(3600));

        let dto = dto_of(&cycle, much_later, &r, &r);

        let expected = (r.work().as_duration() - IDLE_FREEZE).as_secs();
        assert!(dto.frozen);
        assert_eq!(dto.frozen_reason, Some("idle"));
        assert_eq!(dto.remaining_secs, expected);
        assert_eq!(dto.break_in_secs, expected + NOTICE.as_secs());
    }

    #[test]
    fn a_simple_break_reports_how_it_is_veiled() {
        let r = rhythm();
        let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
        cycle.observe_frames(true);
        let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
        cycle.tick(break_at);
        assert_eq!(dto_of(&cycle, break_at, &r, &r).veil_mode, Some("windows"));

        let mut blind = Cycle::start(r, Severity::Simple, Instant::EPOCH);
        blind.tick(break_at);
        assert_eq!(
            dto_of(&blind, break_at, &r, &r).veil_mode,
            Some("fullscreen")
        );
    }

    #[test]
    fn a_hardcore_break_has_no_veil_mode() {
        let r = rhythm();
        let mut cycle = Cycle::start(r, Severity::Hardcore, Instant::EPOCH);
        let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
        cycle.tick(break_at);
        assert_eq!(dto_of(&cycle, break_at, &r, &r).veil_mode, None);
    }

    #[test]
    fn the_day_context_is_carried_as_is() {
        let r = rhythm();
        let dto = to_dto(
            CycleSnapshot::of(&Cycle::start(r, Severity::Simple, Instant::EPOCH)),
            Instant::EPOCH,
            &r,
            &r,
            DayContext {
                served_today: 4,
                inactive_reason: Some("schedule"),
                next_start_label: Some("demain 9:00".to_owned()),
            },
        );
        assert_eq!(dto.served_today, 4);
        assert_eq!(dto.inactive_reason, Some("schedule"));
        assert_eq!(dto.next_start_label.as_deref(), Some("demain 9:00"));
    }
}
