/** User preferences kept in local storage: advanced mode, the function analysis switch, the update notification and the theme. */

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
 * On by default; hidden only when the user has explicitly switched it off.
 */
export let analysisOn = readAnalysisMode();

function readAnalysisMode(): boolean {
  try {
    return localStorage.getItem(ANALYSIS_STORAGE_KEY) !== "off";
  } catch {
    return true;
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
  applyNav();
}

// ---------------------------------------------------------------------------
// Collapsible navigation groups
// ---------------------------------------------------------------------------

// The one open group. Replaces `pmca.nav.groups`, whose list of folded groups
// could not tell a group added later from one the user had opened.
const NAV_OPEN_KEY = "pmca.nav.open";
const NAV_GROUPS = ["camera", "apps", "analysis", "maintenance"];

/** Groups that are folded shut; all of them until the user opens one. */
let collapsedGroups = readCollapsedGroups();

function readCollapsedGroups(): Set<string> {
  try {
    const open = localStorage.getItem(NAV_OPEN_KEY);
    return new Set(NAV_GROUPS.filter((g) => g !== open));
  } catch {
    return new Set(NAV_GROUPS);
  }
}

function saveCollapsedGroups(): void {
  const open = NAV_GROUPS.find((g) => !collapsedGroups.has(g));
  try {
    if (open) localStorage.setItem(NAV_OPEN_KEY, open);
    else localStorage.removeItem(NAV_OPEN_KEY);
  } catch {
    /* the choice still applies to this session */
  }
}

/** Fold a group open or shut and remember it. Opening one closes the others. */
export function toggleNavGroup(group: string): void {
  collapsedGroups = collapsedGroups.has(group)
    ? new Set(NAV_GROUPS.filter((g) => g !== group))
    : new Set(NAV_GROUPS);
  saveCollapsedGroups();
  applyNav();
}

/** Open the group a view lives in and close the others. */
export function expandNavGroup(group: string): void {
  if (!group || !collapsedGroups.has(group)) return;
  collapsedGroups = new Set(NAV_GROUPS.filter((g) => g !== group));
  saveCollapsedGroups();
  applyNav();
}

/**
 * Show or hide every navigation entry from its group's folded state, keeping
 * the analysis entry hidden until it is unlocked.
 */
export function applyNav(): void {
  for (const group of NAV_GROUPS) {
    const folded = collapsedGroups.has(group);
    document
      .querySelector<HTMLElement>(`.nav-section[data-group="${group}"]`)
      ?.classList.toggle("collapsed", folded);
    for (const item of document.querySelectorAll<HTMLElement>(`.nav-item[data-group="${group}"]`)) {
      const locked = item.dataset.view === "analysis" && !analysisOn;
      item.hidden = folded || locked;
    }
  }
}

/** Kept for callers that only mean the analysis entry; folds are applied too. */
export function applyAnalysisNav(): void {
  applyNav();
}

// ---------------------------------------------------------------------------
// Update notification
// ---------------------------------------------------------------------------

const UPDATE_CHECK_STORAGE_KEY = "pmca.updateCheck";

/**
 * Whether the window asks GitHub for a newer release at start.
 *
 * Off by default and whenever storage cannot be read: the check reaches the
 * network, which this program does only when the user has asked for it.
 */
export let updateCheckOn = readUpdateCheck();

function readUpdateCheck(): boolean {
  try {
    return localStorage.getItem(UPDATE_CHECK_STORAGE_KEY) === "on";
  } catch {
    return false;
  }
}

export function setUpdateCheck(on: boolean): void {
  updateCheckOn = on;
  try {
    localStorage.setItem(UPDATE_CHECK_STORAGE_KEY, on ? "on" : "off");
  } catch {
    /* the choice still applies to this session */
  }
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

/**
 * The three theme choices, in the order the button cycles through them.
 *
 * `system` is a first-class option rather than the absence of a choice: the
 * stylesheet resolves it through `prefers-color-scheme`, so a user who never
 * touches the switch follows their desktop, including when it flips at dusk.
 */
const THEMES = ["system", "light", "dark"] as const;
type Theme = (typeof THEMES)[number];
const THEME_STORAGE_KEY = "pmca.theme";

/**
 * Icons as inline SVG paths on a 16x16 grid; `currentColor` lets them follow
 * the button's colour without an icon font.
 */
const THEME_ICONS: Record<"light" | "dark", string> = {
  // Sun: disc plus eight rays.
  light:
    '<circle cx="8" cy="8" r="3.25"/><path d="M8 1v1.5M8 13.5V15M15 8h-1.5M2.5 8H1M12.95 3.05l-1.06 1.06M4.11 11.89l-1.06 1.06M12.95 12.95l-1.06-1.06M4.11 4.11L3.05 3.05"/>',
  // Crescent moon.
  dark: '<path d="M13.5 9.6A5.8 5.8 0 0 1 6.4 2.5a5.8 5.8 0 1 0 7.1 7.1z"/>',
};

// Small "A" in the bottom-right corner marking the automatic (system) mode.
const AUTO_BADGE = '<path class="theme-auto-badge" d="M10 15.5l2.5-6.5 2.5 6.5M10.9 13.2h3.2"/>';

const darkQuery = window.matchMedia("(prefers-color-scheme: dark)");

function currentTheme(): Theme {
  const theme = document.documentElement.dataset.theme ?? null;
  return isTheme(theme) ? theme : "system";
}

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
  renderThemeSwitch();
}

/**
 * Draw the button for the active choice. In system mode it shows the sun or
 * moon the desktop currently resolves to, shrunk to make room for the "A".
 */
function renderThemeSwitch(): void {
  const button = document.querySelector<HTMLButtonElement>("#theme-switch");
  if (!button) return;
  const theme = currentTheme();
  const glyph = theme === "system" ? (darkQuery.matches ? "dark" : "light") : theme;
  const icon =
    theme === "system"
      ? `<g class="theme-auto-glyph" transform="scale(0.75)">${THEME_ICONS[glyph]}</g>${AUTO_BADGE}`
      : THEME_ICONS[glyph];
  button.innerHTML = `<svg viewBox="0 0 16 16" aria-hidden="true">${icon}</svg>`;
  translateThemeSwitch();
}

/** Label the button in the active language. */
export function translateThemeSwitch(): void {
  const button = document.querySelector<HTMLButtonElement>("#theme-switch");
  if (!button) return;
  // Icon-only, so the name has to be supplied for screen readers and the
  // tooltip is the only hint a sighted user gets.
  const label = `${tr("settings-theme")}: ${tr(`settings-theme-${currentTheme()}`)}`;
  button.setAttribute("aria-label", label);
  button.title = label;
}

export function setupThemeSwitch(): void {
  const button = document.querySelector<HTMLButtonElement>("#theme-switch");
  if (!button) return;
  button.addEventListener("click", () => {
    const next = THEMES[(THEMES.indexOf(currentTheme()) + 1) % THEMES.length];
    applyTheme(next);
  });
  // The system icon mirrors the desktop, so it has to follow when that flips.
  darkQuery.addEventListener("change", () => {
    if (currentTheme() === "system") renderThemeSwitch();
  });
  renderThemeSwitch();
}
