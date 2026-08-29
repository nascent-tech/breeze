import { CycleStorePort } from '../application/ports/cycle-store.port.js';

export class KeptCycleStore extends CycleStorePort {
  #kept;
  #persistent;

  constructor(persistent) {
    super();

    this.#persistent = persistent;
    this.#kept = persistent.read();
  }

  read() {
    return this.#kept;
  }

  write(snapshot) {
    const changed = this.#turnsOver(snapshot);

    this.#kept = snapshot;

    if (changed) {
      this.#persistent.write(snapshot);
    }
  }

  flush() {
    if (this.#kept !== null) {
      this.#persistent.write(this.#kept);
    }
  }

  #turnsOver(snapshot) {
    if (this.#kept === null || snapshot === null) {
      return true;
    }

    return this.#kept.phase !== snapshot.phase || this.#kept.endsAt !== snapshot.endsAt;
  }
}
