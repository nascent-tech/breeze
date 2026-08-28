import { InvalidValue } from '../errors/invalid-value.error.js';

const MAXIMUM_NAME = 'screen-saver';
const KNOWN_NAMES = [
  'normal',
  'floating',
  'torn-off-menu',
  'modal-panel',
  'main-menu',
  'status',
  'pop-up-menu',
  MAXIMUM_NAME,
];

export class WindowLevel {
  #name;

  constructor(name = '') {
    if (!KNOWN_NAMES.includes(name)) {
      throw new InvalidValue('a window level carries a name the stacking table knows');
    }

    this.#name = name;

    Object.freeze(this);
  }

  static maximum() {
    return new WindowLevel(MAXIMUM_NAME);
  }

  static fromName(name) {
    return new WindowLevel(name);
  }

  get name() {
    return this.#name;
  }

  equals(other) {
    return other instanceof WindowLevel && other.name === this.#name;
  }
}
