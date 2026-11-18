/** Memory view: every value of one camera's settings store, with its names. */

import { invoke } from "@tauri-apps/api/core";
import type { DumpSummary, MemoryInfo, MemoryPage, MemoryRow } from "../dto";
import {
  accessBadge,
  cameraAnswers,
  card,
  clearStatusState,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  statusLine,
  told,
  withProgressTarget,
} from "../dom";
import { tr, trFill } from "../i18n";
import { fetchNames, fwNameLabel, tablesText } from "./names";

/** Rows fetched per page. */
const PAGE = 200;

type Source = "camera" | "file";

export function viewMemory(host: HTMLElement): void {
  let source: Source | null = null;
  let chosenFile: string | null = null;
  const status = statusLine();
  const result = make("div");
  const filePath = make("span", { className: "cmp-mono cmp-chosen" });
  const browse = make("button", { i18n: "compare-browse" });
  const filePanel = make("div", { className: "cmp-panel" }, make("div", { className: "row" }, browse, filePath));
  const open = make("button", { className: "primary", i18n: "btn-memory-open" });

  const apply = (): void => {
    filePanel.hidden = source !== "file";
    open.disabled = source === null || (source === "file" && chosenFile === null);
  };

  const radios = ([
    ["camera", "memory-source-camera", "memory-source-camera-hint"],
    ["file", "memory-source-file", "memory-source-file-hint"],
  ] as const).map(([value, labelKey, hintKey]) => {
    const radio = make("input", { type: "radio", name: "memory-source", value });
    radio.addEventListener("change", () => {
      if (!radio.checked) return;
      source = value;
      apply();
    });
    return make(
      "label",
      { className: "cmp-choice" },
      make("span", { className: "row cmp-choice-head" }, radio, make("span", { className: "cmp-choice-title", i18n: labelKey })),
      make("span", { className: "hint cmp-choice-hint", i18n: hintKey }),
    );
  });

  browse.addEventListener("click", async () => {
    try {
      const picked = await invoke<string | null>("pick_file", { kind: "dump" });
      if (picked !== null) {
        chosenFile = picked;
        filePath.textContent = picked;
        apply();
      }
    } catch (e) {
      showError(status, e);
    }
  });

  /** Read the camera into a dump in the default folder; its path, or null. */
  const readCamera = async (): Promise<string | null> => {
    if (!(await cameraAnswers())) {
      await told(tr("compare-camera-off"), tr("btn-ok"), true);
      return null;
    }
    const folder =
      (await invoke<string | null>("compare_default_folder")) ??
      (await invoke<string | null>("compare_pick_folder"));
    if (folder === null) return null;
    const summary = await withProgressTarget(status, () =>
      invoke<DumpSummary>("compare_dump_camera", {
        folder,
        keepImage: false,
        withSerial: false,
        withPhotographer: false,
        withCopyright: false,
      }),
    );
    return summary.path;
  };

  open.addEventListener("click", async () => {
    if (source === null) return;
    if (source === "file" && chosenFile === null) {
      showErrorKey(status, "compare-file-required");
      return;
    }
    open.disabled = true;
    result.replaceChildren();
    try {
      const path = source === "camera" ? await readCamera() : chosenFile;
      if (path === null) return;
      showBusy(status, tr("msg-working"));
      const info = await invoke<MemoryInfo>("memory_open", { path });
      if (source === "camera") showDone(status, trFill("compare-saved-to", { path }));
      else {
        clearStatusState(status);
        status.textContent = "";
      }
      result.append(...memoryResult(info));
    } catch (e) {
      showError(status, e);
    } finally {
      apply();
    }
  });

  apply();
  host.append(
    card(
      make("h2", { i18n: "memory-heading" }),
      make("p", {}, accessBadge("read"), " ", make("span", { i18n: "memory-intro" })),
      make("label", { className: "field-label", i18n: "compare-source-question" }),
      ...radios,
      filePanel,
      make("div", { className: "cmp-spaced" }, open),
      status,
    ),
    result,
  );
}

