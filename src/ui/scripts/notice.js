import { countdownOf } from './format.js';

function element(id) {
  const found = document.getElementById(id);

  if (found === null) {
    throw new Error(`the notice carries no ${id}`);
  }

  return found;
}

function paint(state) {
  element('countdown').textContent = countdownOf(state.remainingMilliseconds);
  element('mode').textContent = element('mode').dataset[state.severity] ?? '';
  element('postpone').toggleAttribute('disabled', !state.postpone.offered);
  element('postpone-reason').textContent = element('postpone-reason').dataset[state.postpone.reason] ?? '';
}

element('postpone').addEventListener('click', () => window.breeze.send('postpone'));
window.breeze.onState(paint);
