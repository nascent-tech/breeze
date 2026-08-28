import { PresentationOptionsPort } from '../../application/ports/presentation-options.port.js';

const HIDDEN_DOCK_AND_MENU_BAR = 'hiddenDockAndMenuBar';
const DISABLED_APPLICATION_SWITCHING = 'disabledApplicationSwitching';

export class PresentationOptionsDouble extends PresentationOptionsPort {
  #applied = new Set();

  hideDockAndMenuBar() {
    this.#applied.add(HIDDEN_DOCK_AND_MENU_BAR);
  }

  disableApplicationSwitching() {
    this.#applied.add(DISABLED_APPLICATION_SWITCHING);
  }

  restore() {
    this.#applied.clear();
  }

  driver() {
    return {
      appliedOptions: () => [...this.#applied],
      simulateAbnormalExit: () => this.#applied.clear(),
      supports: () => true,
    };
  }
}
