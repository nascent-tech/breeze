import { WindowLevelPort } from '../../application/ports/window-level.port.js';
import { UnknownWindow } from '../../application/errors/unknown-window.error.js';

export class WindowLevelDouble extends WindowLevelPort {
  #levels = new Map();

  apply(windowId, level) {
    this.#requireOpenWindow(windowId);

    this.#levels.set(windowId, level);
  }

  levelOf(windowId) {
    this.#requireOpenWindow(windowId);

    return this.#levels.get(windowId);
  }

  driver() {
    return {
      openWindow: (windowId) => this.#levels.set(windowId, undefined),
      destroyWindow: (windowId) => this.#levels.delete(windowId),
      supports: () => true,
    };
  }

  #requireOpenWindow(windowId) {
    if (!this.#levels.has(windowId)) {
      throw new UnknownWindow('the window no longer exists');
    }
  }
}
