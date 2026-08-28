import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

import { CycleStorePort } from '../application/ports/cycle-store.port.js';

export class FileCycleStore extends CycleStorePort {
  #path;

  constructor(path) {
    super();

    this.#path = path;
  }

  read() {
    try {
      return JSON.parse(readFileSync(this.#path, 'utf8'));
    } catch {
      return null;
    }
  }

  write(snapshot) {
    mkdirSync(dirname(this.#path), { recursive: true });
    writeFileSync(this.#path, JSON.stringify(snapshot, null, 2), 'utf8');
  }
}
