import { test } from 'node:test';
import { deepStrictEqual, ok, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';
import { Shortcut } from '../../../../../src/modules/system-bridge/application/model/shortcut.value-object.js';

const POSTPONE = { identity: 'postpone', keyEquivalent: 'p', modifiers: ['cmd', 'shift'] };

test('a shortcut refuses an empty combination', () => {
  throws(() => Shortcut.of({ ...POSTPONE, keyEquivalent: '' }), InvalidValue);
});

test('a global shortcut without a modifier is refused, as it would confiscate a key', () => {
  throws(() => Shortcut.of({ ...POSTPONE, modifiers: [] }), InvalidValue);
});

test('a shortcut refuses a modifier nobody knows, and a modifier said twice', () => {
  throws(() => Shortcut.of({ ...POSTPONE, modifiers: ['hyper'] }), InvalidValue);
  throws(() => Shortcut.of({ ...POSTPONE, modifiers: ['cmd', 'cmd'] }), InvalidValue);
});

test('a shortcut refuses an identity it could never name back', () => {
  throws(() => Shortcut.of({ ...POSTPONE, identity: '' }), InvalidValue);
  throws(() => Shortcut.of({ ...POSTPONE, identity: 'a'.repeat(129) }), InvalidValue);
});

test('the same modifiers in another order make the same shortcut', () => {
  const reordered = Shortcut.of({ ...POSTPONE, modifiers: ['shift', 'cmd'] });

  deepStrictEqual(reordered.modifiers, Shortcut.of(POSTPONE).modifiers);
});

test('a shortcut is the registration it names, whatever combination it carries', () => {
  const moved = Shortcut.of({ ...POSTPONE, keyEquivalent: 'k' });

  ok(moved.equals(Shortcut.of(POSTPONE)));
});

test('a shortcut refuses a modifier pushed onto the list it handed out', () => {
  const shortcut = Shortcut.of(POSTPONE);

  shortcut.modifiers.push('ctrl');

  deepStrictEqual(shortcut.modifiers, ['cmd', 'shift']);
});

test('a shortcut built from nothing at all is refused like any other invalid value', () => {
  throws(() => Shortcut.of(), InvalidValue);
});
