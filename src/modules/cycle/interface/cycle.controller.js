import { AdvanceCycle } from '../application/use-cases/advance-cycle.use-case.js';
import { ChangePreferences } from '../application/use-cases/change-preferences.use-case.js';
import { EndBreak } from '../application/use-cases/end-break.use-case.js';
import { EscapeBreak } from '../application/use-cases/escape-break.use-case.js';
import { PostponeBreak } from '../application/use-cases/postpone-break.use-case.js';
import { StartOrResumeCycle } from '../application/use-cases/start-or-resume-cycle.use-case.js';
import { TakeBreakNow } from '../application/use-cases/take-break-now.use-case.js';

export class CycleController {
  #useCases;
  #preferences;
  #start;
  #changes;
  #store;

  constructor(store, clock, preferences) {
    this.#preferences = preferences;
    this.#useCases = new Map();
    this.#useCases.set('advance', new AdvanceCycle(store, clock));
    this.#useCases.set('postpone', new PostponeBreak(store, clock));
    this.#useCases.set('takeBreakNow', new TakeBreakNow(store, clock));
    this.#useCases.set('endBreak', new EndBreak(store, clock));
    this.#useCases.set('escapeBreak', new EscapeBreak(store, clock));
    this.#start = new StartOrResumeCycle(store, clock);
    this.#changes = new ChangePreferences(preferences, store);
    this.#store = store;
  }

  start() {
    return this.#start.execute(this.#preferences.read().snapshot());
  }

  change(command, payload) {
    const changes = new Map([
      ['setSeverity', () => this.#changes.setSeverity(payload)],
      ['setRhythm', () => this.#changes.setRhythm(payload)],
      ['completeOnboarding', () => this.#completeOnboarding(payload)],
      ['restart', () => this.#restart()],
    ]);

    changes.get(command)?.();

    return changes.has(command);
  }

  #restart() {
    this.#store.write(null);
    this.start();
  }

  #completeOnboarding(payload) {
    this.#changes.setRhythm(payload.rhythm);
    this.#changes.setSeverity(payload.severity);
    this.#changes.spare(payload.spared);
    this.#restart();
  }

  handle(command) {
    const useCase = this.#useCases.get(command);

    if (useCase === undefined) {
      throw new Error('unknown command');
    }

    return useCase.execute();
  }
}
