/** Settings page: optional features and the translation editor. */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  clearStatusState,
  confirmed,
  make,
  showDone,
  showError,
  showErrorKey,
  showStatus,
  statusLine,
} from "../dom";
import type { CatalogReport } from "../dto";
import {
  FALLBACK,
  LOCALES,
  applyLocale,
  catalogs,
  currentLocale,
  localeLabel,
  tr,
  type Locale,
} from "../i18n";
import {
  advancedMode,
  analysisOn,
  setAdvancedMode,
  setAnalysisMode,
  setUpdateCheck,
  updateCheckOn,
} from "../preferences";
import { cardPlugins } from "./plugins";
import { cardSources } from "./sources";

/**
 * Settings: everything about the *content* of a translation.
 *
 * The language of the window itself is chosen in the header, next to the theme
 * switch, because it is a preference rather than a setting to be administered.
 * What is left for this view is correcting a translation, which is a different
 * activity: it concerns one language at a time and not necessarily the one
 * currently on screen.
 */
export function viewSettings(host: HTMLElement): void {
  host.append(
    cardTranslationEditor(),
    cardSources(),
    cardPlugins(),
    cardUpdateCheck(),
    cardAdvancedMode(),
    cardAnalysisMode(),
  );
}

/** Opt in to hearing about new releases at start. */
function cardUpdateCheck(): HTMLElement {
  const status = statusLine();
  const toggle = make("input", { type: "checkbox" });
  toggle.checked = updateCheckOn;

  const report = (): void => {
    const key = updateCheckOn ? "settings-update-on" : "settings-update-off";
    showStatus(status, tr(key));
    status.dataset.i18n = key;
  };

  toggle.addEventListener("change", () => {
    setUpdateCheck(toggle.checked);
    report();
  });
  report();

  return card(
    make("h2", { i18n: "settings-update-heading" }),
    make("p", { i18n: "settings-update-purpose" }),
    make("label", { className: "row" }, toggle, make("span", { i18n: "settings-update-toggle" })),
    status,
  );
}

/**
 * Unlock the function analysis.
 *
 * The card says what the analysis is for and why it starts switched off, so
 * the decision is made knowing both. Turning it on asks once more; turning it
 * off does not, and leaves a run under way in the backend untouched.
 */
function cardAnalysisMode(): HTMLElement {
  const status = statusLine();
  const toggle = make("input", { type: "checkbox" });
  toggle.checked = analysisOn;

  const report = (): void => {
    const key = analysisOn ? "settings-analysis-on" : "settings-analysis-off";
    showStatus(status, tr(key));
    status.dataset.i18n = key;
  };

  toggle.addEventListener("change", async () => {
    if (
      toggle.checked &&
      !(await confirmed(tr("settings-analysis-confirm"), tr("btn-analysis-enable")))
    ) {
      toggle.checked = false;
      return;
    }
    setAnalysisMode(toggle.checked);
    report();
  });
  report();

  return card(
    make("h2", { i18n: "settings-analysis-heading" }),
    make("p", { i18n: "settings-analysis-purpose" }),
    make("p", { className: "warning", i18n: "settings-analysis-why-off" }),
    make("label", { className: "row" }, toggle, make("span", { i18n: "settings-analysis-toggle" })),
    status,
  );
}

/**
 * Show the service shell inside this window.
 *
 * Turning it *on* asks first, turning it off does not: one direction makes the
 * most destructive part of the application reachable by clicking, the other
 * only puts it back out of reach. The confirmation is the whole point of the
 * setting — it is what the trip to a console used to be.
 */
function cardAdvancedMode(): HTMLElement {
  const status = statusLine();
  const toggle = make("input", { type: "checkbox" });
  toggle.checked = advancedMode();

  const report = (): void => {
    showStatus(status, tr(advancedMode() ? "settings-advanced-on" : "settings-advanced-off"));
    status.dataset.i18n = advancedMode() ? "settings-advanced-on" : "settings-advanced-off";
  };

  toggle.addEventListener("change", async () => {
    if (
      toggle.checked &&
      !(await confirmed(tr("settings-advanced-confirm"), tr("btn-confirm-show-shells")))
    ) {
      toggle.checked = false;
      return;
    }
    setAdvancedMode(toggle.checked);
    report();
  });
  report();

  return card(
    make("h2", { i18n: "settings-advanced-heading" }),
    make("p", { className: "warning", i18n: "settings-advanced-hint" }),
    make("label", { className: "row" }, toggle, make("span", { i18n: "settings-advanced-toggle" })),
    status,
  );
}

/**
 * Download / edit / upload round-trip for a language file.
 *
 * The catalogues are compiled into the executable, so a correction cannot be
 * written back over the original. An upload is stored per user and layered over
 * the built-in wording instead, which is why leaving entries out is allowed:
 * whatever the file does not mention keeps the text it shipped with.
 */
