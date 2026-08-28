export class CycleStorePort {
  read() {
    throw new Error('CycleStorePort.read has no implementation');
  }

  write(snapshot) {
    throw new Error('CycleStorePort.write has no implementation');
  }
}
