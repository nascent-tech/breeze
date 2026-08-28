import { InvalidValue } from './invalid-value.error.js';

const SIMPLE = 'simple';
const HARDCORE = 'hardcore';
const POSTPONE_QUOTA = { [SIMPLE]: 3, [HARDCORE]: 1 };

export class Severity {
  #name;

  constructor(name = '') {
    if (name !== SIMPLE && name !== HARDCORE) {
      throw new InvalidValue('a severity is either simple or hardcore');
    }

    this.#name = name;

    Object.freeze(this);
  }

  get name() {
    return this.#name;
  }

  get postponeQuota() {
    return POSTPONE_QUOTA[this.#name];
  }

  static simple() {
    return new Severity(SIMPLE);
  }

  static hardcore() {
    return new Severity(HARDCORE);
  }

  static fromName(name) {
    return new Severity(name);
  }

  equals(other) {
    return other instanceof Severity && other.name === this.#name;
  }
}
