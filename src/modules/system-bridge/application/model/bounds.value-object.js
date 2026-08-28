import { InvalidValue } from '../errors/invalid-value.error.js';
import { refuseUnknownShape } from './literal-shape.js';

const REFUSAL = 'bounds are built from an origin, a width and a height, and nothing else';
const KNOWN_KEYS = ['x', 'y', 'width', 'height'];

function finiteCoordinate(value = Number.NaN, subject = '') {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new InvalidValue(`bounds require a finite ${subject}`);
  }

  return value;
}

function nonNegativeExtent(value = Number.NaN, subject = '') {
  if (finiteCoordinate(value, subject) < 0) {
    throw new InvalidValue(`bounds refuse a negative ${subject}`);
  }

  return value;
}

export class Bounds {
  #x;
  #y;
  #width;
  #height;

  constructor(literal) {
    refuseUnknownShape(literal, KNOWN_KEYS, REFUSAL);

    this.#x = finiteCoordinate(literal.x, 'x');
    this.#y = finiteCoordinate(literal.y, 'y');
    this.#width = nonNegativeExtent(literal.width, 'width');
    this.#height = nonNegativeExtent(literal.height, 'height');

    Object.freeze(this);
  }

  static of(literal) {
    return new Bounds(literal);
  }

  get x() {
    return this.#x;
  }

  get y() {
    return this.#y;
  }

  get width() {
    return this.#width;
  }

  get height() {
    return this.#height;
  }

  equals(other) {
    return (
      other instanceof Bounds &&
      other.x === this.#x &&
      other.y === this.#y &&
      other.width === this.#width &&
      other.height === this.#height
    );
  }
}
