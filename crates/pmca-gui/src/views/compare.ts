/** Compare page: collect dumps of settings memories, then lay them side by side. */

import { invoke } from "@tauri-apps/api/core";
import type { CompareReport, CompareRow, CompareSource, DumpSummary } from "../dto";
import {
  card,
  fieldList,
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

/** The most dumps one comparison takes; mirrors `MAX_SOURCES` in `pmca-app`. */
const MAX_SOURCES = 8;

/** The most rows drawn at once. Beyond this the table stops being usable. */
const MAX_ROWS = 500;

/** The seven findings, in the order they are shown. */
const CLASS_ORDER = [
  "identical",
  "similar",
  "different",
  "structural",
  "empty",
  "absent",
  "noise",
] as const;
type Cls = (typeof CLASS_ORDER)[number];

/** Visible by default: the findings worth a first look. */
const DEFAULT_VISIBLE: Record<Cls, boolean> = {
  identical: false,
  similar: true,
  different: true,
  structural: true,
  empty: false,
  absent: true,
  noise: false,
};

/** One dump the user has collected, with what it turned out to hold. */
interface Entry {
  path: string;
  summary: DumpSummary;
  /** The user's own name for this camera; becomes its column heading. */
  label: string;
  /** Which of the three ways it was added, for the line under its name. */
  sourceKey: "compare-source-camera" | "compare-source-image" | "compare-source-dump";
}

/** Which of the three ways the add form is currently set to. */
type Source = "camera" | "image" | "dump";

export function viewCompare(host: HTMLElement): void {
  const entries: Entry[] = [];
  let want = 2;
  let folder: string | null = null;
  let report: CompareReport | null = null;
  /** Nothing preselected: the three ways stay a question until one is picked. */
  let source: Source | null = null;
  let chosenFile: string | null = null;

  const status = statusLine();
  const list = make("div", { className: "cmp-list" });
  const progress = make("span", { className: "cmp-progress" });
  const results = make("div");
  const visible = new Set<Cls>(CLASS_ORDER.filter((cls) => DEFAULT_VISIBLE[cls]));

  const distance = make("input", {
    type: "number",
    min: "0",
    max: "16",
    value: "1",
    className: "cmp-number",
  });
  const autoNoise = make("input", { type: "checkbox", checked: true });
  const keepImage = make("input", { type: "checkbox", checked: true });
  const withSerial = make("input", { type: "checkbox" });
  const withPhotographer = make("input", { type: "checkbox" });
  const withCopyright = make("input", { type: "checkbox" });
  const runButton = make("button", { className: "primary", i18n: "compare-run" });
  const addButton = make("button", { className: "primary", i18n: "compare-add-button" });
  const labelField = make("input", { type: "text", className: "path-input" });
  const filePath = make("span", { className: "cmp-mono cmp-chosen" });
  const browse = make("button", { i18n: "compare-browse" });
  const folderPath = make("span", { className: "cmp-mono cmp-chosen" });
  const folderButton = make("button", { i18n: "compare-folder-change" });
  const countField = make("input", {
    type: "number",
    min: "2",
    max: String(MAX_SOURCES),
    value: "2",
    className: "cmp-number",
  });

  /** Only what the chosen way needs is on screen; the rest is put away. */
  const filePanel = make(
    "div",
    { className: "cmp-panel" },
    make("div", { className: "row" }, browse, filePath),
  );
  const privacyPanel = make(
    "div",
    { className: "cmp-panel" },
    checkboxRow(withSerial, "compare-with-serial", null),
    checkboxRow(withPhotographer, "compare-with-photographer", null),
    checkboxRow(withCopyright, "compare-with-copyright", null),
    make("p", { className: "hint", i18n: "compare-privacy-hint" }),
  );
  const keepPanel = make(
    "div",
    { className: "cmp-panel" },
    checkboxRow(keepImage, "compare-keep-image", "compare-keep-image-hint"),
  );
  const folderPanel = make(
    "div",
    { className: "cmp-panel" },
    make("label", { className: "field-label", i18n: "compare-folder-label" }),
    make("div", { className: "row" }, folderButton, folderPath),
    make("p", { className: "hint", i18n: "compare-folder-hint" }),
  );
  /** Set apart from the three choices: these belong to the chosen one. */
  const panels = make("div", { className: "cmp-panels" }, filePanel, keepPanel, privacyPanel, folderPanel);

  const applySource = (): void => {
    filePanel.hidden = source !== "image" && source !== "dump";
    keepPanel.hidden = source !== "camera";
    // A finished dump is what it is; nothing is written, so nothing to decide.
    privacyPanel.hidden = source !== "camera" && source !== "image";
    folderPanel.hidden = source !== "camera" && source !== "image";
    panels.hidden = source === null;
    if (source !== "image" && source !== "dump") chosenFile = null;
    filePath.textContent = chosenFile ?? "";
  };

  const sourceInputs: HTMLInputElement[] = [];
  const radios = ([
    ["camera", "compare-add-camera", "compare-add-camera-hint"],
    ["image", "compare-add-image", "compare-add-image-hint"],
    ["dump", "compare-add-dump", "compare-add-dump-hint"],
  ] as const).map(([value, labelKey, hintKey]) => {
    const radio = make("input", { type: "radio", name: "cmp-source", value });
    radio.addEventListener("change", () => {
      if (!radio.checked) return;
      source = value;
      chosenFile = null;
      applySource();
      refresh();
    });
    sourceInputs.push(radio);
    return make(
      "label",
      { className: "cmp-choice" },
      make(
        "span",
        { className: "row cmp-choice-head" },
        radio,
        make("span", { className: "cmp-choice-title", i18n: labelKey }),
      ),
      make("span", { className: "hint cmp-choice-hint", i18n: hintKey }),
    );
  });

  browse.addEventListener("click", async () => {
    try {
      const kind = source === "image" ? "backup" : "dump";
      const picked = await invoke<string | null>("pick_file", { kind });
      if (picked !== null) {
        chosenFile = picked;
        filePath.textContent = picked;
      }
    } catch (e) {
      showError(status, e);
    }
  });

  const drawResults = (): void => {
    results.replaceChildren(
      ...(report === null ? [] : renderReport(report, entries, visible, drawResults, status)),
    );
  };

  /** The name the next camera is offered, counting on from what is there. */
  const nextLabel = (): string => trFill("compare-label-default", { n: entries.length + 1 });

  const refresh = (): void => {
    progress.textContent = trFill("compare-list-progress", {
      have: entries.length,
      want: Math.max(want, entries.length),
    });
    list.replaceChildren(
      ...(entries.length === 0
        ? [make("p", { className: "hint", i18n: "compare-list-empty" })]
        : entries.map((entry, index) => entryRow(entry, index, entries, refresh))),
    );
    if (entries.length === 1) {
      list.append(make("p", { className: "hint", i18n: "compare-list-missing" }));
    }
    runButton.disabled = entries.length < 2;
    addButton.disabled = source === null || entries.length >= MAX_SOURCES;
  };

  /** Take a finished dump into the list and ready the form for the next one. */
  const accept = (summary: DumpSummary, path: string, sourceKey: Entry["sourceKey"]): void => {
    const typed = labelField.value.trim();
    entries.push({ path, summary, label: typed === "" ? describe(summary) : typed, sourceKey });
    if (entries.length > want) want = Math.min(entries.length, MAX_SOURCES);
    countField.value = String(want);
    chosenFile = null;
    labelField.value = nextLabel();
    source = null;
    for (const radio of sourceInputs) radio.checked = false;
    applySource();
    refresh();
  };

  countField.addEventListener("change", () => {
    want = Math.min(MAX_SOURCES, Math.max(2, countField.valueAsNumber || 2));
    countField.value = String(want);
    refresh();
  });

  /** The folder every dump of this session is written to, asked for once. */
  const folderFor = async (): Promise<string | null> => {
    if (folder !== null) return folder;
    folder = await invoke<string | null>("compare_pick_folder");
    folderPath.textContent = folder ?? "";
    return folder;
  };

  folderButton.addEventListener("click", async () => {
    try {
      const picked = await invoke<string | null>("compare_pick_folder");
      if (picked !== null) {
        folder = picked;
        folderPath.textContent = picked;
      }
    } catch (e) {
      showError(status, e);
    }
  });

  // Offered up front so that adding a camera never opens a dialog unasked.
  void invoke<string | null>("compare_default_folder").then((dir) => {
    if (folder === null && dir !== null) {
      folder = dir;
      folderPath.textContent = dir;
    }
  });

  addButton.addEventListener("click", async () => {
    if (source === null) return;
    if (source !== "camera" && chosenFile === null) {
      showErrorKey(status, "compare-file-required");
      return;
    }
    // Said before the read starts, because the read itself takes minutes and
    // its failure would not name the usual cause: a camera that is still off.
    if (source === "camera" && !(await cameraAnswers())) {
      await told(tr("compare-camera-off"), tr("btn-ok"), true);
      return;
    }
    addButton.disabled = true;
    try {
      if (source === "dump") {
        const path = chosenFile as string;
        showBusy(status, tr("msg-working"));
        const summary = await invoke<DumpSummary>("compare_inspect", { path });
        accept(summary, path, "compare-source-dump");
        showDone(status, tr("msg-done"));
        return;
      }
      const target = await folderFor();
      if (target === null) return;
      const privacy = {
        withSerial: withSerial.checked,
        withPhotographer: withPhotographer.checked,
        withCopyright: withCopyright.checked,
      };
      const summary =
        source === "camera"
          ? await withProgressTarget(status, () =>
              invoke<DumpSummary>("compare_dump_camera", {
                folder: target,
                keepImage: keepImage.checked,
                ...privacy,
              }),
            )
          : await (async () => {
              showBusy(status, tr("msg-working"));
              return invoke<DumpSummary>("compare_dump", {
                input: chosenFile,
                folder: target,
                ...privacy,
              });
            })();
      accept(
        summary,
        summary.path,
        source === "camera" ? "compare-source-camera" : "compare-source-image",
      );
      showDone(status, trFill("compare-saved-to", { path: summary.path }));
    } catch (e) {
      showError(status, e);
    } finally {
      refresh();
    }
  });

  runButton.addEventListener("click", async () => {
    if (entries.length < 2) {
      showErrorKey(status, "compare-needs-two");
      return;
    }
    runButton.disabled = true;
    showBusy(status, tr("msg-working"));
    try {
      report = await invoke<CompareReport>("compare_run", {
        paths: entries.map((entry) => entry.path),
        similarDistance: Math.min(16, Math.max(0, distance.valueAsNumber || 0)),
        autoNoise: autoNoise.checked,
      });
      showDone(status, tr("msg-done"));
      drawResults();
    } catch (e) {
      showError(status, e);
    } finally {
      refresh();
    }
  });

  labelField.value = nextLabel();
  applySource();
  refresh();
  host.append(
    card(
      make("h2", { i18n: "nav-compare" }),
      make("p", { i18n: "compare-hint" }),
      explanation(),
    ),
    card(
      make("p", {
        className: "field-label",
        textContent: trFill("compare-count-label", { max: MAX_SOURCES }),
      }),
      stepper(countField),

      rule(),

      make("div", { className: "cmp-head" }, make("h3", { i18n: "compare-list-heading" }), progress),
      list,

      make("h3", { i18n: "compare-add-heading" }),
      make("label", { className: "field-label", i18n: "compare-label-field" }),
      labelField,
      make("p", { className: "hint", i18n: "compare-label-hint" }),
      make("label", { className: "field-label", i18n: "compare-source-question" }),
      ...radios,
      panels,
      make("div", { className: "cmp-spaced" }, addButton),

      rule(),

      make("label", { className: "field-label", i18n: "compare-similar-distance" }),
      stepper(distance),
      make("div", { className: "cmp-spaced" }, checkboxRow(autoNoise, "compare-auto-noise", null)),
      tuningHelp(),

      make("div", { className: "cmp-spaced" }, runButton),
      status,
      results,
    ),
  );
}

/** A short divider: it groups the page without cutting it in two. */
function rule(): HTMLElement {
  return make("div", { className: "cmp-rule" });
}

/**
 * A number field with its own minus and plus buttons.
 *
 * The native spinner is a few pixels tall and only appears on hover, which
 * rules it out for touch; these buttons are always there and finger-sized.
 */
function stepper(field: HTMLInputElement): HTMLElement {
  const step = (by: number): void => {
    const low = Number(field.min);
    const high = Number(field.max);
    const next = Math.min(high, Math.max(low, (field.valueAsNumber || low) + by));
    if (next === field.valueAsNumber) return;
    field.value = String(next);
    field.dispatchEvent(new Event("change"));
  };
  const button = (label: string, by: number): HTMLButtonElement => {
    const node = make("button", { className: "cmp-step", type: "button", textContent: label });
    node.addEventListener("click", () => step(by));
    return node;
  };
  return make("div", { className: "cmp-stepper" }, button("\u2212", -1), field, button("+", 1));
}

/** Whether a camera is on the bus. Opens no device, so it costs nothing. */
async function cameraAnswers(): Promise<boolean> {
  try {
    return (await invoke<string>("camera_presence")) === "connected";
  } catch {
    // Unanswerable here; let the read itself report what went wrong.
    return true;
  }
}

/** Model, region and firmware: what a dump says about where it came from. */
function describe(summary: DumpSummary): string {
  const camera = summary.camera;
  return [camera.model ?? "?", camera.region, camera.firmware]
    .filter((part) => part)
    .join(" · ");
}

// ---------------------------------------------------------------------------
// Explanations
// ---------------------------------------------------------------------------

/** Purpose, procedure and risks, folded shut, as the analysis page has it. */
function explanation(): HTMLDetailsElement {
  const details = make("details", {}, make("summary", { i18n: "compare-explanation-toggle" }));
  const sections: [string, string][] = [
    ["compare-purpose-heading", "compare-purpose"],
    ["compare-how-heading", "compare-how"],
    ["compare-reads-heading", "compare-reads"],
    ["compare-limits-heading", "compare-limits"],
    ["compare-risk-heading", "compare-risk"],
  ];
  for (const [heading, body] of sections) {
    details.append(make("h3", { i18n: heading }), make("p", { i18n: body }));
  }
  // The page a backup is made on, named through the navigation key rather than
  // spelled out: it is called something different in every language.
  details.append(
    make(
      "p",
      { className: "hint" },
      make("strong", { i18n: "nav-backup" }),
      " — ",
      make("span", { i18n: "welcome-where-backup" }),
    ),
  );
  return details;
}

/** What the two tuning controls above it actually do. */
function tuningHelp(): HTMLDetailsElement {
  const details = make("details", {}, make("summary", { i18n: "compare-tuning-toggle" }));
  details.append(
    make("h3", { i18n: "compare-distance-heading" }),
    make("p", { i18n: "compare-distance-help" }),
    make("h3", { i18n: "compare-noise-heading" }),
    make("p", { i18n: "compare-noise-help" }),
  );
  return details;
}

/** All seven findings plus the two marks, in full. */
function findingsHelp(): HTMLDetailsElement {
  const details = make("details", {}, make("summary", { i18n: "compare-findings-toggle" }));
  for (const cls of CLASS_ORDER) {
    details.append(
      make("h3", { i18n: `compare-class-${cls}` }),
      make("p", { i18n: `compare-class-${cls}-help` }),
    );
  }
  details.append(
    make("h3", { i18n: "compare-shape-heading" }),
    make("p", { i18n: "compare-shape-help" }),
  );
  return details;
}

// ---------------------------------------------------------------------------
// Collecting dumps
// ---------------------------------------------------------------------------

function checkboxRow(box: HTMLInputElement, labelKey: string, hintKey: string | null): HTMLElement {
  const row = make(
    "div",
    { className: "cmp-way" },
    make("label", { className: "row" }, box, make("span", { i18n: labelKey })),
  );
  if (hintKey !== null) row.append(make("p", { className: "hint", i18n: hintKey }));
  return row;
}

/** One collected dump: the name given to it, then what it actually is. */
function entryRow(entry: Entry, index: number, all: Entry[], refresh: () => void): HTMLElement {
  const remove = make("button", { i18n: "compare-remove" });
  remove.addEventListener("click", () => {
    all.splice(index, 1);
    refresh();
  });

  const layout = make("details", {}, make("summary", { i18n: "compare-layout-toggle" }));
  layout.append(layoutFields(entry.summary));

  const facts = [
    describe(entry.summary),
    trFill("compare-entry-values", { count: entry.summary.properties }),
  ].join(" · ");

  return make(
    "div",
    { className: "cmp-entry" },
    make(
      "div",
      { className: "row cmp-entry-head" },
      make("span", { className: "cmp-entry-title", textContent: entry.label }),
      remove,
    ),
    make("div", { className: "hint", textContent: facts }),
    make("div", { className: "hint", title: entry.path }, make("span", { i18n: entry.sourceKey })),
    layout,
  );
}

/** How the image an entry came from divides up. */
function layoutFields(summary: DumpSummary): HTMLElement {
  const free = summary.layout.free;
  const bytes = (n: number): string => `${n.toLocaleString()} B`;
  const block = make(
    "div",
    {},
    fieldList([
      ["compare-free-header", bytes(free.header)],
      ["compare-free-subsystem-table", bytes(free.subsystemTable)],
      ["compare-free-property-table", bytes(free.propertyTable)],
      ["compare-free-table-reserve", bytes(free.tableReserve)],
      ["compare-free-data-size", bytes(free.dataSize)],
      ["compare-free-data-used", bytes(free.dataUsed)],
      ["compare-free-data-free", bytes(free.dataFree)],
      ["compare-free-data-slack", bytes(free.dataSlack)],
      ["compare-free-largest-gap", bytes(free.largestGap)],
      ["compare-free-trailing", bytes(free.trailing)],
    ]),
    make("p", { className: "hint", i18n: "compare-gaps-note" }),
  );
  const subsystems = Object.entries(summary.layout.subsystems)
    .filter((pair): pair is [string, number] => pair[1] !== undefined)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 20);
  if (subsystems.length > 0) {
    block.append(
      make("h3", { i18n: "compare-subsystems" }),
      make(
        "table",
        { className: "analysis-table" },
        make(
          "tbody",
          {},
          ...subsystems.map(([id, count]) =>
            make(
              "tr",
              {},
              make("th", { className: "cmp-mono", textContent: id }),
              make("td", { textContent: String(count) }),
            ),
          ),
        ),
      ),
    );
  }
  if (summary.imagePath !== null) {
    block.append(
      make("p", {
        className: "hint",
        textContent: trFill("compare-image-kept", { path: summary.imagePath }),
      }),
    );
  }
  return block;
}

