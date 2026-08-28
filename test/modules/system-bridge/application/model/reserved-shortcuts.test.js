import { test } from 'node:test';
import { ok } from 'node:assert/strict';

import { isReservedShortcut } from '../../../../../src/modules/system-bridge/application/model/reserved-shortcuts.js';
import { Shortcut } from '../../../../../src/modules/system-bridge/application/model/shortcut.value-object.js';

test('locking the session stays possible, whatever Breeze registers', () => {
  ok(isReservedShortcut(Shortcut.of({ identity: 'lock', keyEquivalent: 'q', modifiers: ['ctrl', 'cmd'] })));
});

test('powering the Mac down stays possible, whatever Breeze registers', () => {
  ok(isReservedShortcut(Shortcut.of({ identity: 'off', keyEquivalent: 'power', modifiers: ['ctrl'] })));
});

test('a combination the system does not reserve is left to Breeze', () => {
  ok(!isReservedShortcut(Shortcut.of({ identity: 'postpone', keyEquivalent: 'p', modifiers: ['cmd', 'shift'] })));
});
