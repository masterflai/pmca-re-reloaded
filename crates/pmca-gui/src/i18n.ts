/** Localisation: shipped languages, catalogue loading and lookup. */

import { invoke } from "@tauri-apps/api/core";
import { translateElevation } from "./elevation";
import { translateThemeSwitch } from "./preferences";

// Locales shipped with the app. Extend by adding a folder under /locales and
// listing it here (kept explicit so the bundler can resolve the imports).
export const LOCALES = [
  "en-US",
  "de-DE",
  "fr-FR",
  "ja-JP",
  "zh-CN",
  "zh-TW",
  "zh-HK",
  "ko-KR",
  "vi-VN",
  "th-TH",
  "ru-RU",
  "uk-UA",
  "es-ES",
  "hi-IN",
  "ar-SA",
  "pt-BR",
  "id-ID",
  "tr-TR",
  "bn-BD",
  "pa-IN",
  "te-IN",
  "nl-NL",
  "da-DK",
  "nb-NO",
  "pl-PL",
  "cs-CZ",
  "hr-HR",
] as const;
/** A language tag; the shipped ones are in `LOCALES`, plugins may add more. */
export type Locale = string;
export const FALLBACK: Locale = "en-US";

/**
 * The languages the window offers: the shipped ones, then those installed
 * plugins add. Filled by {@link loadLocales}; the translation editor keeps to
 * `LOCALES`, since only a shipped catalogue can be corrected.
 */
export const availableLocales: Locale[] = [...LOCALES];

/** Add the languages of installed plugins, as the backend reports them. */
export async function loadLocales(): Promise<void> {
  try {
    for (const l of await invoke<string[]>("available_locales")) {
      if (!availableLocales.includes(l)) availableLocales.push(l);
    }
  } catch {
    /* the shipped languages remain */
  }
}

/** Languages written right-to-left, which the document direction must follow. */
const RTL_LANGUAGES = ["ar", "fa", "he", "ur"];

// ---------------------------------------------------------------------------
// Localization
// ---------------------------------------------------------------------------

type Catalog = Record<string, string>;
export const catalogs = new Map<Locale, Catalog>();
let activeCatalog: Catalog = {};
let fallbackCatalog: Catalog = {};

/** Minimal Fluent-message parser: `key = value` lines, ignoring comments. */
function parseFtl(text: string): Catalog {
  const out: Catalog = {};
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq === -1) continue;
    out[line.slice(0, eq).trim()] = line.slice(eq + 1).trim();
  }
  return out;
}

async function loadCatalog(locale: Locale): Promise<Catalog> {
  if (catalogs.has(locale)) return catalogs.get(locale)!;
  // The catalogue comes from the backend rather than a fetched asset: the
  // translations are embedded in the executable, and a second copy shipped with
  // the web assets could drift out of sync with the one the backend uses.
  const cat = parseFtl(await invoke<string>("locale_catalog", { locale }));
  catalogs.set(locale, cat);
  return cat;
}

export const LOCALE_STORAGE_KEY = "pmca.locale";

/** The language chosen in the picker, else the one the system names. */
export function detectLocale(): Locale {
  try {
    const saved = localStorage.getItem(LOCALE_STORAGE_KEY);
    if (saved !== null && availableLocales.includes(saved)) return saved;
  } catch {
    /* storage unavailable; follow the system */
  }
  const nav = navigator.language;
  // Exact tag first: zh-TW and zh-HK must not fall to zh-CN by their shared prefix.
  return (
    availableLocales.find((l) => l === nav) ??
    availableLocales.find((l) => l.startsWith(nav.split("-")[0])) ??
    FALLBACK
  );
}

/** Translate a key using the active catalog, falling back to en-US. */
export function tr(key: string): string {
  return activeCatalog[key] ?? fallbackCatalog[key] ?? key;
}

