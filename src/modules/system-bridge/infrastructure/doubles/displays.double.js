import { DisplaysPort } from '../../application/ports/displays.port.js';
import { DuplicateDisplay } from '../../application/errors/duplicate-display.error.js';

function subscribe(listeners, listener) {
  listeners.add(listener);

  return () => listeners.delete(listener);
}

export class DisplaysDouble extends DisplaysPort {
  #displays = [];
  #connectedListeners = new Set();
  #disconnectedListeners = new Set();
  #reconfiguredListeners = new Set();

  list() {
    const identifiers = new Set(this.#displays.map((display) => display.displayId));

    if (identifiers.size !== this.#displays.length) {
      throw new DuplicateDisplay('two displays share the same identifier');
    }

    return [...this.#displays];
  }

  onConnected(listener) {
    return subscribe(this.#connectedListeners, listener);
  }

  onDisconnected(listener) {
    return subscribe(this.#disconnectedListeners, listener);
  }

  onReconfigured(listener) {
    return subscribe(this.#reconfiguredListeners, listener);
  }

  driver() {
    return {
      setDisplays: (displays) => this.#setDisplays(displays),
      emitConnected: (display) => this.#emitConnected(display),
      emitDisconnected: (displayId) => this.#emitDisconnected(displayId),
      emitReconfigured: (display) => this.#emitReconfigured(display),
      supports: () => true,
    };
  }

  #setDisplays(displays) {
    this.#displays = [...displays];
  }

  #emitConnected(display) {
    this.#displays = [...this.#displays, display];
    this.#connectedListeners.forEach((listener) => listener(display));
  }

  #emitDisconnected(displayId) {
    this.#displays = this.#displays.filter((display) => display.displayId !== displayId);
    this.#disconnectedListeners.forEach((listener) => listener(displayId));
  }

  #emitReconfigured(display) {
    this.#displays = this.#displays.map((known) => (known.displayId === display.displayId ? display : known));
    this.#reconfiguredListeners.forEach((listener) => listener(display));
  }
}
