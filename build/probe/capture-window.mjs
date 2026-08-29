import { app, BrowserWindow } from 'electron';
import { writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const [page, width, height, output] = process.argv.slice(-4);
const SETTLE_MILLISECONDS = 1400;

function capture() {
  const window = new BrowserWindow({
    width: Number(width),
    height: Number(height),
    show: false,
    webPreferences: {
      preload: fileURLToPath(new URL('../../preload.js', import.meta.url)),
      sandbox: true,
      contextIsolation: true,
    },
  });

  window.loadFile(fileURLToPath(new URL(`../../src/ui/${page}`, import.meta.url)));
  window.webContents.once('did-finish-load', () => {
    setTimeout(() => {
      window.webContents.capturePage().then((image) => {
        writeFileSync(output, image.toPNG());
        app.exit(0);
      });
    }, SETTLE_MILLISECONDS);
  });
}

app.whenReady().then(capture);
