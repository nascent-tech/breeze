import { runDisplaysContract } from '../../contract/displays.contract.js';
import { DisplaysDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/displays.double.js';

runDisplaysContract(() => {
  const double = new DisplaysDouble();

  return { port: double, driver: double.driver() };
});
