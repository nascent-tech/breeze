import { BreakTooYoung } from './break-too-young.error.js';
import { BudgetTooLow } from './budget-too-low.error.js';
import { CyclePhase } from './cycle-phase.value-object.js';
import { InvalidValue } from './invalid-value.error.js';
import { LeverUnavailable } from './lever-unavailable.error.js';
import { PostponeBudget } from './postpone-budget.value-object.js';
import { PostponeQuotaExhausted } from './postpone-quota-exhausted.error.js';
import { Rhythm } from './rhythm.value-object.js';
import { Severity } from './severity.value-object.js';

const MINUTE = 60000;
const NOTICE_MILLISECONDS = MINUTE;
const RETURN_MILLISECONDS = 3000;
const POSTPONE_MINUTES = 5;
const EARLIEST_END_OF_BREAK_MILLISECONDS = MINUTE;

function requiredInstant(value = Number.NaN) {
  if (!Number.isFinite(value)) {
    throw new InvalidValue('a cycle moves against a finite wall clock instant');
  }

  return value;
}

export class Cycle {
  #state;

  constructor(state) {
    this.#state = Object.freeze(state);
  }

  static start({ rhythm = Rhythm.classic(), severity = Severity.simple(), now, ordinal = 1 }) {
    const startedAt = requiredInstant(now);

    return new Cycle({
      rhythm,
      severity,
      phase: CyclePhase.work(),
      endsAt: startedAt + rhythm.workMinutes * MINUTE,
      startedAt,
      budget: PostponeBudget.full(),
      postponesTaken: 0,
      ordinal,
    });
  }

  static fromSnapshot(snapshot) {
    return new Cycle({
      rhythm: Rhythm.of(snapshot.rhythm),
      severity: Severity.fromName(snapshot.severity),
      phase: CyclePhase.fromName(snapshot.phase),
      endsAt: requiredInstant(snapshot.endsAt),
      startedAt: requiredInstant(snapshot.startedAt),
      budget: PostponeBudget.of(snapshot.budgetRemainingMinutes),
      postponesTaken: snapshot.postponesTaken,
      ordinal: snapshot.ordinal,
    });
  }

  snapshot() {
    const { rhythm, severity, phase, budget } = this.#state;

    return Object.freeze({
      rhythm: { workMinutes: rhythm.workMinutes, pauseMinutes: rhythm.pauseMinutes },
      severity: severity.name,
      phase: phase.name,
      endsAt: this.#state.endsAt,
      startedAt: this.#state.startedAt,
      budgetRemainingMinutes: budget.remainingMinutes,
      budgetFullMinutes: budget.fullMinutes,
      postponesTaken: this.#state.postponesTaken,
      postponeQuota: severity.postponeQuota,
      ordinal: this.#state.ordinal,
    });
  }

  advanceTo(now) {
    const instant = requiredInstant(now);

    return this.#elapsedAt(instant) ? this.#successorAt(instant) : this;
  }

  postpone(now) {
    const instant = requiredInstant(now);
    const { phase, postponesTaken, severity, budget } = this.#state;

    if (!phase.announcesABreak) {
      throw new LeverUnavailable('a break is postponed while it is announced, and only then');
    }

    if (postponesTaken >= severity.postponeQuota) {
      throw new PostponeQuotaExhausted('this severity offers no further postponement this cycle');
    }

    if (!budget.covers(POSTPONE_MINUTES)) {
      throw new BudgetTooLow('the postpone budget does not cover another postponement');
    }

    return this.#postponedFrom(instant);
  }

  takeBreakNow(now) {
    const instant = requiredInstant(now);

    if (this.#state.phase.isBreak || this.#state.phase.equals(CyclePhase.returning())) {
      throw new LeverUnavailable('a break that is already running does not start again');
    }

    return this.#openedBreakAt(instant);
  }

  endBreak(now) {
    const instant = requiredInstant(now);

    if (!this.#state.phase.isBreak) {
      throw new LeverUnavailable('only a running break ends early');
    }

    return this.#endedBreakAt(instant, this.owedMinutesToEndBreakAt(instant));
  }

  owedMinutesToEndBreakAt(now) {
    return Math.ceil(Math.max(0, this.#state.endsAt - requiredInstant(now)) / MINUTE);
  }

  #elapsedAt(instant) {
    const { phase, endsAt } = this.#state;

    return phase.equals(CyclePhase.work()) ? instant >= endsAt - NOTICE_MILLISECONDS : instant >= endsAt;
  }

  #successorAt(instant) {
    const { phase, rhythm, severity, ordinal, endsAt } = this.#state;
    const successors = new Map([
      ['work', () => this.#moved(CyclePhase.notice(), endsAt)],
      ['notice', () => this.#openedBreakAt(instant)],
      ['break', () => this.#moved(CyclePhase.returning(), instant + RETURN_MILLISECONDS)],
      ['return', () => Cycle.start({ rhythm, severity, now: instant, ordinal: ordinal + 1 })],
      ['inactive', () => this],
    ]);
    const successor = successors.get(phase.name) ?? (() => this);

    return successor();
  }

  #endedBreakAt(instant, owedMinutes) {
    if (instant - this.#breakStartedAt() < EARLIEST_END_OF_BREAK_MILLISECONDS) {
      throw new BreakTooYoung('a break is ended after a minute of it has been served');
    }

    if (!this.#state.budget.covers(owedMinutes)) {
      throw new BudgetTooLow('the postpone budget does not cover what is left of this break');
    }

    return new Cycle({
      ...this.#state,
      phase: CyclePhase.returning(),
      endsAt: instant + RETURN_MILLISECONDS,
      budget: this.#state.budget.debit(owedMinutes),
    });
  }

  #breakStartedAt() {
    return this.#state.endsAt - this.#state.rhythm.pauseMinutes * MINUTE;
  }

  #openedBreakAt(instant) {
    return new Cycle({
      ...this.#state,
      phase: CyclePhase.onBreak(),
      endsAt: instant + this.#state.rhythm.pauseMinutes * MINUTE,
    });
  }

  #postponedFrom(instant) {
    return new Cycle({
      ...this.#state,
      phase: CyclePhase.work(),
      endsAt: instant + POSTPONE_MINUTES * MINUTE,
      budget: this.#state.budget.debit(POSTPONE_MINUTES),
      postponesTaken: this.#state.postponesTaken + 1,
    });
  }

  #moved(phase, endsAt) {
    return new Cycle({ ...this.#state, phase, endsAt });
  }
}
