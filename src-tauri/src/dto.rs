use breeze_app::{CyclePhase, CycleSnapshot};
use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{Instant, Rhythm, Severity};
use core::time::Duration;
use serde::Serialize;

#[derive(Serialize)]
pub struct SnapshotDto {
    pub phase: String,
    pub remaining_secs: u64,
    pub total_secs: u64,
    pub break_in_secs: u64,
    pub severity: String,
    pub served_breaks: u32,
    pub work_minutes: u16,
    pub pause_minutes: u16,
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

fn severity_name(severity: Severity) -> &'static str {
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

pub fn to_dto(
    snapshot: CycleSnapshot,
    now: Instant,
    active: &Rhythm,
    configured: &Rhythm,
) -> SnapshotDto {
    let remaining = snapshot
        .deadline
        .map_or(0, |deadline| deadline.elapsed_since(now).as_secs());
    SnapshotDto {
        phase: phase_name(snapshot.phase).to_owned(),
        remaining_secs: remaining,
        total_secs: phase_total(snapshot.phase, active).as_secs(),
        break_in_secs: seconds_until_break(snapshot.phase, remaining),
        severity: severity_name(snapshot.severity).to_owned(),
        served_breaks: snapshot.served_breaks,
        work_minutes: configured.work().count(),
        pause_minutes: configured.pause().count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use breeze_domain::{ActiveDays, Cycle, Minutes};

    fn rhythm() -> Rhythm {
        Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
    }

    #[test]
    fn a_fresh_working_cycle_reports_break_after_work_plus_notice() {
        let r = rhythm();
        let snapshot = CycleSnapshot::of(&Cycle::start(r, Severity::Simple, Instant::EPOCH));
        let dto = to_dto(snapshot, Instant::EPOCH, &r, &r);
        assert_eq!(dto.phase, "Working");
        assert_eq!(dto.total_secs, r.work().as_duration().as_secs());
        assert_eq!(
            dto.break_in_secs,
            r.work().as_duration().as_secs() + NOTICE.as_secs()
        );
        assert_eq!(dto.severity, "Simple");
        assert_eq!(dto.work_minutes, 50);
        assert_eq!(dto.pause_minutes, 10);
    }

    #[test]
    fn the_reported_severity_is_the_configured_one_even_while_working() {
        let r = rhythm();
        let snapshot = CycleSnapshot::of(&Cycle::start(r, Severity::Hardcore, Instant::EPOCH));
        let dto = to_dto(snapshot, Instant::EPOCH, &r, &r);
        assert_eq!(dto.severity, "Hardcore");
    }

    #[test]
    fn the_ring_follows_the_active_rhythm_while_settings_show_the_configured_one() {
        let active = rhythm();
        let configured =
            Rhythm::new(Minutes(25), Minutes(5), None, ActiveDays::everyday()).unwrap();
        let snapshot = CycleSnapshot::of(&Cycle::start(active, Severity::Simple, Instant::EPOCH));
        let dto = to_dto(snapshot, Instant::EPOCH, &active, &configured);
        assert_eq!(dto.total_secs, active.work().as_duration().as_secs());
        assert_eq!(dto.work_minutes, 25);
        assert_eq!(dto.pause_minutes, 5);
    }
}
