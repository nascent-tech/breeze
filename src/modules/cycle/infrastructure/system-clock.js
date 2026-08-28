import { ClockPort } from '../application/ports/clock.port.js';

export class SystemClock extends ClockPort {
  nowInMilliseconds() {
    return Date.now();
  }
}
