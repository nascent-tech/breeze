import { Cycle } from '../../domain/cycle/cycle.aggregate.js';
import { Rhythm } from '../../domain/cycle/rhythm.value-object.js';
import { Severity } from '../../domain/cycle/severity.value-object.js';

export class StartOrResumeCycle {
  #store;
  #clock;

  constructor(store, clock) {
    this.#store = store;
    this.#clock = clock;
  }

  execute(preferences) {
    const kept = this.#store.read();
    const now = this.#clock.nowInMilliseconds();

    if (kept === null) {
      return this.#started(preferences, now);
    }

    return this.#saved(Cycle.fromSnapshot(kept).advanceTo(now));
  }

  #started(preferences, now) {
    return this.#saved(
      Cycle.start({
        rhythm: Rhythm.of(preferences.rhythm),
        severity: Severity.fromName(preferences.severity),
        now,
      }),
    );
  }

  #saved(cycle) {
    const snapshot = cycle.snapshot();

    this.#store.write(snapshot);

    return snapshot;
  }
}
