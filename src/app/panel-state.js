const POSTPONE_MINUTES = 5;

function postponeOffer(snapshot) {
  if (snapshot.phase !== 'notice') {
    return { offered: false, reason: 'phase' };
  }

  if (snapshot.postponesTaken >= snapshot.postponeQuota) {
    return { offered: false, reason: 'quota' };
  }

  if (snapshot.budgetRemainingMinutes < POSTPONE_MINUTES) {
    return { offered: false, reason: 'budget' };
  }

  return { offered: true, reason: 'offered' };
}

function primaryOffer(snapshot, owedMinutes) {
  if (snapshot.phase !== 'break') {
    return { offered: true, owedMinutes: 0 };
  }

  return { offered: snapshot.budgetRemainingMinutes >= owedMinutes, owedMinutes };
}

const ESCAPE_HOLD_SECONDS = 10;

export function panelStateOf(snapshot, now, owedMinutes) {
  return {
    ...snapshot,
    escapeHoldSeconds: ESCAPE_HOLD_SECONDS,
    remainingMilliseconds: Math.max(0, snapshot.endsAt - now),
    postpone: postponeOffer(snapshot),
    primaryLever: primaryOffer(snapshot, owedMinutes),
  };
}
