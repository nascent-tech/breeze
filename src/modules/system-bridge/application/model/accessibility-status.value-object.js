import { InvalidValue } from '../errors/invalid-value.error.js';

const GRANTED = 'granted';
const NOT_GRANTED = 'notGranted';
const KNOWN_NAMES = [GRANTED, NOT_GRANTED];

export class AccessibilityStatus {
  #name;

  constructor(name = '') {
    if (!KNOWN_NAMES.includes(name)) {
      throw new InvalidValue('an accessibility status is either granted or not granted');
    }

    this.#name = name;

    Object.freeze(this);
  }

  static granted() {
    return new AccessibilityStatus(GRANTED);
  }

  static notGranted() {
    return new AccessibilityStatus(NOT_GRANTED);
  }

  static fromName(name) {
    return new AccessibilityStatus(name);
  }

  get name() {
    return this.#name;
  }

  get isGranted() {
    return this.#name === GRANTED;
  }

  equals(other) {
    return other instanceof AccessibilityStatus && other.name === this.#name;
  }
}