// ---------------------------------------------------------------------------
// The result
// ---------------------------------------------------------------------------

function renderReport(
  report: CompareReport,
  entries: Entry[],
  visible: Set<Cls>,
  onToggle: () => void,
  status: HTMLElement,
): Node[] {
  const rows = report.rows.filter((row) => visible.has(row.class as Cls));
  const shown = rows.slice(0, MAX_ROWS);
  const labels = entries.map((entry) => entry.label);

  const nodes: Node[] = [
    make("h3", { i18n: "compare-heading-cameras" }),
    sourcesTable(report.sources, labels),
    make("h3", { i18n: "compare-counts-heading" }),
    chipRow(report, visible, onToggle),
    make("p", { textContent: `${tr("compare-compared")}: ${report.total}` }),
  ];
  if (report.noiseIds.length > 0) {
    nodes.push(
      make("p", {
        textContent: trFill(
          report.noiseMeasured ? "compare-noise-measured" : "compare-noise-supplied",
          { count: report.noiseIds.length },
        ),
      }),
    );
  }
  nodes.push(findingsHelp());
  if (shown.length === 0) {
    nodes.push(make("p", { i18n: "compare-no-rows" }));
  } else {
    nodes.push(grid(report, shown, labels));
    if (shown.length < rows.length) {
      nodes.push(
        make("p", {
          className: "hint",
          textContent: trFill("compare-showing", { shown: shown.length, total: rows.length }),
        }),
      );
    }
  }
  nodes.push(exportSection(report, visible, status));
  return nodes;
}

