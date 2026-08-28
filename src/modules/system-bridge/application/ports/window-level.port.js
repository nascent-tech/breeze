export class WindowLevelPort {
  apply(windowId, level) {
    throw new Error('WindowLevelPort.apply has no implementation');
  }

  levelOf(windowId) {
    throw new Error('WindowLevelPort.levelOf has no implementation');
  }
}
