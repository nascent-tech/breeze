import { test } from 'node:test';
import { ok, strictEqual, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';
import { WindowLevel } from '../../../../../src/modules/system-bridge/application/model/window-level.value-object.js';

test('a window level refuses a name the stacking table does not know', () => {
  throws(() => WindowLevel.fromName('above-everything'), InvalidValue);
});

test('the maximum window level is the highest name the stacking table carries', () => {
  strictEqual(WindowLevel.maximum().name, 'screen-saver');
});

test('a window level read from its name is the one the factory builds', () => {
  ok(WindowLevel.fromName('screen-saver').equals(WindowLevel.maximum()));
});
