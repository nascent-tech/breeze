import { BreakTooYoung } from './break-too-young.error.js';
import { owedMinutesToEndBreakAt } from './owed-minutes.js';
import { BudgetTooLow } from './budget-too-low.error.js';
import { CyclePhase } from './cycle-phase.value-object.js';
import { requiredInstant, snapshotOf, stateFrom } from './cycle-snapshot.js';
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
export class Cycle {
  #state;

  constructor(state) {
    this.#state = Object.freeze(state);
  }

  static start(opening) {
    const { rhythm = Rhythm.classic(), severity = Severity.simple() } = opening;
    const { ordinal = 1, budget = PostponeBudget.full() } = opening;
    const startedAt = requiredInstant(opening.now);

    return new Cycle({
      rhythm,
      severity,
      phase: CyclePhase.work(),
      endsAt: startedAt + rhythm.workMinutes * MINUTE,
      startedAt,
      budget,
      postponesTaken: 0,
      ordinal,
      breakServed: false,
    });
  }

  static fromSnapshot(snapshot) {
    return new Cycle(stateFrom(snapshot));
  }

  snapshot() {
    return snapshotOf(this.#state);
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

    if (!this.#state.severity.equals(Severity.simple())) {
      throw new LeverUnavailable('the hardcore mode offers no button to end a break');
    }

    return this.#endedBreakAt(instant, owedMinutesToEndBreakAt(this.snapshot(), instant));
  }

  escapeBreak(now) {
    const instant = requiredInstant(now);

    if (!this.#state.phase.isBreak) {
      throw new LeverUnavailable('only a running break is escaped');
    }

    return new Cycle({
      ...this.#state,
      phase: CyclePhase.returning(),
      endsAt: instant + RETURN_MILLISECONDS,
      budget: this.#state.budget.debit(owedMinutesToEndBreakAt(this.snapshot(), instant)),
    });
  }

  #elapsedAt(instant) {
    const { phase, endsAt } = this.#state;

    return phase.equals(CyclePhase.work()) ? instant >= endsAt - NOTICE_MILLISECONDS : instant >= endsAt;
  }

  #successorAt(instant) {
    const { phase, endsAt } = this.#state;
    const successors = new Map([
      ['work', () => this.#moved(CyclePhase.notice(), endsAt)],
      ['notice', () => this.#openedBreakAt(instant)],
      ['break', () => this.#servedBreakAt(instant)],
      ['return', () => this.#nextCycleFrom(instant)],
      ['inactive', () => this],
    ]);
    const successor = successors.get(phase.name) ?? (() => this);

    return successor();
  }

  #servedBreakAt(instant) {
    return new Cycle({
      ...this.#state,
      phase: CyclePhase.returning(),
      endsAt: instant + RETURN_MILLISECONDS,
      breakServed: true,
    });
  }

  #nextCycleFrom(instant) {
    const { rhythm, severity, ordinal, budget, breakServed } = this.#state;

    return Cycle.start({
      rhythm,
      severity,
      now: instant,
      ordinal: ordinal + 1,
      budget: breakServed ? PostponeBudget.full() : budget,
    });
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