function sourcesTable(sources: CompareSource[], labels: string[]): HTMLElement {
  return make(
    "table",
    { className: "analysis-table" },
    make(
      "tbody",
      {},
      ...sources.map((source, index) =>
        make(
          "tr",
          {},
          make("th", { textContent: labels[index] ?? `#${index + 1}` }),
          make("td", { textContent: source.camera.model ?? "?" }),
          make("td", { textContent: source.camera.region }),
          make("td", { textContent: source.camera.firmware ?? "" }),
          make("td", { className: "cmp-mono", textContent: source.file }),
        ),
      ),
    ),
  );
}

/** A column heading short enough not to stretch the table, in full on hover. */
function columnHead(label: string | undefined, index: number): HTMLElement {
  const text = label ?? `#${index + 1}`;
  return text.length > 16
    ? make("th", { textContent: `${text.slice(0, 16)}…`, title: text })
    : make("th", { textContent: text });
}

/** One toggle per finding, each with its count and its explanation on hover. */
function chipRow(report: CompareReport, visible: Set<Cls>, onToggle: () => void): HTMLElement {
  const row = make("div", { className: "row cmp-chips" });
  for (const cls of CLASS_ORDER) {
    const count = report.counts[cls] ?? 0;
    const chip = make(
      "button",
      {
        className: `cmp-chip${visible.has(cls) ? " on" : ""}`,
        title: tr(`compare-class-${cls}-help`),
      },
      make("span", { className: `cmp-dot cmp-${cls}` }),
      ` ${tr(`compare-class-${cls}`)} (${count})`,
    );
    // Only this finding is a judgement rather than a reading, so only this one
    // is marked as worth reading about before it is believed.
    if (cls === "empty") chip.append(make("span", { className: "cmp-query", textContent: " ?" }));
    chip.addEventListener("click", () => {
      if (visible.has(cls)) visible.delete(cls);
      else visible.add(cls);
      onToggle();
    });
    row.append(chip);
  }
  return row;
}

