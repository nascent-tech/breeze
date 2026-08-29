import { Cycle } from '../../domain/cycle/cycle.aggregate.js';

export class ChangePreferences {
  #preferences;
  #store;

  constructor(preferences, store) {
    this.#preferences = preferences;
    this.#store = store;
  }

  setSeverity(severity) {
    const kept = this.#store.read();

    this.#preferences.write(this.#preferences.read().withSeverity(severity));

    if (kept === null || kept.phase !== 'work') {
      return;
    }

    this.#store.write(Cycle.fromSnapshot({ ...kept, severity }).snapshot());
  }

  setRhythm(rhythm) {
    this.#preferences.write(this.#preferences.read().withRhythm(rhythm));
  }

  spare(bundleIdentifiers) {
    let preferences = this.#preferences.read();

    for (const bundleIdentifier of bundleIdentifiers) {
      preferences = preferences.withApplicationStatus(bundleIdentifier, 'spared');
    }

    this.#preferences.write(preferences.withOnboardingCompleted());
  }
}
