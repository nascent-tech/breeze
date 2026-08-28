import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { Bounds } from '../../../../../src/modules/system-bridge/application/model/bounds.value-object.js';
import { Display } from '../../../../../src/modules/system-bridge/application/model/display.value-object.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';

const BOUNDS = Bounds.of({ x: 0, y: 0, width: 1920, height: 1080 });
const MAIN = { displayId: 1, bounds: BOUNDS, carriesMenuBar: true };

test('a display refuses an identifier that is not a positive whole number', () => {
  throws(() => Display.of({ ...MAIN, displayId: 0 }), InvalidValue);
});

test('a display refuses bounds it did not receive', () => {
  throws(() => Display.of({ ...MAIN, bounds: null }), InvalidValue);
});

test('a display refuses to leave the menu bar undecided', () => {
  throws(() => Display.of({ ...MAIN, carriesMenuBar: 'yes' }), InvalidValue);
  throws(() => Display.of({ ...MAIN, carriesMenuBar: undefined }), InvalidValue);
});

test('a display refuses a key it does not carry', () => {
  throws(() => Display.of({ ...MAIN, brightness: 0.5 }), InvalidValue);
});

test('two displays of the same identifier, bounds and menu bar are the same display', () => {
  ok(Display.of(MAIN).equals(Display.of(MAIN)));
});

test('a display built from nothing at all is refused like any other invalid value', () => {
  throws(() => Display.of(), InvalidValue);
});