function cardTranslationEditor(): HTMLElement {
  const status = statusLine();
  const state = make("p", { className: "hint" });
  const report = make("div", { className: "catalog-report" });

  const select = make("select");
  for (const l of LOCALES) {
    const option = document.createElement("option");
    option.value = l;
    option.textContent = localeLabel(l);
    if (l === currentLocale) option.selected = true;
    select.append(option);
  }
  const chosen = (): Locale => select.value as Locale;

  /** Say whether the selected language runs on its built-in text or the user's. */
  async function refresh(): Promise<void> {
    report.replaceChildren();
    const path = await invoke<string | null>("catalog_status", { locale: chosen() });
    // The label is a separate node so a language change retranslates it while
    // leaving the path — which is not translatable — where it is.
    state.replaceChildren(
      make("span", {
        i18n: path === null ? "settings-catalog-builtin" : "settings-catalog-custom",
      }),
    );
    if (path !== null) state.append(` ${path}`);
  }

  /** A `label: count` line, kept translatable across a language change. */
  function countItem(key: string, value: number): HTMLElement {
    return make("li", {}, make("span", { i18n: key }), `: ${value}`);
  }

  /**
   * List what an accepted file changed.
   *
   * `missing` and `untranslated` are reported rather than hidden: both are
   * legitimate, and both are things a translator usually wants to know about
   * their own file — an entry left out is one still showing English text.
   *
   * The third count is labelled as entries identical to the English original,
   * not as untranslated ones, because it cannot tell the two apart: German
   * "Version" and "Region" are the German words, and a label calling them
   * untranslated reports a defect that is not there.
   */
  function countList(result: CatalogReport): HTMLElement {
    return make(
      "ul",
      { className: "catalog-counts" },
      countItem("catalog-count-accepted", result.accepted),
      countItem("catalog-count-missing", result.missing),
      countItem("catalog-count-untranslated", result.untranslated),
    );
  }

  /**
   * List why a file was refused, one line of the file per entry.
   *
   * The line number comes first because the only useful next step is to open
   * the file at that spot. The detail is appended only when it adds something
   * to the message id, which for most kinds of mistake it does not.
   */
  function problemList(result: CatalogReport): HTMLElement {
    const list = make("ul", { className: "catalog-problems" });
    for (const problem of result.problems) {
      const item = make(
        "li",
        {},
        make("span", { i18n: "catalog-line" }),
        ` ${problem.line}, ${problem.key}: `,
        make("span", { i18n: `catalog-problem-${problem.kind}` }),
      );
      if (problem.detail !== problem.key) item.append(` ${problem.detail}`);
      list.append(item);
    }
    return list;
  }

  /**
   * Re-read the catalogues the window is currently translating from.
   *
   * Only needed when the edited language is on screen, or is `en-US`, which
   * every other language falls back to for entries it does not define.
   */
  async function reloadIfVisible(locale: Locale): Promise<void> {
    catalogs.delete(locale);
    if (locale === currentLocale || locale === FALLBACK) await applyLocale(currentLocale);
  }

  const download = make("button", { i18n: "btn-catalog-export" });
  download.addEventListener("click", async () => {
    const locale = chosen();
    download.disabled = true;
    try {
      // The backend opens the save dialog itself; a dismissed dialog is a
      // normal outcome, not something to report.
      const written = await invoke<string | null>("export_catalog", { locale });
      if (written === null) return;
      showDone(status, `${tr("catalog-exported")} ${written}`);
    } catch (e) {
      showError(status, e);
    } finally {
      download.disabled = false;
    }
  });

  const upload = make("button", { i18n: "btn-catalog-import" });
  upload.addEventListener("click", async () => {
    const locale = chosen();
    upload.disabled = true;
    try {
      const file = await invoke<string | null>("pick_file", { kind: "catalog" });
      if (file === null) return;
      const result = await invoke<CatalogReport>("import_catalog", { locale, file });
      if (result.problems.length > 0) {
        // Nothing was stored, so the previous state still holds and only the
        // list of things to fix has to be shown.
        showErrorKey(status, "catalog-rejected");
        report.replaceChildren(problemList(result));
        return;
      }
      await reloadIfVisible(locale);
      await refresh();
      report.replaceChildren(countList(result));
      showDone(status, tr("catalog-imported"));
    } catch (e) {
      showError(status, e);
    } finally {
      upload.disabled = false;
    }
  });

  const reset = make("button", { i18n: "btn-catalog-reset" });
  reset.addEventListener("click", async () => {
    const locale = chosen();
    if (!(await confirmed(tr("catalog-reset-confirm"), tr("btn-confirm-discard")))) return;
    reset.disabled = true;
    try {
      const removed = await invoke<boolean>("reset_catalog", { locale });
      await reloadIfVisible(locale);
      await refresh();
      showDone(status, tr(removed ? "catalog-reset-done" : "catalog-reset-none"));
    } catch (e) {
      showError(status, e);
    } finally {
      reset.disabled = false;
    }
  });

  select.addEventListener("change", () => {
    clearStatusState(status);
    status.textContent = "";
    refresh().catch((e) => showError(status, e));
  });
  refresh().catch((e) => showError(status, e));

  return card(
    make("h2", { i18n: "settings-catalog-heading" }),
    make("p", { i18n: "settings-catalog-hint" }),
    make(
      "div",
      { className: "row" },
      make("label", { i18n: "settings-catalog-locale" }),
      select,
      download,
      upload,
      reset,
    ),
    state,
    status,
    report,
  );
}
