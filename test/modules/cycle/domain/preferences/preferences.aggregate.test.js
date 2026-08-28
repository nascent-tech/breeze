import { test } from 'node:test';
import { strictEqual, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/cycle/domain/cycle/invalid-value.error.js';
import { Preferences } from '../../../../../src/modules/cycle/domain/preferences/preferences.aggregate.js';

test('an application nobody has ruled on is blocked, which is what makes the first break real', () => {
  strictEqual(Preferences.byDefault().statusOf('com.figma.Desktop'), 'blocked');
});

test('the system settings are always reachable, whatever the preferences say', () => {
  const strict = Preferences.byDefault().withApplicationStatus('com.apple.systempreferences', 'blocked');

  strictEqual(strict.statusOf('com.apple.systempreferences'), 'spared');
});

test('an application is blocked, spared or ignored, and nothing else', () => {
  throws(() => Preferences.byDefault().withApplicationStatus('com.figma.Desktop', 'muted'), InvalidValue);
});

test('preferences read back from their snapshot are the preferences that were written', () => {
  const written = Preferences.byDefault().withSeverity('hardcore').withApplicationStatus('com.apple.Music', 'spared');
  const read = Preferences.fromSnapshot(written.snapshot());

  strictEqual(read.severity.name, 'hardcore');
  strictEqual(read.statusOf('com.apple.Music'), 'spared');
});

test('someone who sets nothing works fifty minutes, pauses ten, in the simple mode', () => {
  const preferences = Preferences.byDefault();

  strictEqual(preferences.rhythm.workMinutes, 50);
  strictEqual(preferences.severity.name, 'simple');
});
