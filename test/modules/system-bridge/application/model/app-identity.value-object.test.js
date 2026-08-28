import { test } from 'node:test';
import { ok, throws } from 'node:assert/strict';

import { AppIdentity } from '../../../../../src/modules/system-bridge/application/model/app-identity.value-object.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';

const FINDER = { bundleIdentifier: 'com.apple.finder', displayName: 'Finder' };

test('an app identity refuses an empty bundle identifier', () => {
  throws(() => AppIdentity.of({ ...FINDER, bundleIdentifier: '   ' }), InvalidValue);
});

test('an app identity refuses an empty display name', () => {
  throws(() => AppIdentity.of({ ...FINDER, displayName: '' }), InvalidValue);
});

test('an app identity refuses anything that looks like a window title', () => {
  throws(() => AppIdentity.of({ ...FINDER, windowTitle: 'Salary review.pdf' }), InvalidValue);
});

test('an app identity bounds what a third party application names itself', () => {
  throws(() => AppIdentity.of({ ...FINDER, displayName: 'a'.repeat(257) }), InvalidValue);
  throws(() => AppIdentity.of({ ...FINDER, bundleIdentifier: 'a'.repeat(257) }), InvalidValue);
});

test('two app identities of the same bundle and name are the same identity', () => {
  ok(AppIdentity.of(FINDER).equals(AppIdentity.of(FINDER)));
});

test('an app identity accepts no writing once it is built', () => {
  const identity = AppIdentity.of(FINDER);

  throws(() => Object.assign(identity, { displayName: 'Impostor' }), TypeError);
});

test('an app identity built from nothing at all is refused like any other invalid value', () => {
  throws(() => AppIdentity.of(), InvalidValue);
});
