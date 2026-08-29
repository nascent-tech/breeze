import { test } from 'node:test';
import { strictEqual } from 'node:assert/strict';

import { ChangePreferences } from '../../../../../src/modules/cycle/application/use-cases/change-preferences.use-case.js';
import { Preferences } from '../../../../../src/modules/cycle/domain/preferences/preferences.aggregate.js';

const NOON = 1756900000000;

function storeOver(kept) {
  return {
    read: () => kept.cycle,
    write: (snapshot) => {
      kept.cycle = snapshot;
    },
  };
}

function preferencesOver(kept) {
  return {
    read: () => kept.preferences,
    write: (written) => {
      kept.preferences = written;
    },
  };
}

const CLASSIC = { workMinutes: 50, pauseMinutes: 10 };
const RUNNING_CYCLE = {
  rhythm: CLASSIC,
  severity: 'simple',
  phase: 'work',
  endsAt: NOON + 600000,
  startedAt: NOON,
  budgetRemainingMinutes: 15,
  budgetFullMinutes: 15,
  postponesTaken: 0,
  postponeQuota: 3,
  ordinal: 1,
};

function doubles(phase = 'work') {
  const kept = { cycle: { ...RUNNING_CYCLE, phase }, preferences: Preferences.byDefault() };

  return { kept, store: storeOver(kept), preferences: preferencesOver(kept) };
}

test('a severity chosen during the work phase takes hold on the running cycle', () => {
  const { kept, store, preferences } = doubles('work');

  new ChangePreferences(preferences, store).setSeverity('hardcore');

  strictEqual(kept.cycle.severity, 'hardcore');
  strictEqual(kept.cycle.postponeQuota, 1);
});

test('a severity chosen once a break is announced is frozen until the next cycle', () => {
  const { kept, store, preferences } = doubles('notice');

  new ChangePreferences(preferences, store).setSeverity('hardcore');

  strictEqual(kept.cycle.severity, 'simple');
  strictEqual(kept.preferences.severity.name, 'hardcore');
});

test('a rhythm chosen now waits for the next cycle, never the break already due', () => {
  const { kept, store, preferences } = doubles('break');

  new ChangePreferences(preferences, store).setRhythm({ workMinutes: 25, pauseMinutes: 5 });

  strictEqual(kept.cycle.rhythm.workMinutes, 50);
  strictEqual(kept.preferences.rhythm.workMinutes, 25);
});

test('sparing applications at the end of the onboarding records that it was completed', () => {
  const { kept, store, preferences } = doubles('work');

  new ChangePreferences(preferences, store).spare(['com.apple.Music']);

  strictEqual(kept.preferences.statusOf('com.apple.Music'), 'spared');
  strictEqual(kept.preferences.snapshot().onboardingCompleted, true);
});
