import { runWindowLevelContract } from '../../contract/window-level.contract.js';
import { WindowLevelDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/window-level.double.js';

runWindowLevelContract(() => {
  const double = new WindowLevelDouble();

  return { port: double, driver: double.driver() };
});