function grid(report: CompareReport, rows: CompareRow[], labels: string[]): HTMLElement {
  const head = make(
    "tr",
    {},
    make("th", { i18n: "compare-col-id" }),
    make("th", { i18n: "compare-col-size" }),
    make("th", { i18n: "compare-col-name" }),
    make("th", { i18n: "compare-col-class" }),
    ...report.sources.map((_, index) => columnHead(labels[index], index)),
  );
  const tbody = make("tbody", {});
  for (const row of rows) {
    const name = make("td", { textContent: row.name ?? "" });
    if (row.shape !== "plain") {
      const key = row.shape === "switch" ? "compare-shape-switch" : "compare-shape-choice";
      name.prepend(
        make("span", { className: "cmp-mark", title: tr("compare-shape-help"), i18n: key }),
        " ",
      );
    }
    tbody.append(
      make(
        "tr",
        { className: `cmp-${row.class}` },
        make("td", { className: "cmp-mono", textContent: row.id }),
        make("td", { textContent: row.size !== null ? String(row.size) : "–" }),
        name,
        make("td", { textContent: tr(`compare-class-${row.class}`) }),
        ...row.values.map((value) => valueCell(value)),
      ),
    );
  }
  return make("table", { className: "cmp-grid" }, make("thead", {}, head), tbody);
}

