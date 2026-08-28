import { test } from 'node:test';
import { ok, strictEqual, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/cycle/domain/cycle/invalid-value.error.js';
import { PostponeBudget } from '../../../../../src/modules/cycle/domain/cycle/postpone-budget.value-object.js';

test('a cycle opens with fifteen minutes to spend on postponing its break', () => {
  strictEqual(PostponeBudget.full().remainingMinutes, 15);
});

test('a debit larger than what is left empties the budget without owing anything', () => {
  strictEqual(PostponeBudget.of(3).debit(8).remainingMinutes, 0);
});

test('a budget refuses a lever it cannot pay in full', () => {
  ok(!PostponeBudget.of(4).covers(5));
  ok(PostponeBudget.of(5).covers(5));
});

test('a budget refuses to hold a negative amount', () => {
  throws(() => PostponeBudget.of(-1), InvalidValue);
});
