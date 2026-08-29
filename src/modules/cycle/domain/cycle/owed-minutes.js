const MINUTE = 60000;

export function owedMinutesToEndBreakAt(snapshot, now) {
  return Math.ceil(Math.max(0, snapshot.endsAt - now) / MINUTE);
}
