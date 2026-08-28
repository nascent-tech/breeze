import { InvalidValue } from '../errors/invalid-value.error.js';
import { refuseUnknownShape } from './literal-shape.js';
import { Bounds } from './bounds.value-object.js';

const REFUSAL = 'a display is built from an identifier, bounds and a menu bar flag, and nothing else';
const KNOWN_KEYS = ['displayId', 'bounds', 'carriesMenuBar'];

function positiveIdentifier(value = 0) {
  if (!Number.isInteger(value) || value <= 0) {
    throw new InvalidValue('a display requires a positive identifier');
  }

  return value;
}

function requiredBounds(value) {
  if (!(value instanceof Bounds)) {
    throw new InvalidValue('a display requires bounds');
  }

  return value;
}

function requiredFlag(value = 'undecided') {
  if (typeof value !== 'boolean') {
    throw new InvalidValue('a display states whether it carries the menu bar');
  }

  return value;
}

export class Display {
  #displayId;
  #bounds;
  #carriesMenuBar;

  constructor(literal) {
    refuseUnknownShape(literal, KNOWN_KEYS, REFUSAL);

    this.#displayId = positiveIdentifier(literal.displayId);
    this.#bounds = requiredBounds(literal.bounds);
    this.#carriesMenuBar = requiredFlag(literal.carriesMenuBar);

    Object.freeze(this);
  }

  static of(literal) {
    return new Display(literal);
  }

  get displayId() {
    return this.#displayId;
  }

  get bounds() {
    return this.#bounds;
  }

  get carriesMenuBar() {
    return this.#carriesMenuBar;
  }

  equals(other) {
    return (
      other instanceof Display &&
      other.displayId === this.#displayId &&
      other.carriesMenuBar === this.#carriesMenuBar &&
      other.bounds.equals(this.#bounds)
    );
  }
}
