import { InvalidValue } from '../errors/invalid-value.error.js';

const MONOTONIC = 'monotonic';
const WALL_CLOCK = 'wallClock';
const KNOWN_SCALES = [MONOTONIC, WALL_CLOCK];
const NANOSECONDS_PER_MILLISECOND = 1000000;

function requiredNanoseconds(value) {
  if (typeof value !== 'bigint') {
    throw new InvalidValue('a monotonic instant counts nanoseconds, which a number loses');
  }

  return value;
}

function requiredEpochMilliseconds(value = Number.NaN) {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new InvalidValue('a wall clock instant requires a finite number of milliseconds');
  }

  return value;
}

export class Instant {
  #scale;
  #value;

  constructor(scale = '', value) {
    if (!KNOWN_SCALES.includes(scale)) {
      throw new InvalidValue('an instant is read either on the monotonic scale or on the wall clock');
    }

    this.#scale = scale;
    this.#value = scale === MONOTONIC ? requiredNanoseconds(value) : requiredEpochMilliseconds(value);

    Object.freeze(this);
  }

  static monotonic(nanoseconds) {
    return new Instant(MONOTONIC, nanoseconds);
  }

  static wallClock(epochMilliseconds) {
    return new Instant(WALL_CLOCK, epochMilliseconds);
  }

  get isMonotonic() {
    return this.#scale === MONOTONIC;
  }

  millisecondsSince(other) {
    if (!(other instanceof Instant) || other.isMonotonic !== this.isMonotonic) {
      throw new InvalidValue('two instants of different scales have no common origin');
    }

    if (typeof this.#value === 'bigint' && typeof other.#value === 'bigint') {
      return Number(this.#value - other.#value) / NANOSECONDS_PER_MILLISECOND;
    }

    return Number(this.#value) - Number(other.#value);
  }
}
