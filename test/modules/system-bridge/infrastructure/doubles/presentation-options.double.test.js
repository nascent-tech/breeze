import { runPresentationOptionsContract } from '../../contract/presentation-options.contract.js';
import { PresentationOptionsDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/presentation-options.double.js';

runPresentationOptionsContract(() => {
  const double = new PresentationOptionsDouble();

  return { port: double, driver: double.driver() };
});
