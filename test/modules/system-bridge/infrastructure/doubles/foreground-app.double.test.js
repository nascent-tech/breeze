import { runForegroundAppContract } from '../../contract/foreground-app.contract.js';
import { ForegroundAppDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/foreground-app.double.js';

runForegroundAppContract(() => {
  const double = new ForegroundAppDouble();

  return { port: double, driver: double.driver() };
});
