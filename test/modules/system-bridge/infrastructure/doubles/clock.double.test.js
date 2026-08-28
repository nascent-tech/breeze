import { runClockContract } from '../../contract/clock.contract.js';
import { ClockDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/clock.double.js';

runClockContract(() => {
  const double = new ClockDouble();

  return { port: double, driver: double.driver() };
});
