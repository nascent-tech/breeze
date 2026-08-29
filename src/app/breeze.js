import { owedMinutesToEndBreakAt } from '../modules/cycle/domain/cycle/owed-minutes.js';
import { CycleController } from '../modules/cycle/interface/cycle.controller.js';
import { panelStateOf } from './panel-state.js';

const STATE_CHANNEL = 'breeze:state';
const TICK_MILLISECONDS = 1000;

export class Breeze {
  #controller;
  #clock;
  #surfaces;
  #store;
  #breakSurfaces;
  #timers = new Set();
  #listeners = new Set();

  constructor({ store, clock, preferences, surfaces, breakSurfaces }) {
    this.#controller = new CycleController(store, clock, preferences);
    this.#clock = clock;
    this.#surfaces = surfaces;
    this.#store = store;
    this.#breakSurfaces = breakSurfaces;
  }

  start() {
    this.#controller.start();
    this.#timers.add(setInterval(() => this.tick(), TICK_MILLISECONDS));
  }

  stop() {
    for (const timer of this.#timers) {
      clearInterval(timer);
    }

    this.#timers.clear();
  }

  tick() {
    this.#controller.handle('advance');

    const state = this.state();

    this.#breakSurfaces.showFor(state);
    this.#surfaces.broadcast(STATE_CHANNEL, state);
    this.#listeners.forEach((listener) => listener(state));

    return state;
  }

  handle(command, payload) {
    if (!this.#controller.change(command, payload)) {
      this.#controller.handle(command);
    }

    return this.tick();
  }

  onState(listener) {
    this.#listeners.add(listener);

    return () => this.#listeners.delete(listener);
  }

  state() {
    const snapshot = this.#store.read();
    const now = this.#clock.nowInMilliseconds();

    return panelStateOf(snapshot, now, owedMinutesToEndBreakAt(snapshot, now));
  }
}
