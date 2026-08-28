import { InvalidValue } from '../cycle/invalid-value.error.js';
import { Rhythm } from '../cycle/rhythm.value-object.js';
import { Severity } from '../cycle/severity.value-object.js';

const BLOCKED = 'blocked';
const SPARED = 'spared';
const IGNORED = 'ignored';
const KNOWN_STATUSES = [BLOCKED, SPARED, IGNORED];
const ALWAYS_ALLOWED = ['com.apple.systempreferences'];

function knownStatus(status = '') {
  if (!KNOWN_STATUSES.includes(status)) {
    throw new InvalidValue('an application is blocked, spared or ignored');
  }

  return status;
}

export class Preferences {
  #state;

  constructor(state) {
    this.#state = Object.freeze(state);
  }

  get rhythm() {
    return this.#state.rhythm;
  }

  get severity() {
    return this.#state.severity;
  }

  static byDefault() {
    return new Preferences({
      rhythm: Rhythm.classic(),
      severity: Severity.simple(),
      applications: Object.freeze({}),
      onboardingCompleted: false,
    });
  }

  static fromSnapshot(snapshot) {
    return new Preferences({
      rhythm: Rhythm.of(snapshot.rhythm),
      severity: Severity.fromName(snapshot.severity),
      applications: Object.freeze({ ...snapshot.applications }),
      onboardingCompleted: snapshot.onboardingCompleted === true,
    });
  }

  snapshot() {
    return Object.freeze({
      rhythm: { workMinutes: this.rhythm.workMinutes, pauseMinutes: this.rhythm.pauseMinutes },
      severity: this.severity.name,
      applications: { ...this.#state.applications },
      onboardingCompleted: this.#state.onboardingCompleted,
    });
  }

  statusOf(bundleIdentifier) {
    if (ALWAYS_ALLOWED.includes(bundleIdentifier)) {
      return SPARED;
    }

    return this.#state.applications[bundleIdentifier] ?? BLOCKED;
  }

  withRhythm(rhythm) {
    return new Preferences({ ...this.#state, rhythm: Rhythm.of(rhythm) });
  }

  withSeverity(severity) {
    return new Preferences({ ...this.#state, severity: Severity.fromName(severity) });
  }

  withApplicationStatus(bundleIdentifier, status) {
    return new Preferences({
      ...this.#state,
      applications: Object.freeze({ ...this.#state.applications, [bundleIdentifier]: knownStatus(status) }),
    });
  }

  withOnboardingCompleted() {
    return new Preferences({ ...this.#state, onboardingCompleted: true });
  }
}
