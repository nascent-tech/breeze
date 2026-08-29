import { test } from 'node:test';
import { ok, strictEqual, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/cycle/domain/cycle/invalid-value.error.js';
import { Suspension } from '../../../../../src/modules/cycle/domain/suspension/suspension.value-object.js';

const MINUTE = 60000;
const NOON = new Date('2026-08-29T12:00:00').getTime();

test('a suspension of a quarter of an hour ends a quarter of an hour later', () => {
  const held = Suspension.of({ term: 'quarterHour', now: NOON, frozenRemainingMilliseconds: MINUTE });

  strictEqual(held.until, NOON + 15 * MINUTE);
});

test('a suspension until tomorrow morning ends at six, not twenty-four hours later', () => {
  const held = Suspension.of({ term: 'tomorrowMorning', now: NOON, frozenRemainingMilliseconds: 0 });
  const end = new Date(held.until);

  strictEqual(end.getHours(), 6);
  strictEqual(end.getDate(), new Date(NOON).getDate() + 1);
});

test('a suspension without a term is refused: none lasts forever', () => {
  throws(() => Suspension.of({ term: 'forever', now: NOON, frozenRemainingMilliseconds: 0 }), InvalidValue);
});

test('a suspension keeps what was left of the work phase, to the millisecond', () => {
  const held = Suspension.of({ term: 'hour', now: NOON, frozenRemainingMilliseconds: 12 * MINUTE });

  strictEqual(Suspension.fromSnapshot(held.snapshot()).frozenRemainingMilliseconds, 12 * MINUTE);
});

test('a suspension holds until its term, and not one instant after', () => {
  const held = Suspension.of({ term: 'quarterHour', now: NOON, frozenRemainingMilliseconds: 0 });

  ok(held.holdsAt(NOON + 14 * MINUTE));
  ok(!held.holdsAt(NOON + 15 * MINUTE));
});
