import { ForegroundAppPort } from '../../application/ports/foreground-app.port.js';
import { AppIdentity } from '../../application/model/app-identity.value-object.js';

export class ForegroundAppDouble extends ForegroundAppPort {
  #currentApp = AppIdentity.of({ bundleIdentifier: 'co.nascent.breeze', displayName: 'Breeze' });
  #runningApps = [];
  #listeners = new Set();

  currentApp() {
    return this.#currentApp;
  }

  runningApps() {
    return [...this.#runningApps];
  }

  onChange(listener) {
    this.#listeners.add(listener);

    return () => this.#listeners.delete(listener);
  }

  driver() {
    return {
      setCurrentApp: (identity) => this.#setCurrentApp(identity),
      setRunningApps: (identities) => this.#setRunningApps(identities),
      emitChange: (identity) => this.#emitChange(identity),
      supports: () => true,
    };
  }

  #setCurrentApp(identity) {
    this.#currentApp = identity;
  }

  #setRunningApps(identities) {
    this.#runningApps = [...identities];
  }

  #emitChange(identity) {
    if (identity.equals(this.#currentApp)) {
      return;
    }

    this.#currentApp = identity;
    this.#listeners.forEach((listener) => listener(identity));
  }
}
