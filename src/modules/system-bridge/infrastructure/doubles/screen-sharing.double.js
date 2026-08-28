import { ScreenSharingPort } from '../../application/ports/screen-sharing.port.js';
import { ScreenSharingLevel } from '../../application/model/screen-sharing-level.value-object.js';

export class ScreenSharingDouble extends ScreenSharingPort {
  #level = ScreenSharingLevel.none();

  currentLevel() {
    return this.#level;
  }

  driver() {
    return {
      setLevel: (level) => this.#setLevel(level),
      supports: () => true,
    };
  }

  #setLevel(level) {
    this.#level = level;
  }
}