function valueCell(value: string | null): HTMLElement {
  if (value === null) return make("td", { className: "cmp-mono", textContent: "–" });
  if (value.length > 24) {
    return make("td", {
      className: "cmp-mono",
      textContent: `${value.slice(0, 24)} …`,
      title: value,
    });
  }
  return make("td", { className: "cmp-mono", textContent: value });
}

/**
 * The last step of a result, not a card of its own.
 *
 * It follows what the table shows rather than offering a second set of seven
 * switches: the chips above already say which findings matter, and two
 * controls for one decision only invite them to disagree.
 */
function exportSection(report: CompareReport, visible: Set<Cls>, status: HTMLElement): HTMLElement {
  const shownRows = report.rows.filter((row) => visible.has(row.class as Cls)).length;
  const all = make("input", { type: "checkbox" });
  const onlyWritable = make("input", { type: "checkbox" });
  const onlyNamed = make("input", { type: "checkbox" });
  const onlySwitches = make("input", { type: "checkbox" });
  const button = make("button", { className: "primary", i18n: "compare-export" });

  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      const path = await invoke<string | null>("pick_save_path", {
        kind: "analysis",
        name: "comparison.json",
      });
      if (path === null) return;
      showBusy(status, tr("msg-working"));
      const classes = all.checked ? [...CLASS_ORDER] : CLASS_ORDER.filter((c) => visible.has(c));
      const count = await invoke<number>("compare_export", {
        path,
        classes,
        onlyWritable: onlyWritable.checked,
        onlyNamed: onlyNamed.checked,
        onlySwitches: onlySwitches.checked,
      });
      showDone(status, trFill("compare-saved", { count, path }));
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });

  return make(
    "div",
    { className: "cmp-export" },
    make("h3", { i18n: "compare-export-heading" }),
    make("p", { textContent: trFill("compare-export-shown", { count: shownRows }) }),
    checkboxRow(all, "compare-export-all", "compare-export-all-hint"),
    checkboxRow(onlyWritable, "compare-only-writable", "compare-only-writable-hint"),
    checkboxRow(onlyNamed, "compare-only-named", "compare-only-named-hint"),
    checkboxRow(onlySwitches, "compare-only-switches", "compare-only-switches-hint"),
    make("div", { className: "row" }, button),
  );
}
