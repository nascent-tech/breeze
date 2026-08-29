import { clockTimeOf, countdownOf } from './format.js';

const HOLD_STEP_MILLISECONDS = 100;
const HOLD_MILLISECONDS = 10000;

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

function holdToEscape(holdMilliseconds) {
  let held = 0;
  const timer = setInterval(() => {
    held += HOLD_STEP_MILLISECONDS;
    element('escape-gauge').style.width = `${Math.min(100, (held / holdMilliseconds) * 100)}%`;

    if (held >= holdMilliseconds) {
      clearInterval(timer);
      askToLeave();
    }
  }, HOLD_STEP_MILLISECONDS);

  return timer;
}

function releasing(holding) {
  return () => {
    for (const timer of holding) {
      clearInterval(timer);
    }

    holding.clear();
    element('escape-gauge').style.width = '0%';
    window.breeze.send('escapeHoldReleased');
  };
}

function wireEscape(holdMilliseconds) {
  const holding = new Set();
  const release = releasing(holding);

  document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' && holding.size === 0 && element('confirm').hidden) {
      holding.add(holdToEscape(holdMilliseconds));
      window.breeze.send('escapeHoldStarted');
    }
  });

  window.addEventListener('blur', release);

  document.addEventListener('keyup', (event) => {
    if (event.key === 'Escape') {
      release();
    }
  });
}

element('end-break').addEventListener('click', () => window.breeze.send('endBreak'));
element('confirm-exit').addEventListener('click', () => window.breeze.send('escapeHoldReleased'));
element('confirm-stay').addEventListener('click', () => {
  element('confirm').hidden = true;
});

window.breeze.onState((state) => {
  paint(state);
  element('escape-hint').textContent = (element('escape-hint').dataset.pattern ?? '')
    .replace('{seconds}', String(state.escapeHoldSeconds));
});
wireEscape(HOLD_MILLISECONDS);
