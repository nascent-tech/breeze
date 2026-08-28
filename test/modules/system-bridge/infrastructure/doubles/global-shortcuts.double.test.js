import { runGlobalShortcutsContract } from '../../contract/global-shortcuts.contract.js';
import { GlobalShortcutsDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/global-shortcuts.double.js';

runGlobalShortcutsContract(() => {
  const double = new GlobalShortcutsDouble();

  return { port: double, driver: double.driver() };
});
