import { test } from 'node:test';
import { deepStrictEqual, strictEqual, throws } from 'node:assert/strict';

import { ReservedShortcut } from '../../../../src/modules/system-bridge/application/errors/reserved-shortcut.error.js';
import { Shortcut } from '../../../../src/modules/system-bridge/application/model/shortcut.value-object.js';
import { ShortcutAlreadyRegistered } from '../../../../src/modules/system-bridge/application/errors/shortcut-already-registered.error.js';

const POSTPONE = Shortcut.of({ identity: 'postpone', keyEquivalent: 'p', modifiers: ['cmd', 'shift'] });
const LOCK_SESSION = Shortcut.of({ identity: 'lock', keyEquivalent: 'q', modifiers: ['cmd', 'ctrl'] });

export function runGlobalShortcutsContract(makeSubject) {
  test('a global shortcut registry refuses an identity it already holds', () => {
    const { port } = makeSubject();

    port.register(POSTPONE, () => {});

    throws(() => port.register(POSTPONE, () => {}), ShortcutAlreadyRegistered);
  });

  test('a global shortcut refuses a combination the system reserves', () => {
    const { port } = makeSubject();

    throws(() => port.register(LOCK_SESSION, () => {}), ReservedShortcut);
  });

  test('a registered shortcut runs what it was registered with', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('trigger')) {
      return t.skip('a real key combination cannot be pressed from a test');
    }

    let pressed = 0;
    port.register(POSTPONE, () => (pressed += 1));
    driver.trigger(POSTPONE.identity);

    strictEqual(pressed, 1);
  });

  test('a global shortcut registry is empty once everything it held is unregistered', () => {
    const { port } = makeSubject();

    port.register(POSTPONE, () => {});

    strictEqual(port.unregisterAll(), 1);
    deepStrictEqual(port.registered(), []);
  });

  test('unregistering an identity nobody holds does nothing, on every exit path', () => {
    const { port } = makeSubject();

    port.unregister('never-registered');

    deepStrictEqual(port.registered(), []);
  });

  test('a process that dies abnormally leaves its registrations to the system, not to the port', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('simulateAbnormalExit')) {
      return t.skip('a real process cannot be killed from inside its own test');
    }

    port.register(POSTPONE, () => {});
    driver.simulateAbnormalExit();

    deepStrictEqual(port.registered(), []);
    deepStrictEqual(driver.leftovers().map((shortcut) => shortcut.identity), [POSTPONE.identity]);
  });
}
