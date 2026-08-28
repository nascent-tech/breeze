import { app, ipcMain, Menu, nativeImage, Tray } from 'electron';
import { join } from 'node:path';

import { Breeze } from './src/app/breeze.js';
import { FileCycleStore } from './src/modules/cycle/infrastructure/file-cycle.store.js';
import { FilePreferencesStore } from './src/modules/cycle/infrastructure/file-preferences.store.js';
import { SystemClock } from './src/modules/cycle/infrastructure/system-clock.js';
import { BreakSurfaces } from './src/app/break-surfaces.js';
import { Surfaces } from './src/app/surfaces.js';

const COMMAND_CHANNEL = 'breeze:command';
const PANEL = 'panel';
const PANEL_WIDTH = 380;
const PANEL_HEIGHT = 680;

function surfacesOf() {
  return new Surfaces(new URL('./preload.js', import.meta.url));
}

function openPanel(surfaces, breeze) {
  const panel = surfaces.open(PANEL, {
    page: new URL('./src/ui/panel.html', import.meta.url),
    width: PANEL_WIDTH,
    height: PANEL_HEIGHT,
    frame: false,
    resizable: false,
    transparent: true,
    skipTaskbar: true,
  });

  panel.show();
  panel.webContents.once('did-finish-load', () => surfaces.broadcast('breeze:state', breeze.state()));

  return panel;
}

function trayOf(surfaces, breeze) {
  const tray = new Tray(nativeImage.createEmpty());

  tray.setToolTip('Breeze');
  tray.on('click', () => openPanel(surfaces, breeze));
  tray.setContextMenu(Menu.buildFromTemplate([{ role: 'quit' }]));

  return tray;
}

function handleCommands(breeze) {
  ipcMain.handle(COMMAND_CHANNEL, (_event, { command }) => {
    if (command === 'quit') {
      return app.quit();
    }

    if (command === 'openSettings') {
      return breeze.state();
    }

    return breeze.handle(command);
  });
}

function run() {
  const userData = app.getPath('userData');
  const surfaces = surfacesOf();
  const pages = {
    notice: new URL('./src/ui/notice.html', import.meta.url),
    overlay: new URL('./src/ui/overlay.html', import.meta.url),
  };
  const breeze = new Breeze({
    breakSurfaces: new BreakSurfaces(surfaces, pages),
    store: new FileCycleStore(join(userData, 'cycle.json')),
    preferences: new FilePreferencesStore(join(userData, 'preferences.json')),
    clock: new SystemClock(),
    surfaces,
  });

  app.dock?.hide();
  breeze.start();
  handleCommands(breeze);
  global.breezeTray = trayOf(surfaces, breeze);
  openPanel(surfaces, breeze);
}

app.whenReady().then(run);
app.on('window-all-closed', () => {});
