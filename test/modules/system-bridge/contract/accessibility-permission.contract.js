import { test } from 'node:test';
import { ok } from 'node:assert/strict';

import { AccessibilityStatus } from '../../../../src/modules/system-bridge/application/model/accessibility-status.value-object.js';

export function runAccessibilityPermissionContract(makeSubject) {
  test('an accessibility permission reads its status at every call, because a revocation is silent', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setStatus')) {
      return t.skip('the subject reads the permission of the system, which no test sets');
    }

    driver.setStatus(AccessibilityStatus.granted());
    ok(port.currentStatus().isGranted);

    driver.setStatus(AccessibilityStatus.notGranted());
    ok(!port.currentStatus().isGranted);
  });

  test('an accessibility permission asks for a grant without deciding its outcome', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('grantsOnRequest')) {
      return t.skip('the subject asks the human, whose answer no test writes');
    }

    driver.grantsOnRequest(false);
    port.requestGrant();

    ok(!port.currentStatus().isGranted);
  });

  test('an accessibility permission asked twice while granted opens nothing and refuses nothing', (t) => {
    const { port, driver } = makeSubject();

    if (!driver.supports('setStatus')) {
      return t.skip('the subject reads the permission of the system, which no test sets');
    }

    driver.setStatus(AccessibilityStatus.granted());
    port.requestGrant();
    port.requestGrant();

    ok(port.currentStatus().isGranted);
  });
}
