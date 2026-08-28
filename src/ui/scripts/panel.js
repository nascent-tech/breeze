import { clockTimeOf, countdownOf } from './format.js';

const RING_CIRCUMFERENCE = 339.3;
const END_BREAK = 'endBreak';
const TAKE_BREAK = 'takeBreakNow';

function element(id) {
  const found = document.getElementById(id);

  if (found === null) {
    throw new Error(`the panel carries no ${id}`);
  }

  return found;
}

const phased = (id, phase) => element(id).dataset[phase] ?? '';

function paintRing(state) {
  const total = state.phase === 'break' ? state.rhythm.pauseMinutes : state.rhythm.workMinutes;
  const served = Math.min(1, Math.max(0, 1 - state.remainingMilliseconds / (total * 60000)));

  element('ring').setAttribute('stroke-dasharray', `${served * RING_CIRCUMFERENCE} ${RING_CIRCUMFERENCE}`);
}

function toggleDisabled(id, disabled) {
  element(id).toggleAttribute('disabled', disabled);
}

function paintLevers(state) {
  element('primary-lever').textContent = phased('primary-lever', state.phase);
  toggleDisabled('primary-lever', !state.primaryLever.offered);
  toggleDisabled('postpone', !state.postpone.offered);
  element('postpone-reason').textContent = phased('postpone-reason', state.postpone.reason);
  toggleDisabled('restart', state.phase === 'break');
  element('restart-reason').textContent = phased('restart-reason', state.phase === 'break' ? 'break' : 'offered');
}

function paintBudget(state) {
  const { budgetRemainingMinutes, budgetFullMinutes } = state;

  element('budget').textContent = `${budgetRemainingMinutes} / ${budgetFullMinutes} min`;
  element('budget-gauge').style.width = `${(budgetRemainingMinutes / budgetFullMinutes) * 100}%`;
}

function paintSeverity(state) {
  element('severity-chip').textContent = state.severity === 'simple' ? 'Simple' : 'Hardcore';

  for (const item of document.querySelectorAll('[data-severity]')) {
    item.setAttribute('aria-pressed', String(item.getAttribute('data-severity') === state.severity));
  }
}

function paint(state) {
  element('phase-title').textContent = phased('phase-title', state.phase);
  element('phase-ordinal').textContent = (element('phase-ordinal').dataset.pattern ?? '').replace('{n}', state.ordinal);
  element('countdown').textContent = countdownOf(state.remainingMilliseconds);
  element('deadline').textContent = phased('deadline', state.phase).replace('{time}', clockTimeOf(state.endsAt));

  paintRing(state);
  paintLevers(state);
  paintBudget(state);
  paintSeverity(state);
}

function wire() {
  element('primary-lever').addEventListener('click', () => {
    window.breeze.send(element('primary-lever').dataset.command === 'break' ? END_BREAK : TAKE_BREAK);
  });
  element('postpone').addEventListener('click', () => window.breeze.send('postpone'));
  element('restart').addEventListener('click', () => window.breeze.send('restart'));
  element('open-settings').addEventListener('click', () => window.breeze.send('openSettings'));
  element('quit').addEventListener('click', () => window.breeze.send('quit'));

  for (const item of document.querySelectorAll('[data-severity]')) {
    item.addEventListener('click', () => window.breeze.send('setSeverity', item.getAttribute('data-severity')));
  }
}

window.breeze.onState((state) => {
  element('primary-lever').dataset.command = state.phase === 'break' ? 'break' : 'work';
  paint(state);
});
wire();
