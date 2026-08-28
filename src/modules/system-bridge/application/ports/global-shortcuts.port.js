export class GlobalShortcutsPort {
  register(shortcut, listener) {
    throw new Error('GlobalShortcutsPort.register has no implementation');
  }

  unregister(identity) {
    throw new Error('GlobalShortcutsPort.unregister has no implementation');
  }

  unregisterAll() {
    throw new Error('GlobalShortcutsPort.unregisterAll has no implementation');
  }

  registered() {
    throw new Error('GlobalShortcutsPort.registered has no implementation');
  }
}
