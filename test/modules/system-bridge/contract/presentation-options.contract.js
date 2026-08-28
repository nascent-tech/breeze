import { test } from 'node:test';
import { deepStrictEqual, ok } from 'node:assert/strict';

export function runPresentationOptionsContract(makeSubject) {
  test('presentation options restore everything they hid', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('appliedOptions')) {
      return t.skip('the subject hides the real Dock, whose state no test reads');
    }

    port.hideDockAndMenuBar();
    port.disableApplicationSwitching();
    ok(driver.appliedOptions().length > 0);

    port.restore();
    deepStrictEqual(driver.appliedOptions(), []);
  });

  test('a restore without a previous change does nothing, on every exit path', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('appliedOptions')) {
      return t.skip('the subject hides the real Dock, whose state no test reads');
    }

    port.restore();

    deepStrictEqual(driver.appliedOptions(), []);
  });

  test('an abnormal exit leaves no presentation option applied', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('simulateAbnormalExit')) {
      return t.skip('a real process cannot be killed from inside its own test');
    }

    port.hideDockAndMenuBar();
    driver.simulateAbnormalExit();

    deepStrictEqual(driver.appliedOptions(), []);
  });
}