/** Translate a key and fill in its `{ $name }` placeholders. */
export function trFill(key: string, values: Record<string, string | number>): string {
  return tr(key).replace(/\{\s*\$([A-Za-z][A-Za-z0-9_-]*)\s*\}/g, (whole, name: string) =>
    name in values ? String(values[name]) : whole,
  );
}

/**
 * The language the window is currently showing.
 *
 * Kept because editing a translation may affect the catalog in use, in which
 * case the shell has to be re-rendered in the same language it already had.
 */
export let currentLocale: Locale = FALLBACK;

/** A failure reported by a Tauri command. */
interface CommandError {
  code: string;
  message: string;
  detail?: string | null;
  /** An action that can be offered to make the operation possible. */
  recovery?: string | null;
}

export function isCommandError(value: unknown): value is CommandError {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as CommandError).code === "string" &&
    typeof (value as CommandError).message === "string"
  );
}

/**
 * Render a backend failure in the user's language.
 *
 * The backend reports a stable `code` rather than a translated sentence, so the
 * wording can be matched on reliably here. When a code has no translation the
 * backend's own message is shown instead: an untranslated sentence is still far
 * more useful than a bare code. The `detail` (a path, a status byte, a device
 * message) is appended untranslated, because it is diagnostic output rather
 * than prose — dropping it would cost the information a bug report needs.
 */
export function errorText(error: unknown): string {
  if (typeof error === "string") return error;
  if (!isCommandError(error)) return String(error);
  const translated = activeCatalog[`err-${error.code}`] ?? fallbackCatalog[`err-${error.code}`];
  if (!translated) return error.message;
  return error.detail ? `${translated} (${error.detail})` : translated;
}

/** The translated name of a camera mode id from the backend, or the id itself. */
export function modeName(id: string): string {
  return activeCatalog[`mode-${id}`] ?? fallbackCatalog[`mode-${id}`] ?? id;
}

export function translateTree(root: ParentNode): void {
  root.querySelectorAll<HTMLElement>("[data-i18n]").forEach((el) => {
    el.textContent = tr(el.dataset.i18n!);
  });
}

/**
 * Re-render error lines that are already on screen after a language change.
 *
 * Unlike labels, an error line has no `data-i18n` key: its text is produced
 * from a failure that arrived at some earlier point. The failure is therefore
 * kept on the element itself, so switching language does not leave the previous
 * language's sentence standing in an otherwise translated window.
 */
function retranslateErrors(root: ParentNode): void {
  root.querySelectorAll<HTMLElement>("[data-err-message]").forEach((el) => {
    el.textContent = errorText({
      code: el.dataset.errCode ?? "",
      message: el.dataset.errMessage!,
      detail: el.dataset.errDetail ?? null,
    });
  });
}

export async function applyLocale(locale: Locale): Promise<void> {
  [activeCatalog, fallbackCatalog] = await Promise.all([
    loadCatalog(locale),
    loadCatalog(FALLBACK),
  ]);
  currentLocale = locale;
  translateTree(document);
  retranslateErrors(document);
  translateThemeSwitch();
  translateElevation();
  document.documentElement.lang = locale;
  // Arabic and friends need the whole shell mirrored, not just the text runs.
  document.documentElement.dir = RTL_LANGUAGES.includes(locale.split("-")[0])
    ? "rtl"
    : "ltr";
  // Keep backend-generated messages in the same language as the shell.
  await invoke("set_locale", { locale }).catch(() => undefined);
}

/**
 * Name a locale in its own language, so the list stays readable to someone who
 * cannot read the language the app currently happens to be in. `Intl` knows the
 * names already; keeping a table of our own would only add something to forget
 * to update.
 */
export function localeLabel(locale: Locale): string {
  try {
    const name = new Intl.DisplayNames([locale], { type: "language" }).of(locale);
    if (name) return `${name[0].toLocaleUpperCase(locale)}${name.slice(1)}`;
  } catch {
    /* fall through to the bare tag */
  }
  return locale;
}
