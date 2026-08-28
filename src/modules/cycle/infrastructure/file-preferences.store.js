import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

import { Preferences } from '../domain/preferences/preferences.aggregate.js';

export class FilePreferencesStore {
  #path;

  constructor(path) {
    this.#path = path;
  }

  read() {
    try {
      return Preferences.fromSnapshot(JSON.parse(readFileSync(this.#path, 'utf8')));
    } catch {
      return Preferences.byDefault();
    }
  }

  write(preferences) {
    mkdirSync(dirname(this.#path), { recursive: true });
    writeFileSync(this.#path, JSON.stringify(preferences.snapshot(), null, 2), 'utf8');
  }
}
