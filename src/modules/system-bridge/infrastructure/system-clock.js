import { ClockPort } from '../application/ports/clock.port.js';
import { Instant } from '../application/model/instant.value-object.js';

export class SystemClock extends ClockPort {
  monotonicNow() {
    return Instant.monotonic(process.hrtime.bigint());
  }

  wallClockNow() {
    return Instant.wallClock(Date.now());
  }
}
