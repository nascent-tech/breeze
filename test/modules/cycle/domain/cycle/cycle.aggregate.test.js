import { test } from 'node:test';
import { ok, strictEqual, throws } from 'node:assert/strict';

import { BreakTooYoung } from '../../../../../src/modules/cycle/domain/cycle/break-too-young.error.js';
import { BudgetTooLow } from '../../../../../src/modules/cycle/domain/cycle/budget-too-low.error.js';
import { Cycle } from '../../../../../src/modules/cycle/domain/cycle/cycle.aggregate.js';
import { LeverUnavailable } from '../../../../../src/modules/cycle/domain/cycle/lever-unavailable.error.js';
import { PostponeQuotaExhausted } from '../../../../../src/modules/cycle/domain/cycle/postpone-quota-exhausted.error.js';
import { Rhythm } from '../../../../../src/modules/cycle/domain/cycle/rhythm.value-object.js';
import { Severity } from '../../../../../src/modules/cycle/domain/cycle/severity.value-object.js';

const MINUTE = 60000;
const NOON = 1756900000000;

function startedAtNoon(severity = Severity.simple()) {
  return Cycle.start({ rhythm: Rhythm.classic(), severity, now: NOON });
}

function announcedAtNoon(severity = Severity.simple()) {
  return startedAtNoon(severity).advanceTo(NOON + 49 * MINUTE);
}

test('a cycle opens on a full work phase and a full postpone budget', () => {
  const snapshot = startedAtNoon().snapshot();

  strictEqual(snapshot.phase, 'work');
  strictEqual(snapshot.endsAt, NOON + 50 * MINUTE);
  strictEqual(snapshot.budgetRemainingMinutes, 15);
});

test('a break is announced a minute before it falls due', () => {
  strictEqual(startedAtNoon().advanceTo(NOON + 48 * MINUTE).snapshot().phase, 'work');
  strictEqual(announcedAtNoon().snapshot().phase, 'notice');
});

test('the notice does not push the break it announces', () => {
  strictEqual(announcedAtNoon().snapshot().endsAt, NOON + 50 * MINUTE);
});

test('the break falls due when the notice runs out, and lasts the pause of the rhythm', () => {
  const onBreak = announcedAtNoon().advanceTo(NOON + 50 * MINUTE);

  strictEqual(onBreak.snapshot().phase, 'break');
  strictEqual(onBreak.snapshot().endsAt, NOON + 60 * MINUTE);
});

test('a postponement costs five minutes of budget and returns to work', () => {
  const postponed = announcedAtNoon().postpone(NOON + 49 * MINUTE);

  strictEqual(postponed.snapshot().phase, 'work');
  strictEqual(postponed.snapshot().budgetRemainingMinutes, 10);
  strictEqual(postponed.snapshot().endsAt, NOON + 54 * MINUTE);
});

test('a break is postponed while it is announced, and never during the work phase', () => {
  throws(() => startedAtNoon().postpone(NOON + MINUTE), LeverUnavailable);
});

test('three postponements exhaust exactly the budget of a simple cycle', () => {
  let cycle = announcedAtNoon();

  for (let taken = 0; taken < 3; taken += 1) {
    cycle = cycle.postpone(NOON + 49 * MINUTE).advanceTo(NOON + 53 * MINUTE + taken * MINUTE);
    cycle = Cycle.fromSnapshot({ ...cycle.snapshot(), phase: 'notice' });
  }

  strictEqual(cycle.snapshot().budgetRemainingMinutes, 0);
  throws(() => cycle.postpone(NOON + 60 * MINUTE), PostponeQuotaExhausted);
});

test('the hardcore mode offers a single postponement, whatever the budget still holds', () => {
  const postponed = announcedAtNoon(Severity.hardcore()).postpone(NOON + 49 * MINUTE);
  const announcedAgain = Cycle.fromSnapshot({ ...postponed.snapshot(), phase: 'notice' });

  strictEqual(announcedAgain.snapshot().budgetRemainingMinutes, 10);
  throws(() => announcedAgain.postpone(NOON + 53 * MINUTE), PostponeQuotaExhausted);
});

test('a break taken now starts without waiting for the notice', () => {
  const onBreak = startedAtNoon().takeBreakNow(NOON + 10 * MINUTE);

  strictEqual(onBreak.snapshot().phase, 'break');
  strictEqual(onBreak.snapshot().endsAt, NOON + 20 * MINUTE);
});

test('a break that is already running does not start again', () => {
  throws(() => startedAtNoon().takeBreakNow(NOON).takeBreakNow(NOON), LeverUnavailable);
});

test('a break is not ended before a minute of it has been served', () => {
  const onBreak = startedAtNoon().takeBreakNow(NOON);

  throws(() => onBreak.endBreak(NOON + 30000), BreakTooYoung);
});

test('ending a break early debits the minutes it did not serve', () => {
  const ended = startedAtNoon().takeBreakNow(NOON).endBreak(NOON + 2 * MINUTE);

  strictEqual(ended.snapshot().phase, 'return');
  strictEqual(ended.snapshot().budgetRemainingMinutes, 7);
});

test('a break is not ended when the budget does not cover what is left of it', () => {
  const poor = Cycle.fromSnapshot({ ...startedAtNoon().takeBreakNow(NOON).snapshot(), budgetRemainingMinutes: 3 });

  throws(() => poor.endBreak(NOON + 2 * MINUTE), BudgetTooLow);
});

test('only a running break ends early', () => {
  throws(() => startedAtNoon().endBreak(NOON + MINUTE), LeverUnavailable);
});

test('the return lasts three seconds and opens the next cycle, budget full again', () => {
  const returning = startedAtNoon().takeBreakNow(NOON).advanceTo(NOON + 10 * MINUTE);
  const next = returning.advanceTo(NOON + 10 * MINUTE + 3000);

  strictEqual(returning.snapshot().phase, 'return');
  strictEqual(next.snapshot().phase, 'work');
  strictEqual(next.snapshot().ordinal, 2);
  strictEqual(next.snapshot().budgetRemainingMinutes, 15);
});

test('a cycle read back from its snapshot is the cycle that was written', () => {
  const snapshot = announcedAtNoon().snapshot();

  ok(Cycle.fromSnapshot(snapshot).snapshot().endsAt === snapshot.endsAt);
  strictEqual(Cycle.fromSnapshot(snapshot).snapshot().phase, 'notice');
});

test('the emergency exit ends a break the budget could never have paid for', () => {
  const poor = Cycle.fromSnapshot({ ...startedAtNoon().takeBreakNow(NOON).snapshot(), budgetRemainingMinutes: 2 });
  const escaped = poor.escapeBreak(NOON + 30000);

  strictEqual(escaped.snapshot().phase, 'return');
  strictEqual(escaped.snapshot().budgetRemainingMinutes, 0);
});

test('the emergency exit is offered from a break, and from nothing else', () => {
  throws(() => startedAtNoon().escapeBreak(NOON), LeverUnavailable);
});

test('an escaped break never leaves the next cycle in debt', () => {
  const escaped = startedAtNoon().takeBreakNow(NOON).escapeBreak(NOON);
  const next = escaped.advanceTo(NOON + 3000);

  ok(next.snapshot().budgetRemainingMinutes >= 0);
  strictEqual(next.snapshot().budgetRemainingMinutes, 15);
});
