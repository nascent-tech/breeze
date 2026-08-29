import { CyclePhase } from './cycle-phase.value-object.js';
import { InvalidValue } from './invalid-value.error.js';
import { PostponeBudget } from './postpone-budget.value-object.js';
import { Rhythm } from './rhythm.value-object.js';
import { Severity } from './severity.value-object.js';

export function requiredInstant(value = Number.NaN) {
  if (Number.isFinite(value)) {
    return value;
  }

  throw new InvalidValue('a cycle moves against a finite wall clock instant');
}

export function stateFrom(snapshot) {
  return {
    rhythm: Rhythm.of(snapshot.rhythm),
    severity: Severity.fromName(snapshot.severity),
    phase: CyclePhase.fromName(snapshot.phase),
    endsAt: requiredInstant(snapshot.endsAt),
    startedAt: requiredInstant(snapshot.startedAt),
    budget: PostponeBudget.of(snapshot.budgetRemainingMinutes),
    postponesTaken: snapshot.postponesTaken,
    ordinal: snapshot.ordinal,
    breakServed: snapshot.breakServed === true,
  };
}

export function snapshotOf(state) {
  return Object.freeze({
    rhythm: { workMinutes: state.rhythm.workMinutes, pauseMinutes: state.rhythm.pauseMinutes },
    severity: state.severity.name,
    phase: state.phase.name,
    endsAt: state.endsAt,
    startedAt: state.startedAt,
    budgetRemainingMinutes: state.budget.remainingMinutes,
    budgetFullMinutes: state.budget.fullMinutes,
    postponesTaken: state.postponesTaken,
    postponeQuota: state.severity.postponeQuota,
    ordinal: state.ordinal,
    breakServed: state.breakServed === true,
  });
}
