import { clockTimeOf, countdownOf } from './format.js';

const ESCAPE_HOLD_MILLISECONDS = 10000;
const HOLD_STEP_MILLISECONDS = 100;

function element(id) {
  const found = document.getElementById(id);

  if (found === null) {
    throw new Error(`the overlay carries no ${id}`);
  }

  return found;
}

function paint(state) {
  element('overlay').className = `overlay overlay-${state.severity}`;
  element('countdown').textContent = countdownOf(state.remainingMilliseconds);
  const endsAt = element('ends-at');

  endsAt.textContent = (endsAt.dataset.pattern ?? '').replace('{time}', clockTimeOf(state.endsAt));
  element('simple-actions').hidden = state.severity !== 'simple';
  element('hardcore-escape').hidden = state.severity !== 'hardcore';
  element('end-break').toggleAttribute('disabled', !state.primaryLever.offered);
  element('end-break-reason').textContent = (element('end-break-reason').dataset.pattern ?? '')
    .replace('{minutes}', String(state.primaryLever.owedMinutes));
}

function askToLeave() {
  element('confirm').hidden = false;
}

function holdToEscape() {
  let held = 0;
  const timer = setInterval(() => {
    held += HOLD_STEP_MILLISECONDS;
    element('escape-gauge').style.width = `${Math.min(100, (held / ESCAPE_HOLD_MILLISECONDS) * 100)}%`;

    if (held >= ESCAPE_HOLD_MILLISECONDS) {
      clearInterval(timer);
      askToLeave();
    }
  }, HOLD_STEP_MILLISECONDS);

  return timer;
}

function wireEscape() {
  const holding = new Set();

  document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' && holding.size === 0) {
      holding.add(holdToEscape());
    }
  });

  document.addEventListener('keyup', (event) => {
    if (event.key !== 'Escape') {
      return;
    }

    for (const timer of holding) {
      clearInterval(timer);
    }

    holding.clear();
    element('escape-gauge').style.width = '0%';
  });
}

element('end-break').addEventListener('click', () => window.breeze.send('endBreak'));
element('confirm-exit').addEventListener('click', () => window.breeze.send('escapeBreak'));
element('confirm-stay').addEventListener('click', () => {
  element('confirm').hidden = true;
});

wireEscape();
window.breeze.onState(paint);
