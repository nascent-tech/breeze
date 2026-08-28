import { test } from 'node:test';
import { deepStrictEqual, ok, throws } from 'node:assert/strict';

import { Bounds } from '../../../../src/modules/system-bridge/application/model/bounds.value-object.js';
import { Display } from '../../../../src/modules/system-bridge/application/model/display.value-object.js';
import { DuplicateDisplay } from '../../../../src/modules/system-bridge/application/errors/duplicate-display.error.js';

const BOUNDS = Bounds.of({ x: 0, y: 0, width: 1920, height: 1080 });
const MAIN_DISPLAY = Display.of({ displayId: 1, bounds: BOUNDS, carriesMenuBar: true });
const SECOND_DISPLAY = Display.of({ displayId: 2, bounds: BOUNDS, carriesMenuBar: false });

export function runDisplaysContract(makeSubject) {
  test('a session always runs on at least one display', () => {
    const { port, driver } = makeSubject();

    if (driver.supports('setDisplays')) {
      driver.setDisplays([MAIN_DISPLAY]);
    }

    ok(port.list().length >= 1);
  });

  test('displays notify a screen that is plugged in', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitConnected')) {
      return t.skip('a real screen cannot be plugged in from a test');
    }

    const connected = [];
    port.onConnected((display) => connected.push(display.displayId));
    driver.emitConnected(SECOND_DISPLAY);

    deepStrictEqual(connected, [SECOND_DISPLAY.displayId]);
  });

  test('displays notify the identifier of a screen that is unplugged', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitDisconnected')) {
      return t.skip('a real screen cannot be unplugged from a test');
    }

    driver.setDisplays([MAIN_DISPLAY, SECOND_DISPLAY]);
    const disconnected = [];
    port.onDisconnected((displayId) => disconnected.push(displayId));
    driver.emitDisconnected(SECOND_DISPLAY.displayId);

    deepStrictEqual(disconnected, [SECOND_DISPLAY.displayId]);
  });

  test('a display list refuses two screens that share one identifier', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setDisplays')) {
      return t.skip('the subject lists the real screens, which no test duplicates');
    }

    driver.setDisplays([MAIN_DISPLAY, MAIN_DISPLAY]);

    throws(() => port.list(), DuplicateDisplay);
  });
}
