import { app, ipcMain } from 'electron';
import { join } from 'node:path';

import { Breeze } from './src/app/breeze.js';
import { FileCycleStore } from './src/modules/cycle/infrastructure/file-cycle.store.js';
import { KeptCycleStore } from './src/modules/cycle/infrastructure/kept-cycle.store.js';
import { FilePreferencesStore } from './src/modules/cycle/infrastructure/file-preferences.store.js';
import { SystemClock } from './src/modules/cycle/infrastructure/system-clock.js';
import { BreakSurfaces } from './src/app/break-surfaces.js';
import { surfaceMaySend } from './src/app/commands.js';
import { EscapeHold } from './src/app/escape-hold.js';
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

function refusalOf(failure) {
  const named = failure instanceof Error ? failure.name : 'Unknown';

  process.stderr.write(`breeze refused a command: ${named}\n`);

  return named;
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

function answerEscape({ breeze, hold, command }) {
  if (command === 'escapeHoldStarted') {
    hold.start();

    return { ok: true };
  }

  if (!hold.isPaid()) {
    hold.release();

    return { ok: false, reason: 'HoldNotPaid' };
  }

  hold.release();

  return { ok: true, state: breeze.handle('escapeBreak') };
}

function answer({ breeze, surfaces, hold, command, payload }) {
  if (command === 'escapeHoldStarted' || command === 'escapeHoldReleased') {
    return answerEscape({ breeze, hold, command });
  }

  if (command === 'quit') {
    app.quit();

    return { ok: true };
  }

  if (command === 'openSettings') {
    openSettings(surfaces, breeze);

    return { ok: true };
  }

  if (command === 'completeOnboarding') {
    breeze.handle(command, payload);
    surfaces.close(ONBOARDING);

    return { ok: true, state: openPanel(surfaces, breeze) && breeze.state() };
  }

  return { ok: true, state: breeze.handle(command, payload) };
}

function answered(asked) {
  try {
    return answer(asked);
  } catch (failure) {
    return { ok: false, reason: refusalOf(failure) };
  }
}

function handleCommands(breeze, surfaces, hold) {
  ipcMain.handle(COMMAND_CHANNEL, (event, { command, payload }) => {
    const allowed = surfaceMaySend(surfaces.nameOf(event.sender), command);
    const asked = { breeze, surfaces, hold, command, payload };

    return allowed ? answered(asked) : { ok: false, reason: 'NotOfferedHere' };
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
    store: new KeptCycleStore(new FileCycleStore(join(userData, 'cycle.json'))),
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
  handleCommands(breeze, surfaces, new EscapeHold(new SystemClock()));
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
