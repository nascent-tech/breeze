import { ClockPort } from '../../application/ports/clock.port.js';
import { Instant } from '../../application/model/instant.value-object.js';

const NANOSECONDS_PER_MILLISECOND = 1000000n;

export class ClockDouble extends ClockPort {
  #nanoseconds = 0n;
  #epochMilliseconds = 0;

  monotonicNow() {
    return Instant.monotonic(this.#nanoseconds);
  }

  wallClockNow() {
    return Instant.wallClock(this.#epochMilliseconds);
  }

  driver() {
    return {
      setWallClock: (epochMilliseconds) => this.#setWallClock(epochMilliseconds),
      advance: (milliseconds) => this.#advance(milliseconds),
      supports: () => true,
    };
  }

  #setWallClock(epochMilliseconds) {
    this.#epochMilliseconds = epochMilliseconds;
  }

  #advance(milliseconds) {
    this.#nanoseconds += BigInt(milliseconds) * NANOSECONDS_PER_MILLISECOND;
    this.#epochMilliseconds += milliseconds;
  }
}
