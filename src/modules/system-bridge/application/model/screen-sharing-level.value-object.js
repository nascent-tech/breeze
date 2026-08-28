import { InvalidValue } from '../errors/invalid-value.error.js';

const NONE = 'none';
const SHARING = 'sharing';
const PRESENTING = 'presenting';
const INDISTINGUISHABLE = 'indistinguishable';
const KNOWN_NAMES = [NONE, SHARING, PRESENTING, INDISTINGUISHABLE];

export class ScreenSharingLevel {
  #name;

  constructor(name = '') {
    if (!KNOWN_NAMES.includes(name)) {
      throw new InvalidValue('a screen sharing level carries one of the four names it knows');
    }

    this.#name = name;

    Object.freeze(this);
  }

  static none() {
    return new ScreenSharingLevel(NONE);
  }

  static sharing() {
    return new ScreenSharingLevel(SHARING);
  }

  static presenting() {
    return new ScreenSharingLevel(PRESENTING);
  }

  static indistinguishable() {
    return new ScreenSharingLevel(INDISTINGUISHABLE);
  }

  static fromName(name) {
    return new ScreenSharingLevel(name);
  }

  get name() {
    return this.#name;
  }

  equals(other) {
    return other instanceof ScreenSharingLevel && other.name === this.#name;
  }
}
