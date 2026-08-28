import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { UnknownWindow } from '../../../../src/modules/system-bridge/application/errors/unknown-window.error.js';
import { WindowLevel } from '../../../../src/modules/system-bridge/application/model/window-level.value-object.js';

const WINDOW_ID = 12;

export function runWindowLevelContract(makeSubject) {
  test('a window level is read back from the window it was applied to', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('openWindow')) {
      return t.skip('the subject stacks real windows, which no test opens');
    }

    driver.openWindow(WINDOW_ID);
    port.apply(WINDOW_ID, WindowLevel.maximum());

    ok(port.levelOf(WINDOW_ID).equals(WindowLevel.maximum()));
  });

  test('applying a level to a window the system no longer knows is refused', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('destroyWindow')) {
      return t.skip('the subject stacks real windows, which no test destroys');
    }

    driver.openWindow(WINDOW_ID);
    driver.destroyWindow(WINDOW_ID);

    throws(() => port.apply(WINDOW_ID, WindowLevel.maximum()), UnknownWindow);
  });

  test('reading the level of a window the system no longer knows is refused', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('destroyWindow')) {
      return t.skip('the subject stacks real windows, which no test destroys');
    }

    driver.openWindow(WINDOW_ID);
    driver.destroyWindow(WINDOW_ID);

    throws(() => port.levelOf(WINDOW_ID), UnknownWindow);
  });
}
