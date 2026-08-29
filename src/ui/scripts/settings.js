function element(id) {
  const found = document.getElementById(id);

  if (found === null) {
    throw new Error(`the settings carry no ${id}`);
  }

  return found;
}

function field(id) {
  const found = element(id);

  if (!(found instanceof HTMLInputElement)) {
    throw new Error(`${id} is not a field the settings can read`);
  }

  return found;
}

function showTab(name) {
  for (const tab of document.querySelectorAll('.settings-tab')) {
    tab.toggleAttribute('hidden', tab.getAttribute('data-tab') !== name);
  }

  for (const item of document.querySelectorAll('.nav-item')) {
    item.setAttribute('aria-pressed', String(item.getAttribute('data-tab') === name));
  }
}

function readRhythm() {
  return {
    workMinutes: Number(field('work-minutes').value),
    pauseMinutes: Number(field('pause-minutes').value),
  };
}

function sendRhythm() {
  window.breeze.send('setRhythm', readRhythm());
}

function paint(state) {
  field('work-minutes').value = String(state.rhythm.workMinutes);
  field('pause-minutes').value = String(state.rhythm.pauseMinutes);

  for (const item of document.querySelectorAll('[data-severity]')) {
    item.setAttribute('aria-pressed', String(item.getAttribute('data-severity') === state.severity));
  }
}

for (const item of document.querySelectorAll('.nav-item')) {
  item.addEventListener('click', () => showTab(item.getAttribute('data-tab') ?? 'rhythm'));
}

for (const item of document.querySelectorAll('[data-severity]')) {
  item.addEventListener('click', () => window.breeze.send('setSeverity', item.getAttribute('data-severity')));
}

for (const name of ['work-minutes', 'pause-minutes']) {
  field(name).addEventListener('change', sendRhythm);
}

window.breeze.onState(paint);
