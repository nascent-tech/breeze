import { test } from 'node:test';
import { deepStrictEqual, ok } from 'node:assert/strict';

test('system-bridge exposes an importable entry point', async () => {
  const entryPoint = await import('../../../src/modules/system-bridge/index.js');

  ok(entryPoint);
});

test('system-bridge exposes no barrel export from its entry point', async () => {
  const entryPoint = await import('../../../src/modules/system-bridge/index.js');

  deepStrictEqual(Object.keys(entryPoint), []);
});
