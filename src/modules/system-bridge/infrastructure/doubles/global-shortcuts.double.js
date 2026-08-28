import { GlobalShortcutsPort } from '../../application/ports/global-shortcuts.port.js';
import { isReservedShortcut } from '../../application/model/reserved-shortcuts.js';
import { ReservedShortcut } from '../../application/errors/reserved-shortcut.error.js';
import { ShortcutAlreadyRegistered } from '../../application/errors/shortcut-already-registered.error.js';

export class GlobalShortcutsDouble extends GlobalShortcutsPort {
  #registrations = new Map();
  #leftovers = [];

  register(shortcut, listener) {
    if (this.#registrations.has(shortcut.identity)) {
      throw new ShortcutAlreadyRegistered('the identity is already registered');
    }

    if (isReservedShortcut(shortcut)) {
      throw new ReservedShortcut('the system reserves this combination');
    }

    this.#registrations.set(shortcut.identity, { shortcut, listener });
  }

  unregister(identity) {
    this.#registrations.delete(identity);
  }

  unregisterAll() {
    const count = this.#registrations.size;

    this.#registrations.clear();

    return count;
  }

  registered() {
    return [...this.#registrations.values()].map((registration) => registration.shortcut);
  }

  driver() {
    return {
      trigger: (identity) => this.#registrations.get(identity)?.listener(),
      simulateAbnormalExit: () => this.#simulateAbnormalExit(),
      leftovers: () => [...this.#leftovers],
      supports: () => true,
    };
  }

  #simulateAbnormalExit() {
    this.#leftovers = this.registered();
    this.#registrations.clear();
  }
}
