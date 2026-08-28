import { InvalidValue } from './invalid-value.error.js';

const WORK = 'work';
const NOTICE = 'notice';
const BREAK = 'break';
const RETURN = 'return';
const INACTIVE = 'inactive';
const KNOWN_NAMES = [WORK, NOTICE, BREAK, RETURN, INACTIVE];

export class CyclePhase {
  #name;

  constructor(name = '') {
    if (!KNOWN_NAMES.includes(name)) {
      throw new InvalidValue('a cycle phase carries one of the names the state machine knows');
    }

    this.#name = name;

    Object.freeze(this);
  }

  get name() {
    return this.#name;
  }

  get isBreak() {
    return this.#name === BREAK;
  }

  get announcesABreak() {
    return this.#name === NOTICE;
  }

  static work() {
    return new CyclePhase(WORK);
  }

  static notice() {
    return new CyclePhase(NOTICE);
  }

  static onBreak() {
    return new CyclePhase(BREAK);
  }

  static returning() {
    return new CyclePhase(RETURN);
  }

  static inactive() {
    return new CyclePhase(INACTIVE);
  }

  static fromName(name) {
    return new CyclePhase(name);
  }

  equals(other) {
    return other instanceof CyclePhase && other.name === this.#name;
  }
}
