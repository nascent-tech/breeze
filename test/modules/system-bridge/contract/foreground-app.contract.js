import { test } from 'node:test';
import { deepStrictEqual, ok, strictEqual } from 'node:assert/strict';

import { AppIdentity } from '../../../../src/modules/system-bridge/application/model/app-identity.value-object.js';

const OTHER_APP = AppIdentity.of({ bundleIdentifier: 'com.apple.finder', displayName: 'Finder' });

export function runForegroundAppContract(makeSubject) {
  test('a foreground application is named by the identity the brief allows, and nothing more', () => {
    const { port } = makeSubject();

    const app = port.currentApp();

    ok(app instanceof AppIdentity);
    ok(app.bundleIdentifier.length > 0);
    ok(app.displayName.length > 0);
  });

  test('a foreground application notifies the change that brings another one to the front', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitChange')) {
      return t.skip('the subject follows the human, who brings no application to the front in a test');
    }

    const seen = [];
    port.onChange((app) => seen.push(app.bundleIdentifier));
    driver.emitChange(OTHER_APP);

    deepStrictEqual(seen, [OTHER_APP.bundleIdentifier]);
  });

  test('a foreground application that stays in front notifies nothing', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitChange')) {
      return t.skip('the subject follows the human, who brings no application to the front in a test');
    }

    let notifications = 0;
    port.onChange(() => (notifications += 1));
    driver.emitChange(port.currentApp());

    strictEqual(notifications, 0);
  });

  test('a foreground application notifies no listener that unsubscribed', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('emitChange')) {
      return t.skip('the subject follows the human, who brings no application to the front in a test');
    }

    let notifications = 0;
    const unsubscribe = port.onChange(() => (notifications += 1));
    unsubscribe();
    unsubscribe();
    driver.emitChange(OTHER_APP);

    strictEqual(notifications, 0);
  });

  test('a foreground application port lists what the session is running', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setRunningApps')) {
      return t.skip('the subject lists what the system runs, which no test sets');
    }

    driver.setRunningApps([OTHER_APP]);

    deepStrictEqual(port.runningApps().map((app) => app.bundleIdentifier), [OTHER_APP.bundleIdentifier]);
  });
}
