import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/cycle/domain/cycle/invalid-value.error.js';
import { Rhythm } from '../../../../../src/modules/cycle/domain/cycle/rhythm.value-object.js';

test('a rhythm refuses a work phase shorter than five minutes or longer than three hours', () => {
  throws(() => Rhythm.of({ workMinutes: 4, pauseMinutes: 1 }), InvalidValue);
  throws(() => Rhythm.of({ workMinutes: 181, pauseMinutes: 10 }), InvalidValue);
});

test('a rhythm refuses a pause shorter than a minute or longer than an hour', () => {
  throws(() => Rhythm.of({ workMinutes: 50, pauseMinutes: 0 }), InvalidValue);
  throws(() => Rhythm.of({ workMinutes: 90, pauseMinutes: 61 }), InvalidValue);
});

test('a rhythm refuses a pause longer than the work it follows', () => {
  throws(() => Rhythm.of({ workMinutes: 10, pauseMinutes: 15 }), InvalidValue);
});

test('a rhythm refuses minutes that are not whole', () => {
  throws(() => Rhythm.of({ workMinutes: 50.5, pauseMinutes: 10 }), InvalidValue);
});

test('the rhythm of someone who sets nothing is fifty minutes of work and ten of pause', () => {
  ok(Rhythm.classic().equals(Rhythm.of({ workMinutes: 50, pauseMinutes: 10 })));
});
