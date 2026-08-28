import { InvalidValue } from '../errors/invalid-value.error.js';
import { refuseUnknownShape } from './literal-shape.js';
import { Bounds } from './bounds.value-object.js';

const REFUSAL = 'a window frame is built from a window, a process and bounds, and nothing else';
const KNOWN_KEYS = ['windowId', 'processId', 'bounds'];

function positiveIdentifier(value = 0, subject = '') {
  if (!Number.isInteger(value) || value <= 0) {
    throw new InvalidValue(`a window frame requires a positive ${subject}`);
  }

  return value;
}

function requiredBounds(value) {
  if (!(value instanceof Bounds)) {
    throw new InvalidValue('a window frame requires bounds');
  }

  return value;
}

export class WindowFrame {
  #windowId;
  #processId;
  #bounds;

  constructor(literal) {
    refuseUnknownShape(literal, KNOWN_KEYS, REFUSAL);

    this.#windowId = positiveIdentifier(literal.windowId, 'window identifier');
    this.#processId = positiveIdentifier(literal.processId, 'process identifier');
    this.#bounds = requiredBounds(literal.bounds);

    Object.freeze(this);
  }

  static of(literal) {
    return new WindowFrame(literal);
  }

  get windowId() {
    return this.#windowId;
  }

  get processId() {
    return this.#processId;
  }

  get bounds() {
    return this.#bounds;
  }

  equals(other) {
    return (
      other instanceof WindowFrame &&
      other.windowId === this.#windowId &&
      other.processId === this.#processId &&
      other.bounds.equals(this.#bounds)
    );
  }
}
