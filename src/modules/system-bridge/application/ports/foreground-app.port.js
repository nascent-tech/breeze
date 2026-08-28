export class ForegroundAppPort {
  currentApp() {
    throw new Error('ForegroundAppPort.currentApp has no implementation');
  }

  runningApps() {
    throw new Error('ForegroundAppPort.runningApps has no implementation');
  }

  onChange(listener) {
    throw new Error('ForegroundAppPort.onChange has no implementation');
  }
}
