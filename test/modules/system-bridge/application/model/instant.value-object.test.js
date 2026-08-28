import { test } from 'node:test';
import { strictEqual, throws } from 'node:assert/strict';

import { Instant } from '../../../../../src/modules/system-bridge/application/model/instant.value-object.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';

const NANOSECONDS_PER_MILLISECOND = 1000000n;

test('an instant refuses to measure a monotonic reading against a wall clock one', () => {
  throws(() => Instant.monotonic(0n).millisecondsSince(Instant.wallClock(0)), InvalidValue);
});

test('a monotonic instant refuses a number, which loses nanoseconds', () => {
  throws(() => Instant.monotonic(1000), InvalidValue);
});

test('a wall clock instant refuses a number of milliseconds that is not finite', () => {
  throws(() => Instant.wallClock(Number.POSITIVE_INFINITY), InvalidValue);
  throws(() => Instant.wallClock(undefined), InvalidValue);
});

test('an instant refuses a scale that neither the system nor the human reads', () => {
  throws(() => new Instant('bogus', 42), InvalidValue);
});

test('a duration is the difference of two instants of the same scale', () => {
  const later = Instant.monotonic(5n * NANOSECONDS_PER_MILLISECOND);

  strictEqual(later.millisecondsSince(Instant.monotonic(0n)), 5);
  strictEqual(Instant.wallClock(1005).millisecondsSince(Instant.wallClock(1000)), 5);
});
