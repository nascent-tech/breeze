import { InvalidValue } from '../errors/invalid-value.error.js';
import { refuseUnknownShape } from './literal-shape.js';

const REFUSAL = 'an app identity is built from a bundle identifier and a display name, and nothing else';
const KNOWN_KEYS = ['bundleIdentifier', 'displayName'];
const MAXIMUM_LENGTH = 256;

function boundedText(value = '', subject = '') {
  if (typeof value !== 'string' || value.trim() === '') {
    throw new InvalidValue(`an app identity requires a ${subject}`);
  }

  if (value.length > MAXIMUM_LENGTH) {
    throw new InvalidValue(`an app identity refuses a ${subject} beyond ${MAXIMUM_LENGTH} characters`);
  }

  return value;
}

export class AppIdentity {
  #bundleIdentifier;
  #displayName;

  constructor(literal) {
    refuseUnknownShape(literal, KNOWN_KEYS, REFUSAL);

    this.#bundleIdentifier = boundedText(literal.bundleIdentifier, 'bundle identifier');
    this.#displayName = boundedText(literal.displayName, 'display name');

    Object.freeze(this);
  }

  static of(literal) {
    return new AppIdentity(literal);
  }

  get bundleIdentifier() {
    return this.#bundleIdentifier;
  }

  get displayName() {
    return this.#displayName;
  }

  equals(other) {
    return (
      other instanceof AppIdentity &&
      other.bundleIdentifier === this.#bundleIdentifier &&
      other.displayName === this.#displayName
    );
  }
}
