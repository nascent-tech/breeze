const STEPS = ['welcome', 'rhythm', 'severity', 'permissions', 'applications', 'start'];

function element(id) {
  const found = document.getElementById(id);

  if (found === null) {
    throw new Error(`the onboarding carries no ${id}`);
  }

  return found;
}

const chosen = { workMinutes: 50, pauseMinutes: 10, severity: 'simple', spared: new Set() };
let at = 0;

function paintDots() {
  element('dots').replaceChildren(...STEPS.map((step, index) => {
    const dot = document.createElement('span');

    dot.className = 'dot';
    dot.dataset.current = String(index === at);

    return dot;
  }));
}

function paintSummary() {
  element('summary').textContent = (element('summary').dataset.pattern ?? '')
    .replace('{work}', String(chosen.workMinutes))
    .replace('{pause}', String(chosen.pauseMinutes))
    .replace('{severity}', chosen.severity === 'simple' ? 'Simple' : 'Hardcore');
}

function paint() {
  for (const [index, step] of STEPS.entries()) {
    document.querySelector(`[data-step="${step}"]`)?.toggleAttribute('hidden', index !== at);
  }

  element('skip').hidden = STEPS[at] !== 'applications' && STEPS[at] !== 'permissions';
  element('next').textContent = element('next').dataset[STEPS[at]] ?? element('next').dataset.default ?? '';
  paintDots();
  paintSummary();
}

function choose(group, item) {
  for (const other of document.querySelectorAll(`.${group}`)) {
    other.setAttribute('aria-pressed', String(other === item));
  }
}

function wireChoices() {
  for (const item of document.querySelectorAll('.rhythm-choice')) {
    item.addEventListener('click', () => {
      chosen.workMinutes = Number(item.getAttribute('data-work'));
      chosen.pauseMinutes = Number(item.getAttribute('data-pause'));
      choose('rhythm-choice', item);
    });
  }

  for (const item of document.querySelectorAll('.severity-choice')) {
    item.addEventListener('click', () => {
      chosen.severity = item.getAttribute('data-severity') ?? 'simple';
      choose('severity-choice', item);
    });
  }
}

function chosenPreferences() {
  return {
    rhythm: { workMinutes: chosen.workMinutes, pauseMinutes: chosen.pauseMinutes },
    severity: chosen.severity,
    spared: [...chosen.spared],
  };
}

function advance() {
  if (at === STEPS.length - 1) {
    window.breeze.send('completeOnboarding', chosenPreferences());

    return;
  }

  at += 1;
  paint();
}

element('next').addEventListener('click', advance);
element('skip').addEventListener('click', advance);
wireChoices();
paint();
