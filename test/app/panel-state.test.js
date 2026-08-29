import { test } from 'node:test';
import { strictEqual } from 'node:assert/strict';

import { panelStateOf } from '../../src/app/panel-state.js';

const NOON = 1756900000000;
const RUNNING = {
  rhythm: { workMinutes: 50, pauseMinutes: 10 },
  severity: 'simple',
  phase: 'notice',
  endsAt: NOON + 60000,
  startedAt: NOON,
  budgetRemainingMinutes: 15,
  budgetFullMinutes: 15,
  postponesTaken: 0,
  postponeQuota: 3,
  ordinal: 1,
};

test('a postponement is offered while the break is announced', () => {
  strictEqual(panelStateOf(RUNNING, NOON, 0).postpone.offered, true);
});

test('a postponement outside the notice says which rule refuses it', () => {
  const working = panelStateOf({ ...RUNNING, phase: 'work' }, NOON, 0);

  strictEqual(working.postpone.offered, false);
  strictEqual(working.postpone.reason, 'phase');
});

test('a postponement the quota exhausted names the quota, not the budget', () => {
  const spent = panelStateOf({ ...RUNNING, postponesTaken: 3 }, NOON, 0);

  strictEqual(spent.postpone.reason, 'quota');
});

test('a postponement the budget cannot pay names the budget', () => {
  const poor = panelStateOf({ ...RUNNING, budgetRemainingMinutes: 4 }, NOON, 0);

  strictEqual(poor.postpone.reason, 'budget');
});

test('ending a break is refused when the budget does not cover what it did not serve', () => {
  const onBreak = { ...RUNNING, phase: 'break', budgetRemainingMinutes: 3 };

  strictEqual(panelStateOf(onBreak, NOON, 8).primaryLever.offered, false);
  strictEqual(panelStateOf(onBreak, NOON, 8).primaryLever.owedMinutes, 8);
});

test('what the panel counts down is never negative, even past the deadline', () => {
  strictEqual(panelStateOf(RUNNING, NOON + 120000, 0).remainingMilliseconds, 0);
});
