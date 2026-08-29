import { Cycle } from '../../domain/cycle/cycle.aggregate.js';
import { Suspension } from '../../domain/suspension/suspension.value-object.js';

export class AdvanceCycle {
  #store;
  #clock;

  constructor(store, clock) {
    this.#store = store;
    this.#clock = clock;
  }

  execute() {
    const kept = this.#store.read();

    if (kept === null) {
      throw new Error('no cycle is running');
    }

    const now = this.#clock.nowInMilliseconds();

    if (kept.suspension !== undefined && Suspension.fromSnapshot(kept.suspension).holdsAt(now)) {
      return kept;
    }

    const moved = { ...Cycle.fromSnapshot(kept).advanceTo(now).snapshot(), suspension: kept.suspension };

    this.#store.write(moved);

    return moved;
  }
}
