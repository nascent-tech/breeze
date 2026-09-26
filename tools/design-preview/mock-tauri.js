// Hôte Tauri simulé pour l'aperçu design (injecté par serve.py, jamais livré).
// Paramètres d'URL : phase=Working|Notice|Break|Returning|Suspended|Inactive,
// frozen=1, severity=Simple|Hardcore, bar=0 pour masquer la barre d'états.
(function () {
  const params = new URLSearchParams(location.search);
  const PHASES = ["Working", "Notice", "Break", "Returning", "Suspended", "Inactive"];
  const state = {
    phase: PHASES.includes(params.get("phase")) ? params.get("phase") : defaultPhase(),
    frozen: params.get("frozen") === "1",
    severity: params.get("severity") || "Simple",
    workMinutes: 50,
    pauseMinutes: 10,
    servedToday: 3,
    phaseStartedAt: Date.now(),
  };

  function defaultPhase() {
    return /overlay/.test(location.pathname) ? "Break" : "Working";
  }

  const DURATIONS = {
    Working: () => state.workMinutes * 60,
    Notice: () => 60,
    Break: () => state.pauseMinutes * 60,
    Returning: () => 8,
    Suspended: () => 15 * 60,
    Inactive: () => 0,
  };
  const HEAD_START = { Working: 12 * 60 + 12, Notice: 17, Break: 2 * 60 + 18, Returning: 0, Suspended: 4 * 60, Inactive: 0 };

  function snapshot() {
    const total = DURATIONS[state.phase]();
    const elapsed = HEAD_START[state.phase] + Math.floor((Date.now() - state.phaseStartedAt) / 1000);
    const remaining = Math.max(0, total - elapsed);
    return {
      phase: state.phase,
      remaining_secs: remaining,
      total_secs: total,
      break_in_secs: state.phase === "Working" ? remaining + 60 : state.phase === "Notice" ? remaining : 0,
      frozen: state.frozen,
      frozen_reason: state.frozen ? "idle" : null,
      veil_mode: null,
      severity: state.severity,
      chosen_severity: state.severity,
      rhythm_pending: false,
      served_breaks: state.servedToday,
      served_today: state.servedToday,
      work_minutes: state.workMinutes,
      pause_minutes: state.pauseMinutes,
      debt_minutes: 6,
      inactive_reason: state.phase === "Inactive" ? "schedule" : null,
      next_start_label: state.phase === "Inactive" ? "9:00" : null,
    };
  }

  const SETTINGS = {
    work_minutes: 50,
    pause_minutes: 10,
    severity: "Simple",
    active_days: [true, true, true, true, true, false, false],
    schedule_start: "9:00",
    schedule_end: "18:00",
    update_check: true,
    launch_at_login: true,
    menubar_mode: "countdown",
    sounds: true,
  };

  const APPS = [
    ["com.figma.Desktop", "Figma", "blocked"],
    ["com.tinyspeck.slackmacgap", "Slack", "blocked"],
    ["com.microsoft.VSCode", "Visual Studio Code", "blocked"],
    ["com.apple.Safari", "Safari", "blocked"],
    ["com.apple.Notes", "Notes", "spared"],
    ["com.spotify.client", "Spotify", "spared"],
    ["com.apple.mail", "Mail", "blocked"],
  ].map(([id, name, status]) => ({ id, bundle_id: id, name, status, icon: null }));

  const handlers = {
    get_snapshot: snapshot,
    get_settings: () => SETTINGS,
    list_installed_apps: () => APPS,
    get_app_info: () => ({ version: "0.4.0", name: "Breeze" }),
    get_stats: () => ({ days: [], served_today: 3, streak_days: 4 }),
    set_severity: (a) => { state.severity = a.severity; },
    set_rhythm: (a) => { state.workMinutes = a.workMinutes; state.pauseMinutes = a.pauseMinutes; },
    suspend: () => go("Suspended"),
    resume: () => go("Working"),
    interrupt_break: () => go("Working"),
  };

  function go(phase) {
    state.phase = phase;
    state.phaseStartedAt = Date.now();
    document.documentElement.dataset.mockPhase = phase;
  }

  window.__TAURI__ = {
    core: {
      invoke: async (cmd, args) => {
        const handler = handlers[cmd];
        if (!handler) console.info("[mock-tauri]", cmd, args);
        return handler ? handler(args || {}) : null;
      },
    },
    event: { listen: async () => () => {} },
  };

  if (params.get("bar") === "0") return;
  window.addEventListener("DOMContentLoaded", () => {
    const bar = document.createElement("div");
    bar.setAttribute("aria-hidden", "true");
    bar.style.cssText = "position:fixed;left:50%;bottom:8px;transform:translateX(-50%);z-index:99999;display:flex;gap:2px;padding:3px;border-radius:9px;background:rgba(0,0,0,.72);font:11px -apple-system,sans-serif;opacity:.35;transition:opacity .2s";
    bar.addEventListener("mouseenter", () => { bar.style.opacity = "1"; });
    bar.addEventListener("mouseleave", () => { bar.style.opacity = ".35"; });
    PHASES.forEach((phase) => {
      const b = document.createElement("button");
      b.textContent = phase;
      b.style.cssText = "border:0;border-radius:6px;padding:3px 7px;background:transparent;color:#fff;font:inherit";
      b.addEventListener("click", () => go(phase));
      bar.appendChild(b);
    });
    document.body.appendChild(bar);
  });
})();
