/** Firmware names that plugins bring: how they are shown, and the plugins tab of the analysis page. */

import { invoke } from "@tauri-apps/api/core";
import { card, clearStatusState, make, showError, showStatus, statusLine } from "../dom";
import type { FwName, NameHit, NameTableInfo } from "../dto";
import { tr, trFill } from "../i18n";
import { cardPlugins } from "./plugins";

/** How a name is known, in the window's language. */
function evidenceText(fw: FwName): string {
  return fw.conflict ? tr("names-conflict") : tr(`names-evidence-${fw.evidence}`);
}

/**
 * A firmware name beside a property, with where it comes from on hover.
 *
 * Set apart from the toolkit's own names: it is only shown, never acted on.
 */
export function fwNameLabel(fw: FwName | undefined): HTMLElement | null {
  if (fw === undefined) return null;
  return make("span", {
    className: fw.conflict ? "fw-name conflict" : "fw-name",
    textContent: fw.name,
    title: `${evidenceText(fw)} · ${fw.table}`,
  });
}

/** The analysis page's plugins tab: the name tables, a search, and the plugins themselves. */
export function viewNames(host: HTMLElement): void {
  host.append(cardNames(), cardPlugins());
}

function cardNames(): HTMLElement {
  const tables = make("div");
  const status = statusLine();
  const results = make("div");
  const text = make("input", { type: "text", className: "path-input" });
  const model = make("input", { type: "text", className: "path-input", placeholder: "ILCE-6000" });
  const search = make("button", { className: "primary", i18n: "btn-names-search" });

  const run = async (): Promise<void> => {
    results.replaceChildren();
    clearStatusState(status);
    status.textContent = "";
    if (text.value.trim() === "") return;
    search.disabled = true;
    try {
      const hits = await invoke<NameHit[]>("names_search", { text: text.value, model: model.value });
      if (hits.length === 0) showStatus(status, tr("plugin-names-no-match"));
      else {
        showStatus(status, trFill("names-hits", { count: hits.length }));
        results.append(hitTable(hits));
      }
    } catch (e) {
      showError(status, e);
    } finally {
      search.disabled = false;
    }
  };
  search.addEventListener("click", () => void run());
  text.addEventListener("keydown", (event) => {
    if (event.key === "Enter") void run();
  });

  invoke<NameTableInfo[]>("names_tables", { model: null })
    .then((list) => {
      tables.replaceChildren(
        list.length === 0 ? make("p", { className: "hint", i18n: "plugin-names-none" }) : tableList(list),
      );
      search.disabled = list.length === 0;
    })
    .catch((e) => showError(status, e));

  return card(
    make("h2", { i18n: "names-heading" }),
    make("p", { i18n: "names-intro" }),
    tables,
    make("h3", { i18n: "names-search-heading" }),
    make("label", { className: "field-label", i18n: "names-search-label" }),
    text,
    make("label", { className: "field-label", i18n: "names-model-label" }),
    model,
    make("p", { className: "hint", i18n: "names-model-hint" }),
    make("div", { className: "row" }, search),
    status,
    results,
  );
}

function tableList(list: NameTableInfo[]): HTMLElement {
  const head = make(
    "tr",
    {},
    make("th", { i18n: "names-col-table" }),
    make("th", { i18n: "names-col-entries" }),
    make("th", { i18n: "names-col-scope" }),
    make("th", { i18n: "names-col-source" }),
  );
  const rows = list.map((t) =>
    make(
      "tr",
      {},
      make("td", { className: "cmp-mono", textContent: `${t.plugin}/${t.id}` }),
      make("td", { textContent: String(t.entries) }),
      make("td", { textContent: t.scope }),
      make("td", { textContent: t.source }),
    ),
  );
  return make("table", { className: "analysis-table" }, make("thead", {}, head), make("tbody", {}, ...rows));
}

function hitTable(hits: NameHit[]): HTMLElement {
  const head = make(
    "tr",
    {},
    make("th", { i18n: "compare-col-id" }),
    make("th", { i18n: "compare-col-name" }),
    make("th", { i18n: "names-col-evidence" }),
    make("th", { i18n: "names-col-table" }),
  );
  const rows = hits.map((hit) =>
    make(
      "tr",
      {},
      make("td", { className: "cmp-mono", textContent: hit.id }),
      make("td", { className: hit.fw.conflict ? "cmp-mono warning" : "cmp-mono", textContent: hit.fw.name }),
      make("td", { textContent: evidenceText(hit.fw) }),
      make("td", { className: "cmp-mono", textContent: hit.fw.table }),
    ),
  );
  return make("table", { className: "analysis-table" }, make("thead", {}, head), make("tbody", {}, ...rows));
}
