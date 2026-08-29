import { test } from 'node:test';
import { strictEqual, throws } from 'node:assert/strict';

import { BudgetTooLow } from '../../../../../src/modules/cycle/domain/cycle/budget-too-low.error.js';
import { LeverUnavailable } from '../../../../../src/modules/cycle/domain/cycle/lever-unavailable.error.js';
import { RestartCycle } from '../../../../../src/modules/cycle/application/use-cases/restart-cycle.use-case.js';

const MINUTE = 60000;
const NOON = 1756900000000;
const RUNNING = {
  rhythm: { workMinutes: 50, pauseMinutes: 10 },
  severity: 'simple',
  phase: 'work',
  endsAt: NOON + 50 * MINUTE,
  startedAt: NOON,
  budgetRemainingMinutes: 15,
  budgetFullMinutes: 15,
  postponesTaken: 0,
  postponeQuota: 3,
  ordinal: 3,
  breakServed: false,
};

function restartAt(now, kept = RUNNING) {
  const held = { snapshot: kept };
  const store = {
    read: () => held.snapshot,
    write: (written) => {
      held.snapshot = written;
    },
  };

  new RestartCycle(store, { nowInMilliseconds: () => now }).execute();

  return held.snapshot;
}

test('a restart erases the work already done, and pays for it out of the budget', () => {
  const restarted = restartAt(NOON + 7 * MINUTE);

  strictEqual(restarted.budgetRemainingMinutes, 8);
  strictEqual(restarted.endsAt, NOON + 7 * MINUTE + 50 * MINUTE);
});

test('a restart the budget cannot pay is refused, which closes the endless loop', () => {
  throws(() => restartAt(NOON + 16 * MINUTE), BudgetTooLow);
});

test('a restart keeps the cycle it restarts, rather than opening the next one', () => {
  strictEqual(restartAt(NOON + MINUTE).ordinal, 3);
});

test('a break already running is not restarted', () => {
  throws(() => restartAt(NOON, { ...RUNNING, phase: 'break' }), LeverUnavailable);
});
