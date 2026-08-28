import { runScreenSharingContract } from '../../contract/screen-sharing.contract.js';
import { ScreenSharingDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/screen-sharing.double.js';

runScreenSharingContract(() => {
  const double = new ScreenSharingDouble();

  return { port: double, driver: double.driver() };
});
