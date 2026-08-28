import { AccessibilityPermissionPort } from '../../application/ports/accessibility-permission.port.js';
import { AccessibilityStatus } from '../../application/model/accessibility-status.value-object.js';

export class AccessibilityPermissionDouble extends AccessibilityPermissionPort {
  #status = AccessibilityStatus.notGranted();
  #grantsOnRequest = false;

  currentStatus() {
    return this.#status;
  }

  requestGrant() {
    if (this.#grantsOnRequest) {
      this.#status = AccessibilityStatus.granted();
    }
  }

  driver() {
    return {
      setStatus: (status) => this.#setStatus(status),
      grantsOnRequest: (grants) => this.#setGrantsOnRequest(grants),
      supports: () => true,
    };
  }

  #setStatus(status) {
    this.#status = status;
  }

  #setGrantsOnRequest(grants) {
    this.#grantsOnRequest = grants;
  }
}
