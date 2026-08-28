import { InvalidValue } from './invalid-value.error.js';

const KNOWN_KEYS = ['workMinutes', 'pauseMinutes'];
const WORK_BOUNDS = { least: 5, most: 180 };
const PAUSE_BOUNDS = { least: 1, most: 60 };

function wholeMinutes(value = Number.NaN, bounds = WORK_BOUNDS, subject = '') {
  if (!Number.isInteger(value) || value < bounds.least || value > bounds.most) {
    throw new InvalidValue(`a rhythm holds ${bounds.least} to ${bounds.most} minutes of ${subject}`);
  }

  return value;
}

export class Rhythm {
  #workMinutes;
  #pauseMinutes;

  constructor(literal) {
    if (literal === null || typeof literal !== 'object') {
      throw new InvalidValue('a rhythm is built from work minutes and pause minutes');
    }

    if (Object.keys(literal).some((key) => !KNOWN_KEYS.includes(key))) {
      throw new InvalidValue('a rhythm is built from work minutes and pause minutes');
    }

    this.#workMinutes = wholeMinutes(literal.workMinutes, WORK_BOUNDS, 'work');
    this.#pauseMinutes = wholeMinutes(literal.pauseMinutes, PAUSE_BOUNDS, 'pause');

    this.#refuseAPauseLongerThanItsWork();

    Object.freeze(this);
  }

  get workMinutes() {
    return this.#workMinutes;
  }

  get pauseMinutes() {
    return this.#pauseMinutes;
  }

  static of(literal) {
    return new Rhythm(literal);
  }

  static classic() {
    return new Rhythm({ workMinutes: 50, pauseMinutes: 10 });
  }

  equals(other) {
    return (
      other instanceof Rhythm &&
      other.workMinutes === this.#workMinutes &&
      other.pauseMinutes === this.#pauseMinutes
    );
  }

  #refuseAPauseLongerThanItsWork() {
    if (this.#pauseMinutes > this.#workMinutes) {
      throw new InvalidValue('a pause never runs longer than the work it follows');
    }
  }
}
