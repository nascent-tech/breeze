import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { AccessibilityStatus } from '../../../../../src/modules/system-bridge/application/model/accessibility-status.value-object.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';

test('an accessibility status refuses a name it does not know', () => {
  throws(() => AccessibilityStatus.fromName('neverAsked'), InvalidValue);
});

test('an accessibility status reads granted and not granted apart', () => {
  ok(AccessibilityStatus.granted().isGranted);
  ok(!AccessibilityStatus.notGranted().isGranted);
});

test('an accessibility status read from its name is the one the factory builds', () => {
  ok(AccessibilityStatus.fromName('granted').equals(AccessibilityStatus.granted()));
});
