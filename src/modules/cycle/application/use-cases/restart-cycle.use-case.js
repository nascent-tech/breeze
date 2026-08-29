import { BudgetTooLow } from '../../domain/cycle/budget-too-low.error.js';
import { Cycle } from '../../domain/cycle/cycle.aggregate.js';
import { LeverUnavailable } from '../../domain/cycle/lever-unavailable.error.js';
import { PostponeBudget } from '../../domain/cycle/postpone-budget.value-object.js';
import { restartPriceMinutesOf } from '../../domain/cycle/restart-price.js';
import { Rhythm } from '../../domain/cycle/rhythm.value-object.js';
import { Severity } from '../../domain/cycle/severity.value-object.js';

export class RestartCycle {
  #store;
  #clock;

  constructor(store, clock) {
    this.#store = store;
    this.#clock = clock;
  }

  execute() {
    const kept = this.#store.read();
    const now = this.#clock.nowInMilliseconds();

    if (kept === null || kept.phase !== 'work') {
      throw new LeverUnavailable('a cycle restarts from its work phase, and from nothing else');
    }

    return this.#restartedFrom(kept, now, restartPriceMinutesOf(kept, now));
  }

  #restartedFrom(kept, now, priceMinutes) {
    const budget = PostponeBudget.of(kept.budgetRemainingMinutes);

    if (!budget.covers(priceMinutes)) {
      throw new BudgetTooLow('the postpone budget does not cover the work minutes a restart erases');
    }

    const restarted = Cycle.start({
      rhythm: Rhythm.of(kept.rhythm),
      severity: Severity.fromName(kept.severity),
      now,
      ordinal: kept.ordinal,
      budget: budget.debit(priceMinutes),
    }).snapshot();

    this.#store.write(restarted);

    return restarted;
  }
}
