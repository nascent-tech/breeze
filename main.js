import { app, ipcMain } from 'electron';
import { join } from 'node:path';

import { Breeze } from './src/app/breeze.js';
import { FileCycleStore } from './src/modules/cycle/infrastructure/file-cycle.store.js';
import { FilePreferencesStore } from './src/modules/cycle/infrastructure/file-preferences.store.js';
import { SystemClock } from './src/modules/cycle/infrastructure/system-clock.js';
import { BreakSurfaces } from './src/app/break-surfaces.js';
import { MenuBar } from './src/app/menu-bar.js';
import { Surfaces } from './src/app/surfaces.js';

const COMMAND_CHANNEL = 'breeze:command';
const PANEL = 'panel';
const SETTINGS = 'settings';
const ONBOARDING = 'onboarding';
const SETTINGS_WIDTH = 860;
const SETTINGS_HEIGHT = 640;
const ONBOARDING_WIDTH = 560;
const ONBOARDING_HEIGHT = 680;
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

function openSettings(surfaces, breeze) {
  const settings = surfaces.open(SETTINGS, {
    page: new URL('./src/ui/settings.html', import.meta.url),
    width: SETTINGS_WIDTH,
    height: SETTINGS_HEIGHT,
    title: 'Réglages de Breeze',
  });

  settings.webContents.once('did-finish-load', () => surfaces.broadcast('breeze:state', breeze.state()));
  settings.show();

  return breeze.state();
}

function openOnboarding(surfaces) {
  const onboarding = surfaces.open(ONBOARDING, {
    page: new URL('./src/ui/onboarding.html', import.meta.url),
    width: ONBOARDING_WIDTH,
    height: ONBOARDING_HEIGHT,
    resizable: false,
    title: 'Bienvenue dans Breeze',
  });

  onboarding.show();
}

function menuBarOf(surfaces, breeze) {
  const labels = { name: 'Breeze', quit: 'Quitter Breeze', working: '{minutes} min', onBreak: 'Pause · {minutes} min' };

  return new MenuBar(labels, () => openPanel(surfaces, breeze));
}

function handleCommands(breeze, surfaces) {
  ipcMain.handle(COMMAND_CHANNEL, (_event, { command, payload }) => {
    if (command === 'quit') {
      return app.quit();
    }

    if (command === 'openSettings') {
      return openSettings(surfaces, breeze);
    }

    if (command === 'completeOnboarding') {
      surfaces.close(ONBOARDING);
    }

    return breeze.handle(command, payload);
  });
}

function breezeOver(surfaces, preferences) {
  const userData = app.getPath('userData');
  const pages = {
    notice: new URL('./src/ui/notice.html', import.meta.url),
    overlay: new URL('./src/ui/overlay.html', import.meta.url),
  };

  return new Breeze({
    breakSurfaces: new BreakSurfaces(surfaces, pages),
    store: new FileCycleStore(join(userData, 'cycle.json')),
    preferences,
    clock: new SystemClock(),
    surfaces,
  });
}

function keepMenuBarPainted(surfaces, breeze) {
  const menuBar = menuBarOf(surfaces, breeze);

  global.breezeMenuBar = menuBar;
  breeze.onState((state) => menuBar.paint(state));
}

function run() {
  const surfaces = surfacesOf();
  const preferences = new FilePreferencesStore(join(app.getPath('userData'), 'preferences.json'));
  const breeze = breezeOver(surfaces, preferences);

  app.dock?.hide();
  breeze.start();
  handleCommands(breeze, surfaces);
  keepMenuBarPainted(surfaces, breeze);

  if (preferences.read().snapshot().onboardingCompleted) {
    return openPanel(surfaces, breeze);
  }

  openOnboarding(surfaces);
}

app.whenReady().then(run).catch((failure) => {
  process.stderr.write(`breeze failed to start: ${failure?.stack ?? failure}\n`);
  app.exit(1);
});
app.on('window-all-closed', () => {});
