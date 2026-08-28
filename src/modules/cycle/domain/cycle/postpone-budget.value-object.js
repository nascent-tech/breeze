import { InvalidValue } from './invalid-value.error.js';

const FULL_MINUTES = 15;

export class PostponeBudget {
  #remainingMinutes;

  constructor(remainingMinutes = Number.NaN) {
    if (!Number.isFinite(remainingMinutes) || remainingMinutes < 0) {
      throw new InvalidValue('a postpone budget never goes below zero');
    }

    this.#remainingMinutes = Math.min(remainingMinutes, FULL_MINUTES);

    Object.freeze(this);
  }

  get remainingMinutes() {
    return this.#remainingMinutes;
  }

  get fullMinutes() {
    return FULL_MINUTES;
  }

  static full() {
    return new PostponeBudget(FULL_MINUTES);
  }

  static of(remainingMinutes) {
    return new PostponeBudget(remainingMinutes);
  }

  covers(minutes) {
    return minutes <= this.#remainingMinutes;
  }

  debit(minutes) {
    return new PostponeBudget(Math.max(0, this.#remainingMinutes - minutes));
  }
}
