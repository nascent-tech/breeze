import { test } from 'node:test';
import { ok, strictEqual } from 'node:assert/strict';

import { ScreenSharingLevel } from '../../../../src/modules/system-bridge/application/model/screen-sharing-level.value-object.js';

export function runScreenSharingContract(makeSubject) {
  test('screen sharing reads a level, never an event', () => {
    const { port } = makeSubject();

    ok(port.currentLevel() instanceof ScreenSharingLevel);
  });

  test('screen sharing tells a level it cannot read apart from no sharing at all', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setLevel')) {
      return t.skip('the subject reads the session of the system, which no test shares');
    }

    driver.setLevel(ScreenSharingLevel.indistinguishable());

    strictEqual(port.currentLevel().equals(ScreenSharingLevel.none()), false);
  });

  test('screen sharing reads the level the session is in', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setLevel')) {
      return t.skip('the subject reads the session of the system, which no test shares');
    }

    driver.setLevel(ScreenSharingLevel.presenting());

    ok(port.currentLevel().equals(ScreenSharingLevel.presenting()));
  });
}