/** The camera, its names, the filters and the first page of rows. */
function memoryResult(info: MemoryInfo): Node[] {
  const c = info.camera;
  const title = [c.model ?? "?", c.region, c.firmware ? `FW ${c.firmware}` : null, c.build]
    .filter((part) => part)
    .join(" · ");
  const head: Node[] = [
    make("h2", { textContent: title }),
    make("p", { className: "hint cmp-mono", textContent: info.file }),
    make("p", { className: "hint", textContent: tablesText(info.tables) }),
  ];
  if (info.tables.length === 0) {
    head.push(make("p", { i18n: "memory-no-names" }), fetchNames(true));
  }

  const search = make("input", { type: "search", className: "path-input" });
  const onlyNamed = make("input", { type: "checkbox" });
  const onlyTouched = make("input", { type: "checkbox" });
  const onlyWritable = make("input", { type: "checkbox" });
  const further = make("input", { type: "checkbox" });
  const count = make("p", { className: "hint" });
  const body = make("tbody");
  const more = make("button", { i18n: "btn-memory-more" });
  const status = statusLine();
  let shown = 0;
  let generation = 0;

  const query = (offset: number) => ({
    text: search.value,
    onlyNamed: onlyNamed.checked,
    onlyTouched: onlyTouched.checked,
    onlyWritable: onlyWritable.checked,
    further: further.checked,
    offset,
    limit: PAGE,
  });

  const load = async (reset: boolean): Promise<void> => {
    const mine = reset ? ++generation : generation;
    try {
      const page = await invoke<MemoryPage>("memory_page", { query: query(reset ? 0 : shown) });
      if (mine !== generation) return;
      if (reset) {
        body.replaceChildren();
        shown = 0;
      }
      body.append(...page.rows.map(memoryRow));
      shown += page.rows.length;
      count.textContent = trFill("memory-count", { shown, total: page.total });
      more.hidden = shown >= page.total;
    } catch (e) {
      showError(status, e);
    }
  };

  let typing: number | undefined;
  search.addEventListener("input", () => {
    window.clearTimeout(typing);
    typing = window.setTimeout(() => void load(true), 250);
  });
  for (const box of [onlyNamed, onlyTouched, onlyWritable, further]) {
    box.addEventListener("change", () => void load(true));
  }
  more.addEventListener("click", () => void load(false));
  void load(true);

  const filter = (box: HTMLInputElement, key: string): HTMLElement =>
    make("label", { className: "row" }, box, make("span", { i18n: key }));
  const table = make(
    "table",
    { className: "analysis-table memory-table" },
    make(
      "thead",
      {},
      make(
        "tr",
        {},
        make("th", { i18n: "compare-col-id" }),
        make("th", { i18n: "compare-col-name" }),
        make("th", { i18n: "compare-col-size" }),
        make("th", { i18n: "memory-col-value" }),
        make("th", { i18n: "memory-col-attr" }),
      ),
    ),
    body,
  );
  const filters = [filter(onlyNamed, "memory-only-named"), filter(onlyTouched, "memory-only-touched"), filter(onlyWritable, "memory-only-writable")];
  if (info.further > 0) filters.push(filter(further, "memory-further"));

  return [
    card(
      ...head,
      make("label", { className: "field-label", i18n: "memory-search" }),
      search,
      make("div", { className: "row memory-filters" }, ...filters),
      count,
      table,
      make("div", { className: "row" }, more),
      status,
    ),
  ];
}

function memoryRow(row: MemoryRow): HTMLElement {
  const name = make("td", { textContent: row.name ?? "" });
  const fw = fwNameLabel(row.fwName);
  if (fw !== null) name.append(make("div", {}, fw));
  const value = make("td", {
    className: "cmp-mono",
    textContent: row.redacted ? "••" : row.truncated ? `${row.value} …` : row.value,
  });
  const attr = make("td", { className: "hint", textContent: row.attrNames.join(", ") });
  return make(
    "tr",
    { className: row.touched ? "memory-touched" : "" },
    make("td", { className: "cmp-mono", textContent: row.id }),
    name,
    make("td", { textContent: String(row.size) }),
    value,
    attr,
  );
}
