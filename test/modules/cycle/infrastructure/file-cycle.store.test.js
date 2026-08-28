import { test } from 'node:test';
import { deepStrictEqual, strictEqual } from 'node:assert/strict';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { FileCycleStore } from '../../../../src/modules/cycle/infrastructure/file-cycle.store.js';

function storeInATemporaryDirectory() {
  const directory = mkdtempSync(join(tmpdir(), 'breeze-'));

  return { store: new FileCycleStore(join(directory, 'state', 'cycle.json')), directory };
}

test('a store with nothing written yet reads no cycle, rather than failing', () => {
  const { store, directory } = storeInATemporaryDirectory();

  strictEqual(store.read(), null);
  rmSync(directory, { recursive: true, force: true });
});

test('a cycle written to the store is read back as it was written', () => {
  const { store, directory } = storeInATemporaryDirectory();
  const snapshot = { phase: 'work', endsAt: 42, budgetRemainingMinutes: 15 };

  store.write(snapshot);

  deepStrictEqual(store.read(), snapshot);
  rmSync(directory, { recursive: true, force: true });
});
