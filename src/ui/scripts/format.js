const MINUTE = 60000;

export function countdownOf(milliseconds) {
  const seconds = Math.max(0, Math.ceil(milliseconds / 1000));
  const minutes = Math.floor(seconds / 60);

  return `${String(minutes).padStart(2, '0')}:${String(seconds % 60).padStart(2, '0')}`;
}

export function clockTimeOf(epochMilliseconds) {
  return new Date(epochMilliseconds).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
}

export function minutesLeftOf(milliseconds) {
  return Math.ceil(Math.max(0, milliseconds) / MINUTE);
}
