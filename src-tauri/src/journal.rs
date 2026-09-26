use crate::local_time::{iso_date, local_now};
use crate::{lock, AppState};
use breeze_domain::BreakOutcome;
use breeze_ports::{LedgerEntry, PersistenceError};
use std::sync::PoisonError;
use std::time::{Duration, Instant};

// Après un échec d'écriture, on ne réessaie pas à chaque tick (4 par seconde) : un
// disque plein ne doit pas noyer le journal système.
const RETRY_AFTER: Duration = Duration::from_secs(30);

// File des sorts pas encore écrits au journal. Un sort prélevé au scheduler n'existe
// plus qu'ici : en cas d'échec il reste en file, jamais perdu tant que l'app vit.
#[derive(Default)]
pub struct Journal {
    pending: Vec<LedgerEntry>,
    retry_at: Option<Instant>,
}

impl Journal {
    pub fn enqueue(&mut self, entries: impl IntoIterator<Item = LedgerEntry>) {
        self.pending.extend(entries);
    }

    #[cfg(test)]
    fn pending(&self) -> usize {
        self.pending.len()
    }

    // Écrit dans l'ordre et s'arrête au premier échec : la suite reste en file, pour ne
    // jamais écrire un sort après un plus ancien resté en rade.
    pub fn flush<F>(&mut self, now: Instant, force: bool, mut record: F)
    where
        F: FnMut(&LedgerEntry) -> Result<(), PersistenceError>,
    {
        let waiting = self.retry_at.is_some_and(|at| now < at);
        if self.pending.is_empty() || (waiting && !force) {
            return;
        }
        let mut written = 0;
        for entry in &self.pending {
            if let Err(error) = record(entry) {
                eprintln!("breeze: could not journal a break outcome: {}", error.0);
                break;
            }
            written += 1;
        }
        self.pending.drain(..written);
        self.retry_at = (!self.pending.is_empty()).then(|| now + RETRY_AFTER);
    }
}

impl AppState {
    // Prélève les sorts apparus depuis le dernier appel et les journalise. Le verrou du
    // scheduler n'est tenu que pour prélever, jamais pendant l'écriture disque.
    pub(crate) fn record_outcomes(&self) -> usize {
        let outcomes = lock(&self.scheduler).take_new_outcomes();
        self.journal_outcomes(outcomes)
    }

    pub(crate) fn journal_outcomes(&self, outcomes: Vec<BreakOutcome>) -> usize {
        let fresh = outcomes.len();
        let ended_at_unix = self.clock.wall().as_unix_secs();
        let local_date = iso_date(local_now().date);
        let mut journal = self.journal.lock().unwrap_or_else(PoisonError::into_inner);
        journal.enqueue(outcomes.into_iter().map(|outcome| LedgerEntry {
            ended_at_unix,
            local_date: local_date.clone(),
            outcome,
        }));
        journal.flush(Instant::now(), fresh > 0, |entry| {
            self.persistence.record_break(entry)
        });
        fresh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(ended_at_unix: u64) -> LedgerEntry {
        LedgerEntry {
            ended_at_unix,
            local_date: "2026-09-26".to_owned(),
            outcome: BreakOutcome::ValidatedByAbsence,
        }
    }

    fn disk_full(_: &LedgerEntry) -> Result<(), PersistenceError> {
        Err(PersistenceError("disk full".to_owned()))
    }

    #[test]
    fn a_failed_write_keeps_the_outcome_queued_and_a_later_write_drains_it() {
        let mut journal = Journal::default();
        let start = Instant::now();
        journal.enqueue([entry(1), entry(2)]);

        journal.flush(start, true, disk_full);
        assert_eq!(journal.pending(), 2);

        let mut written = Vec::new();
        journal.flush(start + RETRY_AFTER, false, |entry| {
            written.push(entry.ended_at_unix);
            Ok(())
        });
        assert_eq!(journal.pending(), 0);
        assert_eq!(written, vec![1, 2]);
    }

    #[test]
    fn after_a_failure_the_ticker_waits_before_retrying() {
        let mut journal = Journal::default();
        let start = Instant::now();
        journal.enqueue([entry(1)]);
        journal.flush(start, true, disk_full);

        let mut attempts = 0;
        journal.flush(start + Duration::from_secs(1), false, |_| {
            attempts += 1;
            Ok(())
        });

        assert_eq!(attempts, 0);
        assert_eq!(journal.pending(), 1);
    }

    #[test]
    fn a_failure_midway_keeps_the_rest_in_order() {
        let mut journal = Journal::default();
        journal.enqueue([entry(1), entry(2), entry(3)]);
        let mut calls = 0;
        journal.flush(Instant::now(), true, |_| {
            calls += 1;
            if calls == 2 {
                Err(PersistenceError("busy".to_owned()))
            } else {
                Ok(())
            }
        });

        assert_eq!(journal.pending(), 2);
        let mut left = Vec::new();
        journal.flush(Instant::now(), true, |entry| {
            left.push(entry.ended_at_unix);
            Ok(())
        });
        assert_eq!(left, vec![2, 3]);
    }
}
