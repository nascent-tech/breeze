import { runAccessibilityPermissionContract } from '../../contract/accessibility-permission.contract.js';
import { AccessibilityPermissionDouble } from '../../../../../src/modules/system-bridge/infrastructure/doubles/accessibility-permission.double.js';

runAccessibilityPermissionContract(() => {
  const double = new AccessibilityPermissionDouble();

  return { port: double, driver: double.driver() };
});
