import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';
import { ScreenSharingLevel } from '../../../../../src/modules/system-bridge/application/model/screen-sharing-level.value-object.js';

test('a screen sharing level refuses a name it does not know', () => {
  throws(() => ScreenSharingLevel.fromName('maybe'), InvalidValue);
});

test('a level macOS cannot tell apart is not the absence of sharing', () => {
  ok(!ScreenSharingLevel.indistinguishable().equals(ScreenSharingLevel.none()));
});

test('a screen sharing level read from its name is the one the factory builds', () => {
  ok(ScreenSharingLevel.fromName('presenting').equals(ScreenSharingLevel.presenting()));
});
