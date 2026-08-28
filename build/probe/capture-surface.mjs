import { app, BrowserWindow } from 'electron';
import { writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { BreakSurfaces } from '../../src/app/break-surfaces.js';
import { Breeze } from '../../src/app/breeze.js';
import { FileCycleStore } from '../../src/modules/cycle/infrastructure/file-cycle.store.js';
import { FilePreferencesStore } from '../../src/modules/cycle/infrastructure/file-preferences.store.js';
import { Surfaces } from '../../src/app/surfaces.js';
import { SystemClock } from '../../src/modules/cycle/infrastructure/system-clock.js';

const [severity, phase, output] = process.argv.slice(-3);
const SETTLE_MILLISECONDS = 1600;
const MINUTE = 60000;

function seeded(store, clock) {
  const now = clock.nowInMilliseconds();
  const onBreak = phase === 'break';

  store.write({
    rhythm: { workMinutes: 50, pauseMinutes: 10 },
    severity,
    phase,
    endsAt: onBreak ? now + 8 * MINUTE : now + MINUTE,
    startedAt: now - 49 * MINUTE,
    budgetRemainingMinutes: 15,
    budgetFullMinutes: 15,
    postponesTaken: 0,
    postponeQuota: severity === 'simple' ? 3 : 1,
    ordinal: 4,
  });
}

function capture() {
  const userData = app.getPath('userData');
  const clock = new SystemClock();
  const store = new FileCycleStore(join(userData, `probe-${severity}-${phase}.json`));
  const surfaces = new Surfaces(new URL('../../preload.js', import.meta.url));
  const pages = {
    notice: new URL('../../src/ui/notice.html', import.meta.url),
    overlay: new URL('../../src/ui/overlay.html', import.meta.url),
  };

  seeded(store, clock);

  const breeze = new Breeze({
    store,
    preferences: new FilePreferencesStore(join(userData, 'probe-preferences.json')),
    clock,
    surfaces,
    breakSurfaces: new BreakSurfaces(surfaces, pages),
  });

  breeze.tick();

  setTimeout(() => {
    const [window] = BrowserWindow.getAllWindows();

    window.webContents.capturePage().then((image) => {
      writeFileSync(output, image.toPNG());
      app.exit(0);
    });
  }, SETTLE_MILLISECONDS);
}

app.whenReady().then(capture);
