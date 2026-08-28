import { WindowFramesPort } from '../../application/ports/window-frames.port.js';
import { AccessibilityDenied } from '../../application/errors/accessibility-denied.error.js';

export class WindowFramesDouble extends WindowFramesPort {
  #framesByProcess = new Map();
  #moveListeners = new Set();
  #resizeListeners = new Set();
  #accessibilityGranted = true;
  #polling = false;

  framesOf(processId) {
    this.#requireAccessibility();

    return [...(this.#framesByProcess.get(processId) ?? [])];
  }

  onMove(listener) {
    this.#requireAccessibility();

    return this.#subscribe(this.#moveListeners, listener);
  }

  onResize(listener) {
    this.#requireAccessibility();

    return this.#subscribe(this.#resizeListeners, listener);
  }

  pollingActive() {
    return this.#polling;
  }

  driver() {
    return {
      setFrames: (processId, frames) => this.#framesByProcess.set(processId, [...frames]),
      emitMove: (frame) => this.#emit(this.#moveListeners, frame),
      emitResize: (frame) => this.#emit(this.#resizeListeners, frame),
      simulateSilence: (milliseconds) => this.#simulateSilence(milliseconds),
      denyAccessibility: () => this.#setAccessibility(false),
      grantAccessibility: () => this.#setAccessibility(true),
      supports: () => true,
    };
  }

  #emit(listeners, frame) {
    this.#polling = false;
    listeners.forEach((listener) => listener(frame));
  }

  #simulateSilence(milliseconds) {
    this.#polling = milliseconds > 0;
  }

  #setAccessibility(granted) {
    this.#accessibilityGranted = granted;
  }

  #subscribe(listeners, listener) {
    listeners.add(listener);

    return () => listeners.delete(listener);
  }

  #requireAccessibility() {
    if (!this.#accessibilityGranted) {
      throw new AccessibilityDenied('reading window frames requires the accessibility permission');
    }
  }
}
