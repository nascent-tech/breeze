import { Cycle } from '../../domain/cycle/cycle.aggregate.js';

export class PostponeBreak {
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

    const moved = Cycle.fromSnapshot(kept).postpone(this.#clock.nowInMilliseconds()).snapshot();

    this.#store.write(moved);

    return moved;
  }
}
