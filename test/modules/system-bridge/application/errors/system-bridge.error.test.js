import { test } from 'node:test';
import { ok, strictEqual } from 'node:assert/strict';

import { SystemBridgeError } from '../../../../../src/modules/system-bridge/application/errors/system-bridge.error.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';

test('SystemBridgeError carries the name of the class that was thrown', () => {
  const error = new InvalidValue('a bundle identifier is required');

  strictEqual(error.name, 'InvalidValue');
});

test('SystemBridgeError makes every refusal of the context catchable as one family', () => {
  const error = new InvalidValue('a bundle identifier is required');

  ok(error instanceof SystemBridgeError);
});
