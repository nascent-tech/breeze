import { AdvanceCycle } from '../application/use-cases/advance-cycle.use-case.js';
import { EndBreak } from '../application/use-cases/end-break.use-case.js';
import { PostponeBreak } from '../application/use-cases/postpone-break.use-case.js';
import { StartOrResumeCycle } from '../application/use-cases/start-or-resume-cycle.use-case.js';
import { TakeBreakNow } from '../application/use-cases/take-break-now.use-case.js';

export class CycleController {
  #useCases;
  #preferences;
  #start;

  constructor(store, clock, preferences) {
    this.#preferences = preferences;
    this.#useCases = new Map();
    this.#useCases.set('advance', new AdvanceCycle(store, clock));
    this.#useCases.set('postpone', new PostponeBreak(store, clock));
    this.#useCases.set('takeBreakNow', new TakeBreakNow(store, clock));
    this.#useCases.set('endBreak', new EndBreak(store, clock));
    this.#start = new StartOrResumeCycle(store, clock);
  }

  start() {
    return this.#start.execute(this.#preferences.read().snapshot());
  }

  handle(command) {
    const useCase = this.#useCases.get(command);

    if (useCase === undefined) {
      throw new Error('unknown command');
    }

    return useCase.execute();
  }
}
