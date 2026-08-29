const MINUTE = 60000;

export function restartPriceMinutesOf(snapshot, now) {
  return Math.ceil(Math.max(0, now - snapshot.startedAt) / MINUTE);
}
