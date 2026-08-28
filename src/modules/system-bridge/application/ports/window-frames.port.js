export class WindowFramesPort {
  framesOf(processId) {
    throw new Error('WindowFramesPort.framesOf has no implementation');
  }

  onMove(listener) {
    throw new Error('WindowFramesPort.onMove has no implementation');
  }

  onResize(listener) {
    throw new Error('WindowFramesPort.onResize has no implementation');
  }

  pollingActive() {
    throw new Error('WindowFramesPort.pollingActive has no implementation');
  }
}
