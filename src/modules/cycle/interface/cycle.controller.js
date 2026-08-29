import { AdvanceCycle } from '../application/use-cases/advance-cycle.use-case.js';
import { ChangePreferences } from '../application/use-cases/change-preferences.use-case.js';
import { EndBreak } from '../application/use-cases/end-break.use-case.js';
import { EscapeBreak } from '../application/use-cases/escape-break.use-case.js';
import { PostponeBreak } from '../application/use-cases/postpone-break.use-case.js';
import { RestartCycle } from '../application/use-cases/restart-cycle.use-case.js';
import { StartOrResumeCycle } from '../application/use-cases/start-or-resume-cycle.use-case.js';
import { SuspendBreeze } from '../application/use-cases/suspend-breeze.use-case.js';
import { TakeBreakNow } from '../application/use-cases/take-break-now.use-case.js';

function levers(store, clock) {
  const useCases = new Map();

  useCases.set('advance', new AdvanceCycle(store, clock));
  useCases.set('postpone', new PostponeBreak(store, clock));
  useCases.set('takeBreakNow', new TakeBreakNow(store, clock));
  useCases.set('endBreak', new EndBreak(store, clock));
  useCases.set('escapeBreak', new EscapeBreak(store, clock));
  useCases.set('restart', new RestartCycle(store, clock));

  return useCases;
}

export class CycleController {
  #useCases;
  #preferences;
  #start;
  #changes;
  #suspension;

  constructor(store, clock, preferences) {
    this.#preferences = preferences;
    this.#useCases = levers(store, clock);
    this.#start = new StartOrResumeCycle(store, clock);
    this.#changes = new ChangePreferences(preferences, store);
    this.#suspension = new SuspendBreeze(store, clock);
  }

  start() {
    return this.#start.execute(this.#preferences.read().snapshot());
  }

  change(command, payload) {
    const changes = new Map([
      ['setSeverity', () => this.#changes.setSeverity(payload)],
      ['setRhythm', () => this.#changes.setRhythm(payload)],
      ['completeOnboarding', () => this.#completeOnboarding(payload)],
      ['suspend', () => this.#suspension.suspend(payload)],
      ['resume', () => this.#suspension.resume()],
    ]);

    changes.get(command)?.();

    return changes.has(command);
  }

  #completeOnboarding(payload) {
    this.#changes.setRhythm(payload.rhythm);
    this.#changes.setSeverity(payload.severity);
    this.#changes.spare(payload.spared);
    this.start();
  }

  handle(command) {
    const useCase = this.#useCases.get(command);

    if (useCase === undefined) {
      throw new Error('unknown command');
    }

    return useCase.execute();
  }
}
