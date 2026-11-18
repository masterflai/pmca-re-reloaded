/** Firmware names that plugins bring: how they are shown, and where more are fetched. */

import { invoke } from "@tauri-apps/api/core";
import { make } from "../dom";
import type { FwName, NameTableInfo } from "../dto";
import { tr, trFill } from "../i18n";
import { pluginSources } from "./plugins";

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

/** Tables as one line: `plugin/table (977)`, or that there are none. */
export function tablesText(tables: NameTableInfo[]): string {
  if (tables.length === 0) return tr("names-line-none");
  const list = tables.map((t) => `${t.plugin}/${t.id} (${t.entries})`).join(", ");
  return trFill("names-line", { tables: list });
}

/** The installed name tables in one line, filled in once the backend answers. */
export function namesLine(): HTMLElement {
  const line = make("p", { className: "hint" });
  invoke<NameTableInfo[]>("names_tables", { model: null })
    .then((tables) => {
      line.textContent = tablesText(tables);
    })
    .catch(() => line.remove());
  return line;
}

/** A fold-away list of the plugins the user's sources offer, for more name tables. */
export function fetchNames(open = false): HTMLDetailsElement {
  const details = make(
    "details",
    { className: "analysis-info", open },
    make("summary", { i18n: "names-fetch" }),
    make("p", { className: "hint", i18n: "names-fetch-hint" }),
  );
  let filled = false;
  const fill = (): void => {
    if (filled || !details.open) return;
    filled = true;
    details.append(...pluginSources(() => undefined));
  };
  details.addEventListener("toggle", fill);
  fill();
  return details;
}
