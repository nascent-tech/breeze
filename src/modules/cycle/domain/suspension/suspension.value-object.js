import { InvalidValue } from '../cycle/invalid-value.error.js';

const MINUTE = 60000;
const QUARTER_HOUR = 15 * MINUTE;
const HOUR = 60 * MINUTE;
const TOMORROW_HOUR = 6;

function endOfQuarterHour(now) {
  return now + QUARTER_HOUR;
}

function endOfHour(now) {
  return now + HOUR;
}

function tomorrowMorning(now) {
  const morning = new Date(now);

  morning.setDate(morning.getDate() + 1);
  morning.setHours(TOMORROW_HOUR, 0, 0, 0);

  return morning.getTime();
}

const TERMS = new Map([
  ['quarterHour', endOfQuarterHour],
  ['hour', endOfHour],
  ['tomorrowMorning', tomorrowMorning],
]);

export class Suspension {
  #state;

  constructor(state) {
    this.#state = Object.freeze(state);
  }

  get until() {
    return this.#state.until;
  }

  get frozenRemainingMilliseconds() {
    return this.#state.frozenRemainingMilliseconds;
  }

  static of({ term, now, frozenRemainingMilliseconds }) {
    const end = TERMS.get(term);

    if (end === undefined) {
      throw new InvalidValue('a suspension lasts a quarter of an hour, an hour, or until tomorrow morning');
    }

    if (!Number.isFinite(now) || !Number.isFinite(frozenRemainingMilliseconds)) {
      throw new InvalidValue('a suspension freezes a finite remainder against a finite instant');
    }

    return new Suspension({ until: end(now), frozenRemainingMilliseconds });
  }

  static fromSnapshot(snapshot) {
    return new Suspension({
      until: snapshot.until,
      frozenRemainingMilliseconds: snapshot.frozenRemainingMilliseconds,
    });
  }

  snapshot() {
    return Object.freeze({ ...this.#state });
  }

  holdsAt(now) {
    return now < this.#state.until;
  }
}
