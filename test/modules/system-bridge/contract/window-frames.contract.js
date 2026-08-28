import { test } from 'node:test';
import { deepStrictEqual, ok, strictEqual, throws } from 'node:assert/strict';

import { AccessibilityDenied } from '../../../../src/modules/system-bridge/application/errors/accessibility-denied.error.js';
import { Bounds } from '../../../../src/modules/system-bridge/application/model/bounds.value-object.js';
import { WindowFrame } from '../../../../src/modules/system-bridge/application/model/window-frame.value-object.js';

const PROCESS_ID = 4242;
const FRAME = WindowFrame.of({
  windowId: 7,
  processId: PROCESS_ID,
  bounds: Bounds.of({ x: 0, y: 0, width: 800, height: 600 }),
});

export function runWindowFramesContract(makeSubject) {
  test('a process without a window has no frame, and that is not a refusal', () => {
    const { port } = makeSubject();

    deepStrictEqual(port.framesOf(PROCESS_ID), []);
  });

  test('window frames notify a move and a resize apart from one another', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitMove')) {
      return t.skip('the subject follows real windows, which no test moves');
    }

    const moved = [];
    const resized = [];
    port.onMove((frame) => moved.push(frame));
    port.onResize((frame) => resized.push(frame));
    driver.emitMove(FRAME);

    deepStrictEqual([moved.length, resized.length], [1, 0]);
  });

  test('window frames notify a resize without calling it a move', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitResize')) {
      return t.skip('the subject follows real windows, which no test resizes');
    }

    const moved = [];
    const resized = [];
    port.onMove((frame) => moved.push(frame));
    port.onResize((frame) => resized.push(frame));
    driver.emitResize(FRAME);

    deepStrictEqual([moved.length, resized.length], [0, 1]);
  });

  test('window frames fall back to polling after a silence, and stop once notifications resume', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('simulateSilence')) {
      return t.skip('the subject depends on a silence threshold the reconnaissance has yet to measure');
    }

    driver.simulateSilence(1);
    ok(port.pollingActive());

    driver.emitMove(FRAME);
    ok(!port.pollingActive());
  });

  test('window frames refuse every reading without the accessibility permission', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('denyAccessibility')) {
      return t.skip('the subject reads the permission of the system, which no test revokes');
    }

    driver.denyAccessibility();

    throws(() => port.framesOf(PROCESS_ID), AccessibilityDenied);
  });

  test('window frames poll no longer than the silence that started it', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('simulateSilence')) {
      return t.skip('the subject depends on a silence threshold the reconnaissance has yet to measure');
    }

    driver.simulateSilence(0);

    strictEqual(port.pollingActive(), false);
  });
}
