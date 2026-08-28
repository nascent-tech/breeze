import { test } from 'node:test';
import { equal, ok } from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtemp } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const runner = join(import.meta.dirname, '..', '..', 'build', 'run-tests.mjs');

test('test runner refuses a suite that matches no test file', async () => {
  const emptyTree = await mkdtemp(join(tmpdir(), 'breeze-empty-suite-'));

  const { status, stderr } = spawnSync(process.execPath, [runner], {
    cwd: emptyTree,
    encoding: 'utf8',
  });

  equal(status, 1);
  ok(stderr.includes('test/**/*.test.js'));
});
