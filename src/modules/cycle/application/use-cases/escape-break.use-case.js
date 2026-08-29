import { Cycle } from '../../domain/cycle/cycle.aggregate.js';

export class EscapeBreak {
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

    const escaped = Cycle.fromSnapshot(kept).escapeBreak(this.#clock.nowInMilliseconds()).snapshot();

    this.#store.write(escaped);

    return escaped;
  }
}
