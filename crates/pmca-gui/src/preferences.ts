/** User preferences kept in local storage: advanced mode, the function analysis switch and the theme. */

import { tr } from "./i18n";

// ---------------------------------------------------------------------------
// Advanced mode
// ---------------------------------------------------------------------------

const ADVANCED_STORAGE_KEY = "pmca.advanced";

/**
 * Whether the service shell is shown inside this window.
 *
 * A guard against a slip of the hand, not against an attacker: everything it
 * shows is a `pmca-console serviceshell` away regardless, and the file it is
 * stored in belongs to the user. What it buys is that reaching the most
 * destructive part of this application stays a decision rather than a click,
 * which is what the console used to provide for free.
 *
 * Held in a variable rather than read back from storage each time, so the
 * choice still holds for this session on a platform that refuses to store it.
 * It starts off whenever the answer is in doubt — a storage failure must not be
 * the reason a camera gets written to.
 */
let advanced = readAdvancedMode();

function readAdvancedMode(): boolean {
  try {
    return localStorage.getItem(ADVANCED_STORAGE_KEY) === "on";
  } catch {
    return false;
  }
}

export function advancedMode(): boolean {
  return advanced;
}

/** Apply the choice, and remember it if the platform lets us. */
export function setAdvancedMode(on: boolean): void {
  advanced = on;
  try {
    localStorage.setItem(ADVANCED_STORAGE_KEY, on ? "on" : "off");
  } catch {
    /* the choice still applies to this session */
  }
}

// ---------------------------------------------------------------------------
// Function analysis switch
// ---------------------------------------------------------------------------

const ANALYSIS_STORAGE_KEY = "pmca.analysis";

/**
 * Whether the function analysis appears in the navigation.
 *
 * Off by default and off whenever storage cannot be read, for the reason given
 * at {@link advanced}: it is a tool for people who want to find out something
 * new about their camera, not something an ordinary user should stumble into.
 */
export let analysisOn = readAnalysisMode();

function readAnalysisMode(): boolean {
  try {
    return localStorage.getItem(ANALYSIS_STORAGE_KEY) === "on";
  } catch {
    return false;
  }
}

/** Apply the choice, remember it, and show or hide the navigation entry. */
export function setAnalysisMode(on: boolean): void {
  analysisOn = on;
  try {
    localStorage.setItem(ANALYSIS_STORAGE_KEY, on ? "on" : "off");
  } catch {
    /* the choice still applies to this session */
  }
  applyAnalysisNav();
}

export function applyAnalysisNav(): void {
  const item = document.querySelector<HTMLElement>('.nav-item[data-view="analysis"]');
  if (item) item.hidden = !analysisOn;
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

/**
 * The three theme choices, in the order they appear in the switch.
 *
 * `system` is a first-class option rather than the absence of a choice: the
 * stylesheet resolves it through `prefers-color-scheme`, so a user who never
 * touches the switch follows their desktop, including when it flips at dusk.
 */
const THEMES = ["system", "light", "dark"] as const;
type Theme = (typeof THEMES)[number];
const THEME_STORAGE_KEY = "pmca.theme";

/**
 * Icons for the three choices, as inline SVG paths on a 16x16 grid.
 *
 * Inline rather than an icon font or an image: three glyphs do not justify a
 * dependency, and `currentColor` lets them inherit the segment's colour for
 * free -- including the inversion when a segment is selected.
 */
const THEME_ICONS: Record<Theme, string> = {
  // Monitor: the desktop decides.
  system:
    '<rect x="1.5" y="2.5" width="13" height="9" rx="1.5"/><path d="M5.5 14h5M8 11.5V14"/>',
  // Sun: disc plus eight rays.
  light:
    '<circle cx="8" cy="8" r="3.25"/><path d="M8 1v1.5M8 13.5V15M15 8h-1.5M2.5 8H1M12.95 3.05l-1.06 1.06M4.11 11.89l-1.06 1.06M12.95 12.95l-1.06-1.06M4.11 4.11L3.05 3.05"/>',
  // Crescent moon.
  dark: '<path d="M13.5 9.6A5.8 5.8 0 0 1 6.4 2.5a5.8 5.8 0 1 0 7.1 7.1z"/>',
};

function isTheme(value: string | null): value is Theme {
  return value !== null && (THEMES as readonly string[]).includes(value);
}

/**
 * The stored choice, or `system` when there is none.
 *
 * A theme is a deliberate preference, so it is persisted. `localStorage` can
 * throw when storage is disabled, which must not stop the app from starting.
 */
export function storedTheme(): Theme {
  try {
    const saved = localStorage.getItem(THEME_STORAGE_KEY);
    if (isTheme(saved)) return saved;
  } catch {
    /* storage unavailable; fall back to following the system */
  }
  return "system";
}

export function applyTheme(theme: Theme): void {
  document.documentElement.dataset.theme = theme;
  try {
    localStorage.setItem(THEME_STORAGE_KEY, theme);
  } catch {
    /* the choice still applies to this session */
  }
  document
    .querySelectorAll<HTMLButtonElement>("#theme-switch button")
    .forEach((b) => {
      b.setAttribute("aria-checked", String(b.dataset.theme === theme));
    });
}

/** Label the switch and its segments in the active language. */
export function translateThemeSwitch(): void {
  const group = document.querySelector<HTMLElement>("#theme-switch");
  if (!group) return;
  group.setAttribute("aria-label", tr("settings-theme"));
  group.querySelectorAll<HTMLButtonElement>("button").forEach((b) => {
    // Icon-only controls carry no text, so the name has to be supplied for
    // screen readers -- and the tooltip is the only hint a sighted user gets.
    const label = tr(`settings-theme-${b.dataset.theme}`);
    b.setAttribute("aria-label", label);
    b.title = label;
  });
}

export function setupThemeSwitch(current: Theme): void {
  const group = document.querySelector<HTMLElement>("#theme-switch");
  if (!group) return;
  for (const theme of THEMES) {
    const button = document.createElement("button");
    button.type = "button";
    button.dataset.theme = theme;
    button.setAttribute("role", "radio");
    button.setAttribute("aria-checked", String(theme === current));
    button.innerHTML = `<svg viewBox="0 0 16 16" aria-hidden="true">${THEME_ICONS[theme]}</svg>`;
    button.addEventListener("click", () => applyTheme(theme));
    group.append(button);
  }
}
