const { contextBridge, ipcRenderer } = require('electron');

const CHANNEL_STATE = 'breeze:state';
const CHANNEL_COMMAND = 'breeze:command';

contextBridge.exposeInMainWorld('breeze', {
  onState: (listener) => {
    ipcRenderer.on(CHANNEL_STATE, (_event, state) => listener(state));

    return () => ipcRenderer.removeAllListeners(CHANNEL_STATE);
  },
  send: (command, payload) => ipcRenderer.invoke(CHANNEL_COMMAND, { command, payload }),
});
