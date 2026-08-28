import { test } from 'node:test';
import { strictEqual, throws } from 'node:assert/strict';

import { InvalidValue } from '../../../../../src/modules/cycle/domain/cycle/invalid-value.error.js';
import { Severity } from '../../../../../src/modules/cycle/domain/cycle/severity.value-object.js';

test('the simple mode offers three postponements a cycle, the hardcore one', () => {
  strictEqual(Severity.simple().postponeQuota, 3);
  strictEqual(Severity.hardcore().postponeQuota, 1);
});

test('a severity refuses a name that is neither simple nor hardcore', () => {
  throws(() => Severity.fromName('gentle'), InvalidValue);
});
