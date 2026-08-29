import { BrowserWindow } from 'electron';
import { fileURLToPath } from 'node:url';

export class Surfaces {
  #windows = new Map();
  #preloadPath;

  constructor(preloadPath) {
    this.#preloadPath = fileURLToPath(preloadPath);
  }

  open(name, { page, ...options }) {
    const known = this.#windows.get(name);

    if (known !== undefined && !known.isDestroyed()) {
      known.show();

      return known;
    }

    const opened = this.#built(page, options);

    this.#windows.set(name, opened);

    return opened;
  }

  hide(name) {
    const known = this.#windows.get(name);

    if (known !== undefined && !known.isDestroyed()) {
      known.hide();
    }
  }

  close(name) {
    const known = this.#windows.get(name);

    this.#windows.delete(name);

    if (known !== undefined && !known.isDestroyed()) {
      known.destroy();
    }
  }

  closeEvery(prefix) {
    for (const name of [...this.#windows.keys()]) {
      if (String(name).startsWith(prefix)) {
        this.close(name);
      }
    }
  }

  nameOf(webContents) {
    for (const [name, window] of this.#windows) {
      if (!window.isDestroyed() && window.webContents.id === webContents.id) {
        return name;
      }
    }

    return '';
  }

  isOpen(name) {
    const known = this.#windows.get(name);

    return known !== undefined && !known.isDestroyed() && known.isVisible();
  }

  broadcast(channel, payload) {
    for (const window of this.#windows.values()) {
      if (!window.isDestroyed()) {
        window.webContents.send(channel, payload);
      }
    }
  }

  #forget(window) {
    for (const [name, known] of [...this.#windows]) {
      if (known === window) {
        this.#windows.delete(name);
      }
    }
  }

  #built(page, options) {
    const window = new BrowserWindow({
      show: false,
      ...options,
      webPreferences: { preload: this.#preloadPath, sandbox: true, contextIsolation: true, nodeIntegration: false },
    });

    window.webContents.setWindowOpenHandler(() => ({ action: 'deny' }));
    window.webContents.on('will-navigate', (event) => event.preventDefault());
    window.on('closed', () => this.#forget(window));
    window.loadFile(fileURLToPath(page));

    return window;
  }
}
