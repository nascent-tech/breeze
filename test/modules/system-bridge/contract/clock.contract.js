import { test } from 'node:test';
import { ok, strictEqual } from 'node:assert/strict';

import { Instant } from '../../../../src/modules/system-bridge/application/model/instant.value-object.js';

export function runClockContract(makeSubject) {
  test('a clock reads monotonic instants that never go backwards', () => {
    const { port } = makeSubject();

    const first = port.monotonicNow();
    const second = port.monotonicNow();

    ok(second.millisecondsSince(first) >= 0);
  });

  test('a clock carries the monotonic reading forward as time passes', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('advance')) {
      return t.skip('the subject reads the clock of the system, which no test advances');
    }

    const before = port.monotonicNow();
    driver.advance(5);

    strictEqual(port.monotonicNow().millisecondsSince(before), 5);
  });

  test('a clock reads a wall clock instant the test drives', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setWallClock')) {
      return t.skip('the subject reads the clock of the system, which no test sets');
    }

    driver.setWallClock(1000);

    strictEqual(port.wallClockNow().millisecondsSince(Instant.wallClock(0)), 1000);
  });

  test('a clock keeps its two scales apart', () => {
    const { port } = makeSubject();

    ok(port.monotonicNow().isMonotonic);
    ok(!port.wallClockNow().isMonotonic);
  });
}
