/** Memory view: every value of one camera's settings store, with its names and meanings. */

import { invoke } from "@tauri-apps/api/core";
import type {
  DumpSummary,
  MemoryDetail,
  MemoryGroup,
  MemoryInfo,
  MemoryPage,
  MemoryRow,
} from "../dto";
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
  showStatus,
  statusLine,
  told,
  withProgressTarget,
} from "../dom";
import { tr, trFill } from "../i18n";
import { fetchNames, fwNameLabel, tablesText } from "./names";
import { addReferenceEntry } from "./reference";

/** Rows fetched per page of a group. */
const PAGE = 200;

/** Groups opened by themselves when no more rows than this match. */
const AUTO_OPEN = 200;

type Source = "camera" | "file";

/** How a value is shown. */
type Display = "auto" | "hex" | "uint" | "int" | "bits" | "text" | "bcd";
const DISPLAYS: Display[] = ["auto", "hex", "uint", "int", "bits", "text", "bcd"];

/** What the memory view can ask of the page around it. */
export interface MemoryActions {
  /** Open the reference check, where a finding was just added. */
  openReference: () => void;
}

/** Read a camera into a dump in the default folder; its path, or null. */
async function readCamera(status: HTMLElement): Promise<string | null> {
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
}

export function viewMemory(host: HTMLElement, actions: MemoryActions): void {
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

  /** Open `path`; with `after`, over the store on screen so changes show. */
  const show = async (path: string, after: boolean, fromCamera: boolean): Promise<void> => {
    showBusy(status, tr("msg-working"));
    const info = await invoke<MemoryInfo>("memory_open", { path, after });
    if (fromCamera) showDone(status, trFill("compare-saved-to", { path }));
    else {
      clearStatusState(status);
      status.textContent = "";
    }
    result.replaceChildren(...memoryResult(info, actions, show));
  };

  open.addEventListener("click", async () => {
    if (source === null) return;
    if (source === "file" && chosenFile === null) {
      showErrorKey(status, "compare-file-required");
      return;
    }
    open.disabled = true;
    try {
      const path = source === "camera" ? await readCamera(status) : chosenFile;
      if (path !== null) await show(path, false, source === "camera");
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

type Reopen = (path: string, after: boolean, fromCamera: boolean) => Promise<void>;

/** The camera, its names, the tools for the whole store, the filters and the groups. */
function memoryResult(info: MemoryInfo, actions: MemoryActions, reopen: Reopen): Node[] {
  const c = info.camera;
  const title = [c.model ?? "?", c.region, c.firmware ? `FW ${c.firmware}` : null, c.build]
    .filter((part) => part)
    .join(" · ");
  const status = statusLine();

  const head: Node[] = [
    make("h2", { textContent: title }),
    make("p", { className: "hint cmp-mono", textContent: info.file }),
    make("p", { className: "hint", textContent: tablesText(info.tables) }),
  ];
  if (info.own > 0) head.push(make("p", { className: "hint", textContent: trFill("memory-own-count", { count: info.own }) }));
  if (info.tables.length === 0) head.push(make("p", { i18n: "memory-no-names" }), fetchNames(true));
  if (info.changed !== null) {
    head.push(make("p", { className: "memory-changed-note", textContent: trFill("memory-changed-count", { count: info.changed }) }));
    if (info.sameCamera === false) head.push(make("p", { className: "warning", i18n: "memory-other-camera" }));
  }

  const reread = make("button", { i18n: "btn-memory-reread" });
  reread.addEventListener("click", async () => {
    reread.disabled = true;
    try {
      const path = await readCamera(status);
      if (path !== null) await reopen(path, true, true);
    } catch (e) {
      showError(status, e);
    } finally {
      reread.disabled = false;
    }
  });
  const reopenFile = make("button", { i18n: "btn-memory-reopen" });
  reopenFile.addEventListener("click", async () => {
    try {
      const path = await invoke<string | null>("pick_file", { kind: "dump" });
      if (path !== null) await reopen(path, true, false);
    } catch (e) {
      showError(status, e);
    }
  });

  const search = make("input", { type: "search", className: "path-input" });
  const display = make("select", { className: "field-select" });
  for (const d of DISPLAYS) display.append(make("option", { value: d, i18n: `memory-display-${d}` }));
  const hideEmpty = make("input", { type: "checkbox", checked: true });
  const onlyNamed = make("input", { type: "checkbox" });
  const onlyTouched = make("input", { type: "checkbox" });
  const onlyWritable = make("input", { type: "checkbox" });
  const onlyChanged = make("input", { type: "checkbox", checked: info.changed !== null && info.changed > 0 });
  const further = make("input", { type: "checkbox" });
  const groups = make("div", { className: "memory-groups" });
  const count = make("p", { className: "hint" });

  const query = () => ({
    text: search.value,
    hideEmpty: hideEmpty.checked,
    onlyNamed: onlyNamed.checked,
    onlyTouched: onlyTouched.checked,
    onlyWritable: onlyWritable.checked,
    onlyChanged: onlyChanged.checked,
    further: further.checked,
  });
  const mode = (): Display => display.value as Display;

  let generation = 0;
  const reload = async (): Promise<void> => {
    const mine = ++generation;
    try {
      const list = await invoke<MemoryGroup[]>("memory_groups", { query: query() });
      if (mine !== generation) return;
      const total = list.reduce((sum, g) => sum + g.total, 0);
      count.textContent = trFill("memory-match-count", { total, groups: list.length });
      const openAll = list.length === 1 || total <= AUTO_OPEN;
      groups.replaceChildren(...list.map((g) => groupNode(g, query, mode, actions, openAll)));
    } catch (e) {
      showError(status, e);
    }
  };

  const csv = make("button", { i18n: "btn-memory-csv" });
  csv.addEventListener("click", async () => {
    try {
      const path = await invoke<string | null>("memory_export_csv", { query: query() });
      if (path !== null) showDone(status, trFill("compare-saved-to", { path }));
    } catch (e) {
      showError(status, e);
    }
  });
  const exportNames = make("button", { i18n: "btn-memory-export-names" });
  exportNames.addEventListener("click", async () => {
    try {
      const path = await invoke<string | null>("memory_export_names");
      if (path !== null) showDone(status, trFill("compare-saved-to", { path }));
    } catch (e) {
      showError(status, e);
    }
  });

  let typing: number | undefined;
  search.addEventListener("input", () => {
    window.clearTimeout(typing);
    typing = window.setTimeout(() => void reload(), 250);
  });
  for (const box of [hideEmpty, onlyNamed, onlyTouched, onlyWritable, onlyChanged, further]) {
    box.addEventListener("change", () => void reload());
  }
  display.addEventListener("change", () => void reload());
  void reload();

  const filter = (box: HTMLInputElement, key: string): HTMLElement =>
    make("label", { className: "row" }, box, make("span", { i18n: key }));
  const filters = [
    filter(hideEmpty, "memory-hide-empty"),
    filter(onlyNamed, "memory-only-named"),
    filter(onlyTouched, "memory-only-touched"),
    filter(onlyWritable, "memory-only-writable"),
  ];
  if (info.changed !== null) filters.push(filter(onlyChanged, "memory-only-changed"));
  if (info.further > 0) filters.push(filter(further, "memory-further"));

  return [
    card(
      ...head,
      make("div", { className: "row memory-actions" }, reread, reopenFile, csv, exportNames),
      status,
    ),
    card(
      make("label", { className: "field-label", i18n: "memory-search" }),
      search,
      make("label", { className: "field-label", i18n: "memory-display" }),
      display,
      make("div", { className: "row memory-filters" }, ...filters),
      count,
      groups,
    ),
  ];
}

/** One subsystem, folded; its rows are fetched when it is opened. */
function groupNode(
  group: MemoryGroup,
  query: () => Record<string, unknown>,
  mode: () => Display,
  actions: MemoryActions,
  open: boolean,
): HTMLElement {
  const summary = make(
    "summary",
    {},
    make("span", { className: "cmp-mono", textContent: group.key }),
    group.label ? ` · ${group.label}` : "",
    make("span", { className: "hint", textContent: ` — ${trFill("memory-group-count", { total: group.total, named: group.named })}` }),
  );
  if (group.changed > 0) {
    summary.append(" ", make("span", { className: "memory-changed-tag", textContent: trFill("memory-group-changed", { count: group.changed }) }));
  }
  const details = make("details", { className: "memory-group" }, summary);
  const body = make("tbody");
  const more = make("button", { i18n: "btn-memory-more" });
  const status = statusLine();
  let shown = 0;
  let loaded = false;

  const load = async (): Promise<void> => {
    try {
      const page = await invoke<MemoryPage>("memory_page", {
        query: { ...query(), group: group.key, offset: shown, limit: PAGE },
      });
      body.append(...page.rows.flatMap((row) => rowNodes(row, mode(), actions)));
      shown += page.rows.length;
      more.hidden = shown >= page.total;
    } catch (e) {
      showError(status, e);
    }
  };
  more.hidden = true;
  more.addEventListener("click", () => void load());
  details.addEventListener("toggle", () => {
    if (!details.open || loaded) return;
    loaded = true;
    void load();
  });

  details.append(
    make(
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
    ),
    make("div", { className: "row" }, more),
    status,
  );
  details.open = open;
  return details;
}

/** A row, and the folded detail row below it. */
function rowNodes(row: MemoryRow, mode: Display, actions: MemoryActions): HTMLElement[] {
  const name = make("td", {}, ...nameNodes(row));
  const bytes = bytesOf(row.value);
  const shown = row.redacted ? "••" : render(mode, bytes, row);
  const value = make("td", {
    className: "cmp-mono",
    textContent: row.truncated ? `${shown} …` : shown,
    title: row.redacted ? "" : row.value,
  });
  const tr_ = make(
    "tr",
    { className: ["memory-row", row.touched ? "memory-touched" : "", row.changed ? "memory-changed" : ""].join(" ").trim() },
    make("td", { className: "cmp-mono", textContent: row.id }),
    name,
    make("td", { textContent: String(row.size) }),
    value,
    make("td", { className: "hint", textContent: row.attrNames.map(attrText).join(", ") }),
  );
  const detailCell = make("td", { colSpan: 5 });
  const detailRow = make("tr", { className: "memory-detail", hidden: true }, detailCell);
  tr_.addEventListener("click", async () => {
    detailRow.hidden = !detailRow.hidden;
    if (detailRow.hidden || detailCell.childElementCount > 0) return;
    try {
      const detail = await invoke<MemoryDetail | null>("memory_detail", { id: row.id });
      if (detail !== null) detailCell.append(...detailNodes(detail, actions));
    } catch (e) {
      const s = statusLine();
      detailCell.append(s);
      showError(s, e);
    }
  });
  return [tr_, detailRow];
}

/** The toolkit's name, the firmware's name and the user's own, as far as known. */
function nameNodes(row: MemoryRow): Node[] {
  const nodes: Node[] = [];
  if (row.name) nodes.push(make("div", { textContent: row.name }));
  const fw = fwNameLabel(row.fwName);
  if (fw !== null) nodes.push(make("div", {}, fw));
  if (row.own) {
    nodes.push(make("div", { className: "memory-own", textContent: row.own.name, title: row.own.note }));
  }
  return nodes;
}

/** Everything about one property, its own name to edit, and what to do next. */
function detailNodes(d: MemoryDetail, actions: MemoryActions): Node[] {
  const bytes = bytesOf(d.hex);
  const status = statusLine();
  const facts: [string, string][] = [
    [tr("memory-detail-offset"), `${d.offset} · ${trFill("memory-detail-size", { size: d.row.size, max: d.maxSize })}`],
  ];
  if (d.reset !== null) facts.push([tr("memory-detail-factory"), d.reset]);
  if (d.before !== null) facts.push([tr("memory-detail-before"), d.before]);
  if (d.row.attrNames.length > 0) facts.push([tr("memory-col-attr"), d.row.attrNames.map(attrText).join(", ")]);
  if (d.row.fwName) {
    const fw = d.row.fwName;
    const origin = fw.conflict ? tr("names-conflict") : tr(`names-evidence-${fw.evidence}`);
    facts.push([tr("memory-detail-fw"), `${fw.name} · ${origin} · ${fw.table}`]);
  }
  const factTable = make(
    "table",
    { className: "analysis-table" },
    ...facts.map(([k, v]) => make("tr", {}, make("th", { textContent: k }), make("td", { className: "cmp-mono", textContent: v }))),
  );
  const views = make(
    "table",
    { className: "analysis-table" },
    ...DISPLAYS.filter((m) => m !== "auto" && m !== "hex").map((m) =>
      make("tr", {}, make("th", { i18n: `memory-display-${m}` }), make("td", { className: "cmp-mono", textContent: render(m, bytes, d.row, true) })),
    ),
  );

  const nodes: Node[] = [
    make("pre", { className: "shell-output memory-dump", textContent: d.row.redacted ? "••" : hexDump(bytes) }),
    make("h4", { i18n: "memory-detail-views" }),
    views,
    factTable,
  ];

  const copy = make("button", { i18n: "btn-memory-copy-id" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(d.row.id);
      showDone(status, tr("msg-url-copied"));
    } catch (e) {
      showError(status, e);
    }
  });
  const buttons: HTMLElement[] = [copy];

  // Own names and findings belong to the first image, the one that is written.
  if (!d.row.id.includes(":")) {
    const ownName = make("input", { type: "text", className: "path-input", value: d.row.own?.name ?? "" });
    const ownNote = make("input", { type: "text", className: "path-input", value: d.row.own?.note ?? "" });
    const save = make("button", { i18n: "btn-memory-own-save" });
    save.addEventListener("click", async () => {
      try {
        await invoke("memory_set_own", { id: d.row.id, name: ownName.value, note: ownNote.value });
        showDone(status, tr("memory-own-saved"));
      } catch (e) {
        showError(status, e);
      }
    });
    nodes.push(
      make("h4", { i18n: "memory-own-heading" }),
      make("label", { className: "field-label", i18n: "memory-own-name" }),
      ownName,
      make("label", { className: "field-label", i18n: "memory-own-note" }),
      ownNote,
      make("div", { className: "row" }, save),
    );

    const check = make("button", { i18n: "btn-memory-check-elsewhere" });
    check.addEventListener("click", () => {
      const label = d.row.own?.name || d.row.name || d.row.fwName?.name || d.row.id;
      addReferenceEntry({ id: d.row.id, label, variants: [{ label: tr("memory-current-value"), hex: d.hex }] });
      actions.openReference();
    });
    buttons.push(check);
  }
  nodes.push(make("div", { className: "row" }, ...buttons), status);
  if (d.row.redacted) showStatus(status, tr("memory-redacted"));
  return nodes;
}

/** Attribute bits in words. */
function attrText(name: string): string {
  const key = name.startsWith("HAS_RESET") ? "has-reset" : name.toLowerCase().replace(/_/g, "-");
  const text = tr(`memory-attr-${key}`);
  return text === `memory-attr-${key}` ? name : text;
}

function bytesOf(hex: string): number[] {
  return hex
    .split(/\s+/)
    .filter((b) => b !== "")
    .map((b) => parseInt(b, 16));
}

/** Little-endian integer of 1 to 8 bytes. */
function integer(bytes: number[], signed: boolean): bigint {
  let value = 0n;
  bytes.forEach((b, i) => {
    value |= BigInt(b) << BigInt(8 * i);
  });
  const bits = BigInt(8 * bytes.length);
  if (signed && bytes.length > 0 && (bytes[bytes.length - 1] & 0x80) !== 0) value -= 1n << bits;
  return value;
}

function asNumber(bytes: number[], signed: boolean): string {
  if ([1, 2, 4, 8].includes(bytes.length)) return integer(bytes, signed).toString();
  return bytes.map((b) => (signed && b > 127 ? b - 256 : b).toString()).join(" ");
}

function asText(bytes: number[]): string {
  let end = bytes.length;
  while (end > 0 && bytes[end - 1] === 0) end--;
  const text = bytes
    .slice(0, end)
    .map((b) => (b >= 0x20 && b < 0x7f) || b >= 0xa0 ? String.fromCharCode(b) : "·")
    .join("");
  return `“${text}”`;
}

function asBcd(bytes: number[]): string {
  return bytes.map((b) => [b >> 4, b & 15].map((n) => (n > 9 ? "?" : String(n))).join("")).join(" ");
}

/** Whether the bytes read as text: printable, then nothing but zeros. */
function looksLikeText(bytes: number[]): boolean {
  let end = bytes.length;
  while (end > 0 && bytes[end - 1] === 0) end--;
  if (end < 3) return false;
  return bytes.slice(0, end).every((b) => b >= 0x20 && b < 0x7f);
}

/** A value in one display mode; `full` shows long values whole. */
function render(mode: Display, bytes: number[], row: MemoryRow, full = false): string {
  const cut = full ? bytes : bytes.slice(0, 16);
  const more = !full && bytes.length > 16 ? " …" : "";
  switch (mode) {
    case "hex":
      return row.value === "" ? "–" : bytes.map((b) => b.toString(16).padStart(2, "0")).join(" ");
    case "uint":
      return asNumber(cut, false) + more;
    case "int":
      return asNumber(cut, true) + more;
    case "bits":
      return cut.map((b) => b.toString(2).padStart(8, "0")).join(" ") + more;
    case "text":
      return asText(bytes);
    case "bcd":
      return asBcd(cut) + more;
    case "auto":
      if (row.meaning) return meaningText(row.meaning);
      if (looksLikeText(bytes)) return asText(bytes);
      if ([1, 2, 4].includes(bytes.length)) return asNumber(bytes, false);
      return render("hex", bytes, row);
  }
}

function meaningText(meaning: string): string {
  if (meaning === "on") return tr("memory-meaning-on");
  if (meaning === "off") return tr("memory-meaning-off");
  return meaning;
}

/** 16 bytes a line: offset, hex, and the bytes as text. */
function hexDump(bytes: number[]): string {
  const lines: string[] = [];
  for (let at = 0; at < bytes.length; at += 16) {
    const line = bytes.slice(at, at + 16);
    const hex = line.map((b) => b.toString(16).padStart(2, "0")).join(" ").padEnd(47, " ");
    const text = line.map((b) => (b >= 0x20 && b < 0x7f ? String.fromCharCode(b) : ".")).join("");
    lines.push(`${at.toString(16).padStart(4, "0")}  ${hex}  ${text}`);
  }
  return lines.join("\n");
}
