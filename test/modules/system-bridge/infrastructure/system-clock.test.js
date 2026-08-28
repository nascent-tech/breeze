import { runClockContract } from '../contract/clock.contract.js';
import { SystemClock } from '../../../../src/modules/system-bridge/infrastructure/system-clock.js';

runClockContract(() => ({ port: new SystemClock(), driver: { supports: () => false } }));
