import { screen } from 'electron';

const STATE_CHANNEL = 'breeze:state';

const NOTICE = 'notice';
const OVERLAY = 'overlay:';
const NOTICE_WIDTH = 420;
const NOTICE_HEIGHT = 128;
const NOTICE_MARGIN = 24;

function paintOnLoad(window, state) {
  if (window.webContents.isLoading()) {
    window.webContents.once('did-finish-load', () => window.webContents.send(STATE_CHANNEL, state));

    return;
  }

  window.webContents.send(STATE_CHANNEL, state);
}

export class BreakSurfaces {
  #surfaces;
  #pages;
  #state = {};

  constructor(surfaces, pages) {
    this.#surfaces = surfaces;
    this.#pages = pages;
  }

  showFor(state) {
    this.#state = state;

    if (state.phase === 'notice') {
      return this.#openNotice();
    }

    if (state.phase === 'break') {
      return this.#openOverlays(state);
    }

    this.#surfaces.close(NOTICE);
    this.#surfaces.closeEvery(OVERLAY);
  }

  #openNotice() {
    this.#surfaces.closeEvery(OVERLAY);

    const display = screen.getPrimaryDisplay().workArea;
    const notice = this.#surfaces.open(NOTICE, {
      page: this.#pages.notice,
      width: NOTICE_WIDTH,
      height: NOTICE_HEIGHT,
      x: display.x + display.width - NOTICE_WIDTH - NOTICE_MARGIN,
      y: display.y + NOTICE_MARGIN,
      frame: false,
      transparent: true,
      resizable: false,
      focusable: false,
      skipTaskbar: true,
    });

    notice.setAlwaysOnTop(true, 'screen-saver');
    paintOnLoad(notice, this.#state);
    notice.showInactive();
  }

  #openOverlays(state) {
    this.#surfaces.close(NOTICE);

    for (const display of screen.getAllDisplays()) {
      this.#openOverlayOn(display, state);
    }
  }

  #openOverlayOn(display, state) {
    const overlay = this.#surfaces.open(`${OVERLAY}${display.id}`, {
      page: this.#pages.overlay,
      ...display.bounds,
      frame: false,
      transparent: state.severity === 'simple',
      resizable: false,
      movable: false,
      skipTaskbar: true,
      fullscreenable: false,
    });

    overlay.setAlwaysOnTop(true, 'screen-saver');
    overlay.setVisibleOnAllWorkspaces(true, { visibleOnFullScreenScreen: true });
    paintOnLoad(overlay, state);
    overlay.show();
  }
}
