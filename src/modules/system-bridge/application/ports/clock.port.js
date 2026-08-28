export class ClockPort {
  monotonicNow() {
    throw new Error('ClockPort.monotonicNow has no implementation');
  }

  wallClockNow() {
    throw new Error('ClockPort.wallClockNow has no implementation');
  }
}
