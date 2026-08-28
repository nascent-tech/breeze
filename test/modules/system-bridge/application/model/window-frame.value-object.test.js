import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { Bounds } from '../../../../../src/modules/system-bridge/application/model/bounds.value-object.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';
import { WindowFrame } from '../../../../../src/modules/system-bridge/application/model/window-frame.value-object.js';

const BOUNDS = Bounds.of({ x: 0, y: 0, width: 800, height: 600 });
const FRAME = { windowId: 1, processId: 2, bounds: BOUNDS };

test('a window frame refuses an identifier that is not a positive whole number', () => {
  throws(() => WindowFrame.of({ ...FRAME, windowId: 0 }), InvalidValue);
  throws(() => WindowFrame.of({ ...FRAME, processId: 1.5 }), InvalidValue);
});

test('a window frame refuses bounds it did not receive', () => {
  throws(() => WindowFrame.of({ ...FRAME, bounds: { x: 0, y: 0, width: 1, height: 1 } }), InvalidValue);
});

test('a window frame refuses the title of the window it describes', () => {
  throws(() => WindowFrame.of({ ...FRAME, title: 'Inbox' }), InvalidValue);
});

test('two window frames of the same window, process and bounds are the same frame', () => {
  ok(WindowFrame.of(FRAME).equals(WindowFrame.of(FRAME)));
});

test('a window frame accepts no writing once it is built', () => {
  throws(() => Object.assign(WindowFrame.of(FRAME), { windowId: 9 }), TypeError);
});

test('a window frame built from nothing at all is refused like any other invalid value', () => {
  throws(() => WindowFrame.of(), InvalidValue);
});
