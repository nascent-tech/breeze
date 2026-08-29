const PANEL_LEVERS = ['takeBreakNow', 'endBreak', 'postpone', 'restart', 'suspend', 'resume'];
const PANEL_SETTINGS = ['setSeverity', 'openSettings', 'quit'];

const ALLOWED = new Map([
  ['panel', [...PANEL_LEVERS, ...PANEL_SETTINGS]],
  ['notice', ['postpone']],
  ['overlay', ['endBreak', 'escapeHoldStarted', 'escapeHoldReleased']],
  ['onboarding', ['completeOnboarding']],
  ['settings', ['setSeverity', 'setRhythm']],
]);

export function surfaceMaySend(surface, command) {
  const kind = String(surface).split(':')[0];

  return (ALLOWED.get(kind) ?? []).includes(command);
}
