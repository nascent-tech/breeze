import { app } from 'electron';
import { writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { Breeze } from '../../src/app/breeze.js';
import { FileCycleStore } from '../../src/modules/cycle/infrastructure/file-cycle.store.js';
import { FilePreferencesStore } from '../../src/modules/cycle/infrastructure/file-preferences.store.js';
import { BreakSurfaces } from '../../src/app/break-surfaces.js';
import { Surfaces } from '../../src/app/surfaces.js';
import { SystemClock } from '../../src/modules/cycle/infrastructure/system-clock.js';

const OUTPUT = process.argv[process.argv.length - 1];
const SETTLE_MILLISECONDS = 1200;
const PANEL_WIDTH = 380;
const PANEL_HEIGHT = 680;

function capture() {
  const userData = app.getPath('userData');
  const surfaces = new Surfaces(new URL('../../preload.js', import.meta.url));
  const pages = {
    notice: new URL('../../src/ui/notice.html', import.meta.url),
    overlay: new URL('../../src/ui/overlay.html', import.meta.url),
  };
  const breeze = new Breeze({
    breakSurfaces: new BreakSurfaces(surfaces, pages),
    store: new FileCycleStore(join(userData, 'probe-cycle.json')),
    preferences: new FilePreferencesStore(join(userData, 'probe-preferences.json')),
    clock: new SystemClock(),
    surfaces,
  });

  breeze.start();

  const panel = surfaces.open('panel', {
    page: new URL('../../src/ui/panel.html', import.meta.url),
    width: PANEL_WIDTH,
    height: PANEL_HEIGHT,
    frame: false,
    transparent: true,
  });

  panel.webContents.once('did-finish-load', () => {
    surfaces.broadcast('breeze:state', breeze.state());
    setTimeout(() => {
      panel.webContents.capturePage().then((image) => {
        writeFileSync(OUTPUT, image.toPNG());
        app.exit(0);
      });
    }, SETTLE_MILLISECONDS);
  });
}

app.whenReady().then(capture);
