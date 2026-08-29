import { Cycle } from '../../domain/cycle/cycle.aggregate.js';
import { Suspension } from '../../domain/suspension/suspension.value-object.js';

export class SuspendBreeze {
  #store;
  #clock;

  constructor(store, clock) {
    this.#store = store;
    this.#clock = clock;
  }

  suspend(term) {
    const kept = this.#store.read();
    const now = this.#clock.nowInMilliseconds();
    const suspension = Suspension.of({
      term,
      now,
      frozenRemainingMilliseconds: Math.max(0, kept.endsAt - now),
    });

    this.#store.write({ ...kept, suspension: suspension.snapshot() });
  }

  resume() {
    const kept = this.#store.read();

    if (kept.suspension === undefined) {
      return;
    }

    this.#store.write(this.#resumedFrom(kept, this.#clock.nowInMilliseconds()));
  }

  #resumedFrom(kept, now) {
    const suspension = Suspension.fromSnapshot(kept.suspension);
    const resumed = Cycle.fromSnapshot({
      ...kept,
      endsAt: now + suspension.frozenRemainingMilliseconds,
    }).snapshot();

    return { ...resumed, suspension: undefined };
  }
}
