import { Menu, nativeImage, Tray } from 'electron';

const MINUTE = 60000;

export class MenuBar {
  #tray;
  #labels;

  constructor(labels, onClick) {
    this.#labels = labels;
    this.#tray = new Tray(nativeImage.createEmpty());
    this.#tray.setToolTip(labels.name);
    this.#tray.on('click', onClick);
    this.#tray.setContextMenu(Menu.buildFromTemplate([{ role: 'quit', label: labels.quit }]));
  }

  paint(state) {
    this.#tray.setTitle(this.#titleOf(state));
  }

  #titleOf(state) {
    const minutes = Math.ceil(state.remainingMilliseconds / MINUTE);

    if (state.phase === 'break') {
      return this.#labels.onBreak.replace('{minutes}', String(minutes));
    }

    return this.#labels.working.replace('{minutes}', String(minutes));
  }
}
