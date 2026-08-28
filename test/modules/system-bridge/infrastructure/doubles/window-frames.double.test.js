import { runWindowFramesContract } from '../../contract/window-frames.contract.js';
import { WindowFramesDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/window-frames.double.js';

runWindowFramesContract(() => {
  const double = new WindowFramesDouble();

  return { port: double, driver: double.driver() };
});
