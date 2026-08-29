const HOLD_MILLISECONDS = 10000;

export class EscapeHold {
  #startedAt = 0;
  #clock;

  constructor(clock) {
    this.#clock = clock;
  }

  get holdMilliseconds() {
    return HOLD_MILLISECONDS;
  }

  start() {
    this.#startedAt = this.#clock.nowInMilliseconds();
  }

  release() {
    this.#startedAt = 0;
  }

  isPaid() {
    return this.#startedAt !== 0 && this.#clock.nowInMilliseconds() - this.#startedAt >= HOLD_MILLISECONDS;
  }
}
