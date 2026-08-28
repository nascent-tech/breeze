import { spawnSync } from 'node:child_process';
import { globSync } from 'node:fs';

const TEST_PATTERN = 'test/**/*.test.js';

const testFiles = globSync(TEST_PATTERN);

if (testFiles.length === 0) {
  console.error(`No file matches ${TEST_PATTERN}: the test gate would pass on an empty suite.`);
  process.exit(1);
}

const { status } = spawnSync(process.execPath, ['--test', ...testFiles], { stdio: 'inherit' });

process.exit(status ?? 1);
