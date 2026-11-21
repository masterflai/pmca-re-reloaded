/** Analysis page: a choice of three analyses, then the chosen one. */

import { invoke } from "@tauri-apps/api/core";
import type { Access as AnalysisAccess } from "../bindings/Access";
import type { CandidateView as AnalysisCandidate } from "../bindings/CandidateView";
import type { CrossCheckView as AnalysisCrossCheck } from "../bindings/CrossCheckView";
import type { EvaluationView as AnalysisEvaluation } from "../bindings/EvaluationView";
import type { PlanStep as AnalysisStep } from "../bindings/PlanStep";
import type { RunState as AnalysisState } from "../bindings/RunState";
import {
  accessBadge,
  card,
  clearStatusState,
  confirmed,
  connectingText,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  statusLine,
  withProgressTarget,
  writeConsentDialog,
} from "../dom";
import { refreshElevation, type DriverRights } from "../elevation";
import { tr, trFill } from "../i18n";
import { hostPlatform } from "../platform";
import { writeTestsAllowed } from "../preferences";
import { viewCompare } from "./compare";
import { viewMemory } from "./memory";
import { fetchNames, fwNameLabel, namesLine } from "./names";
import { viewReference, writeLockedNote } from "./reference";

// ---------------------------------------------------------------------------
// Function analysis
// ---------------------------------------------------------------------------

// The shapes come from `pmca_app::analysis` via ts-rs (see `../dto.ts`).

type AnalysisRender = (state: AnalysisState | null) => void;

/** Set when the user finishes a run without the write test they agreed to. */
let analysisWriteSkipped = false;

/** A badge naming what a step does to the camera: reads, writes, or neither. */
/**
 * Instructions for the user, framed; red-edged when a write follows.
 *
 * Catalogue entries are single lines, so numbered steps ("1. … 2. …") are put
 * on lines of their own here.
 */
function instructionBox(text: string, write = false): HTMLElement {
  return make("div", {
    className: write ? "analysis-box write" : "analysis-box",
    textContent: text.replace(/ (?=\d+\. )/g, "\n"),
  });
}

/** Every step of the run, the next one highlighted. */
function analysisPlan(plan: AnalysisStep[]): HTMLElement {
  const list = make("ol", { className: "analysis-plan" });
  let current = false;
  for (const step of plan) {
    const item = make("li", {}, accessBadge(step.access), make("span", { textContent: step.text }));
    if (step.done) {
      item.classList.add("done");
    } else if (!current) {
      item.classList.add("current");
      current = true;
    }
    list.append(item);
  }
  return list;
}

/**
 * What the analysis is for and what it does, in full.
 *
 * Folded shut by default; the user opens it when they want the background.
 */
function analysisExplanation(): HTMLDetailsElement {
  const details = make(
    "details",
    {},
    make("summary", { i18n: "analysis-explanation-toggle" }),
  );
  const sections: [string, string, AnalysisAccess | null][] = [
    ["analysis-purpose-heading", "analysis-purpose", null],
    ["analysis-how-heading", "analysis-how", null],
    ["analysis-requirements-heading", "analysis-requirements", null],
    ["analysis-read-heading", "analysis-read", "read"],
    ["analysis-write-heading", "analysis-write", "write"],
    ["analysis-legal-heading", "analysis-legal", null],
  ];
  for (const [heading, body, access] of sections) {
    const h3 = make("h3", {}, make("span", { i18n: heading }));
    if (access !== null) h3.prepend(accessBadge(access), " ");
    details.append(h3, make("p", { className: "analysis-text", i18n: body }));
  }
  return details;
}

type Mode = "memory" | "find" | "compare";
type FindMode = "discover" | "reference";

/** The analysis on screen; kept for the session so leaving the page does not lose the place. */
let currentMode: Mode | null = null;
let findMode: FindMode | null = null;

/**
 * The analysis page: three analyses to choose from, then only the chosen one.
 *
 * A discovery run or reference check under way opens its analysis directly.
 */
export function viewAnalysis(host: HTMLElement): void {
  const show = (mode: Mode | null): void => {
    if (currentMode === "compare" && mode !== "compare") void invoke("compare_close").catch(() => undefined);
    if (currentMode === "memory" && mode !== "memory") void invoke("memory_close").catch(() => undefined);
    currentMode = mode;
    host.replaceChildren();
    if (mode === null) {
      host.append(chooser(show));
    } else {
      const back = make("button", { className: "link-button analysis-back", i18n: "analysis-back" });
      back.addEventListener("click", () => show(null));
      const stage = make("div");
      host.append(back, stage);
      if (mode === "memory") {
        viewMemory(stage, {
          openReference: () => {
            findMode = "reference";
            show("find");
          },
        });
      } else if (mode === "compare") viewCompare(stage);
      else viewFind(stage);
    }
    host.closest<HTMLElement>(".content")?.scrollTo({ top: 0 });
  };

  show(currentMode);
  if (currentMode !== null) return;
  Promise.all([
    invoke<unknown>("analysis_state").catch(() => null),
    invoke<unknown>("reference_state").catch(() => null),
  ]).then(([run, check]) => {
    if (currentMode !== null || !host.isConnected || (run === null && check === null)) return;
    findMode = run !== null ? "discover" : "reference";
    show("find");
  });
}

/** The three analyses as tiles, and which firmware names are installed. */
function chooser(show: (mode: Mode) => void): HTMLElement {
  const tile = (mode: Mode, titleKey: string, subKey: string, tags: Node[]): HTMLElement => {
    const button = make(
      "button",
      { className: "analysis-tile" },
      make("span", { className: "analysis-tile-title", i18n: titleKey }),
      make("span", { className: "analysis-tile-sub", i18n: subKey }),
      make("span", { className: "analysis-tile-tags" }, ...tags),
    );
    button.addEventListener("click", () => show(mode));
    return button;
  };
  const tag = (key: string): HTMLElement => make("span", { className: "analysis-tag", i18n: key });
  // Its own line, as before; the badge sits outside the tag text, whose smaller type would shrink it.
  const writeTags = writeTestsAllowed
    ? [make("span", { className: "analysis-tile-write" }, accessBadge("write"), tag("analysis-tag-write"))]
    : [tag("analysis-tag-write-locked")];
  return card(
    make("h2", { i18n: "analysis-choose-heading" }),
    make(
      "div",
      { className: "analysis-tiles" },
      tile("memory", "analysis-tile-memory", "analysis-tile-memory-sub", [
        accessBadge("read"),
        tag("analysis-tag-source"),
      ]),
      tile("find", "analysis-tile-find", "analysis-tile-find-sub", [
        accessBadge("read"),
        tag("analysis-tag-camera"),
        ...writeTags,
      ]),
      tile("compare", "analysis-tile-compare", "analysis-tile-compare-sub", [
        accessBadge("read"),
        tag("analysis-tag-source"),
      ]),
    ),
    namesLine(),
    fetchNames(),
  );
}

/** Finding a setting: search for it, or check a place already known. */
function viewFind(host: HTMLElement): void {
  const stage = make("div");
  const render = (): void => {
    stage.replaceChildren();
    if (findMode === "discover") viewDiscovery(stage);
    else if (findMode === "reference") viewReference(stage);
  };
  const choices = ([
    ["discover", "analysis-mode-discover", "analysis-mode-discover-sub"],
    ["reference", "analysis-mode-reference", "analysis-mode-reference-sub"],
  ] as const).map(([value, labelKey, hintKey]) => {
    const radio = make("input", { type: "radio", name: "find-mode", value, checked: findMode === value });
    radio.addEventListener("change", () => {
      if (!radio.checked) return;
      findMode = value;
      render();
    });
    return make(
      "label",
      { className: "cmp-choice" },
      make("span", { className: "row cmp-choice-head" }, radio, make("span", { className: "cmp-choice-title", i18n: labelKey })),
      make("span", { className: "hint cmp-choice-hint", i18n: hintKey }),
    );
  });
  host.append(card(make("h2", { i18n: "analysis-find-question" }), ...choices), stage);
  render();
}

function viewDiscovery(host: HTMLElement): void {
  const status = statusLine();
  const stage = make("div", { className: "analysis-stage" });
  const explanation = analysisExplanation();
  const pending = make("div", { hidden: true });
  const admin = make("div", { hidden: true });
  host.append(
    card(
      make("h2", { i18n: "analysis-title" }),
      make(
        "div",
        { className: "notice-block" },
        make("p", { i18n: "analysis-own-hardware" }),
      ),
      explanation,
    ),
    admin,
    pending,
    card(stage, status),
  );

  const render: AnalysisRender = (state) => {
    stage.replaceChildren(
      ...(state === null ? analysisForm(status, render) : analysisRunNodes(state, status, render)),
    );
  };

  // Offered before anything is typed: every snapshot needs a driver, and the
  // one rights prompt of the session is better taken now than mid-run.
  if (hostPlatform === "windows") {
    invoke<DriverRights>("driver_rights")
      .then((rights) => {
        if (rights !== "standard") return;
        admin.hidden = false;
        admin.replaceChildren(elevationCard(() => (admin.hidden = true)));
      })
      .catch(() => undefined);
  }
  invoke<AnalysisState | null>("analysis_state")
    .then(render)
    .catch((e) => {
      render(null);
      showError(status, e);
    });
  invoke<boolean>("analysis_pending")
    .then((open) => {
      if (!open) return;
      pending.hidden = false;
      pending.replaceChildren(pendingRestoreCard());
    })
    .catch(() => undefined);
}

/** Offer the driver helper up front; `done` hides the card once it runs. */
function elevationCard(done: () => void): HTMLElement {
  const status = statusLine();
  const button = make("button", { className: "primary", i18n: "btn-start-driver-helper" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      await invoke("start_driver_helper");
      await refreshElevation();
      done();
    } catch (e) {
      showError(status, e);
      button.disabled = false;
    }
  });
  return card(
    make("div", { className: "notice-block" }, make("p", { i18n: "analysis-needs-admin" })),
    make("div", { className: "row" }, button),
    status,
  );
}

/** Offer to restore what an interrupted write test left on the camera. */
function pendingRestoreCard(): HTMLElement {
  const status = statusLine();
  const button = make("button", { className: "primary", i18n: "btn-analysis-restore" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      showBusy(status, connectingText());
      await withProgressTarget(status, () => invoke<number>("analysis_restore_pending"));
      showDone(status, tr("analysis-restored"));
      button.hidden = true;
    } catch (e) {
      showError(status, e);
      button.disabled = false;
    }
  });
  return card(
    make("div", { className: "notice-block" }, make("p", { i18n: "analysis-pending" })),
    make("div", { className: "row" }, accessBadge("write"), button),
    status,
  );
}

const ANALYSIS_FORM_KEY = "pmca.analysis.form";

/** The setting and values last typed, so a restart does not lose them. */
function storedAnalysisForm(): { setting: string; values: string } {
  try {
    const saved = JSON.parse(localStorage.getItem(ANALYSIS_FORM_KEY) ?? "{}");
    return { setting: String(saved.setting ?? ""), values: String(saved.values ?? "") };
  } catch {
    return { setting: "", values: "" };
  }
}

/**
 * The start page: setting, values, and the write test as an opt-in.
 *
 * The write test's consent is deliberately not remembered.
 */
function analysisForm(status: HTMLElement, render: AnalysisRender): Node[] {
  const saved = storedAnalysisForm();
  const setting = make("input", { type: "text", className: "path-input", value: saved.setting });
  const values = make("input", { type: "text", className: "path-input", value: saved.values });
  const remember = (): void => {
    try {
      localStorage.setItem(
        ANALYSIS_FORM_KEY,
        JSON.stringify({ setting: setting.value, values: values.value }),
      );
    } catch {
      /* only a convenience */
    }
  };
  setting.addEventListener("input", remember);
  values.addEventListener("input", remember);
  const write = make("input", { type: "checkbox", disabled: !writeTestsAllowed });
  write.addEventListener("change", async () => {
    if (write.checked && !(await writeConsentDialog("btn-analysis-write-enable"))) write.checked = false;
  });

  const start = make("button", { className: "primary", i18n: "btn-analysis-start" });
  start.addEventListener("click", async () => {
    start.disabled = true;
    try {
      const list = values.value
        .split(",")
        .map((v) => v.trim())
        .filter((v) => v !== "");
      const state = await invoke<AnalysisState>("analysis_start", {
        setting: setting.value,
        values: list,
        writeTest: write.checked,
      });
      analysisWriteSkipped = false;
      clearStatusState(status);
      status.textContent = "";
      render(state);
    } catch (e) {
      showError(status, e);
    } finally {
      start.disabled = false;
    }
  });

  return [
    make("h2", { i18n: "analysis-start-heading" }),
    make("label", { className: "field-label", i18n: "analysis-setting-label" }),
    setting,
    make("label", { className: "field-label", i18n: "analysis-values-label" }),
    values,
    make("p", { className: "hint", i18n: "analysis-values-hint" }),
    make(
      "label",
      { className: "row" },
      write,
      accessBadge("write"),
      make("span", { i18n: "analysis-write-option" }),
    ),
    ...(writeTestsAllowed ? [] : [make("p", {}, writeLockedNote())]),
    make("div", { className: "row" }, start),
  ];
}

/**
 * Check the candidates on other cameras: the results so far, and the button
 * for the next one while the run allows it.
 */
function analysisCrossCheck(
  state: AnalysisState,
  status: HTMLElement,
  render: AnalysisRender,
): Node[] {
  const nodes: Node[] = [
    make("h3", { i18n: "analysis-cross-heading" }),
    make("p", { i18n: "analysis-cross-intro" }),
  ];
  for (const check of state.crossChecks) nodes.push(crossCheckTable(check));
  if (state.canCrossCheck) {
    const button = make("button", { i18n: "btn-analysis-cross" });
    analysisButton(button, status, render, "analysis_cross_check");
    nodes.push(
      instructionBox(tr("analysis-cross-do")),
      make("div", { className: "row" }, accessBadge("read"), button),
    );
  }
  return nodes;
}

/** One other camera: its name on one line, each candidate as found there below. */
function crossCheckTable(check: AnalysisCrossCheck): HTMLElement {
  const c = check.camera;
  const title = [c.modelName ?? "?", c.firmware ? `FW ${c.firmware}` : null, c.region]
    .filter((part) => part)
    .join(" · ");
  const table = make("table", { className: "analysis-table" });
  for (const v of check.values) {
    const cell = make("td");
    if (!v.present) {
      cell.append(make("span", { i18n: "analysis-cross-absent" }));
    } else {
      cell.append(trFill("analysis-cross-present", { value: v.hex ?? `${v.size} B` }), " · ");
      cell.append(
        v.matches !== null
          ? trFill("analysis-cross-match", { label: v.matches })
          : make("span", { i18n: "analysis-cross-nomatch" }),
      );
    }
    table.append(make("tr", {}, make("th", { className: "mono addr", textContent: v.id }), cell));
  }
  return make(
    "div",
    { className: "analysis-property" },
    make("div", { className: "analysis-caption", textContent: title }),
    table,
  );
}

/** Run one backend step from a button, showing progress in `status`. */
function analysisButton(
  button: HTMLButtonElement,
  status: HTMLElement,
  render: AnalysisRender,
  command: string,
): void {
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      showBusy(status, connectingText());
      render(await withProgressTarget(status, () => invoke<AnalysisState>(command)));
    } catch (e) {
      showError(status, e);
      button.disabled = false;
    }
  });
}

/** The comparison: one table per candidate, a value per snapshot. */
function analysisResult(e: AnalysisEvaluation): HTMLElement {
  const box = make(
    "div",
    { className: "analysis-result" },
    make("p", {
      textContent: trFill("analysis-result", {
        compared: e.compared,
        count: e.candidates.length,
        running: e.running,
      }),
    }),
  );
  for (const c of e.candidates) {
    box.append(
      analysisTable(c, c.consistent ? "analysis-candidate-consistent" : "analysis-candidate-inconsistent"),
    );
  }
  if (e.candidates.length === 0 && e.changed.length > 0) {
    // Nothing found: these values are the only lead, so they are shown open.
    box.append(make("div", { className: "notice-block" }, make("p", { i18n: "analysis-changed-hint" })));
    for (const c of e.changed) box.append(analysisTable(c, null));
  } else if (e.running > 0) {
    const info = make(
      "details",
      { className: "analysis-info" },
      make("summary", { i18n: "analysis-changed-info" }),
    );
    for (const c of e.changed) info.append(analysisTable(c, null));
    box.append(info);
  }
  return box;
}

/** One property: its name on one line, its value at every snapshot below. */
function analysisTable(c: AnalysisCandidate, fitKey: string | null): HTMLElement {
  const names = c.attrNames.length > 0 ? ` (${c.attrNames.join(", ")})` : "";
  const caption = make(
    "div",
    { className: "analysis-caption" },
    make("span", { className: "addr", textContent: c.id }),
    ` · ${c.size} B · attr ${c.attr}${names}`,
  );
  if (fitKey !== null) caption.append(" · ", make("span", { i18n: fitKey }));
  const fw = fwNameLabel(c.fwName);
  if (fw !== null) caption.append(" · ", fw);
  const table = make("table", { className: "analysis-table" });
  for (const v of c.values) {
    const shown = v.hex ?? `Δ ${v.changedOffsets.map((o) => `0x${o.toString(16)}`).join(", ")}`;
    table.append(
      make(
        "tr",
        {},
        make("th", { textContent: v.label }),
        make("td", { className: "mono", textContent: shown }),
      ),
    );
  }
  return make("div", { className: "analysis-property" }, caption, table);
}

/** Ask which value the camera's menu shows now. */
function analysisMenuQuestion(
  state: AnalysisState,
  status: HTMLElement,
  render: AnalysisRender,
): Node[] {
  const group = `analysis-menu-${state.restored ? "restore" : "write"}`;
  const choices = state.values.map((value) => ({
    value,
    input: make("input", { type: "radio", name: group }),
  }));
  const otherChoice = make("input", { type: "radio", name: group });
  const other = make("input", { type: "text", className: "path-input" });
  other.addEventListener("input", () => {
    otherChoice.checked = true;
  });

  const confirm = make("button", { className: "primary", i18n: "btn-analysis-menu-confirm" });
  confirm.addEventListener("click", async () => {
    const picked =
      choices.find((c) => c.input.checked)?.value ??
      (otherChoice.checked ? other.value.trim() : "");
    if (picked === "") {
      showErrorKey(status, "analysis-menu-question");
      return;
    }
    confirm.disabled = true;
    try {
      const next = await invoke<AnalysisState>("analysis_menu", { observed: picked });
      clearStatusState(status);
      status.textContent = "";
      render(next);
    } catch (e) {
      showError(status, e);
      confirm.disabled = false;
    }
  });

  return [
    make("p", { i18n: "analysis-menu-question" }),
    ...choices.map((c) =>
      make("label", { className: "row" }, c.input, make("span", { textContent: c.value })),
    ),
    make(
      "label",
      { className: "row" },
      otherChoice,
      make("span", { i18n: "analysis-menu-other" }),
      other,
    ),
    make("div", { className: "row" }, accessBadge("user"), confirm),
  ];
}

/** Leave the run behind and go back to the start page. */
function analysisNewButton(status: HTMLElement, render: AnalysisRender, ask: boolean): HTMLElement {
  const button = make("button", { i18n: "btn-analysis-new" });
  button.addEventListener("click", async () => {
    if (ask && !(await confirmed(tr("analysis-discard-confirm"), tr("btn-analysis-new")))) return;
    try {
      await invoke("analysis_discard");
      analysisWriteSkipped = false;
      clearStatusState(status);
      status.textContent = "";
      render(null);
    } catch (e) {
      showError(status, e);
    }
  });
  return button;
}

/** The page for wherever the run stands. */
function analysisRunNodes(
  state: AnalysisState,
  status: HTMLElement,
  render: AnalysisRender,
): Node[] {
  const nodes: Node[] = [make("h2", { textContent: state.setting }), analysisPlan(state.plan)];

  if (state.nextLabel !== null) {
    const key = state.taken === 0 ? "analysis-do-prepare" : "analysis-do-change";
    nodes.push(instructionBox(trFill(key, { setting: state.setting, value: state.nextLabel })));
    const read = make("button", { className: "primary", i18n: "btn-analysis-snapshot" });
    analysisButton(read, status, render, "analysis_snapshot");
    nodes.push(
      make("p", {
        className: "warning",
        textContent: trFill("analysis-reading", { index: state.taken + 1, total: state.total }),
      }),
      make("div", { className: "row" }, accessBadge("read"), read, analysisNewButton(status, render, true)),
    );
    return nodes;
  }

  const evaluation = state.evaluation!;
  nodes.push(analysisResult(evaluation));
  // Switched off in the settings since the run began: nothing more is written,
  // but a value already written is still put back.
  const allowed = writeTestsAllowed || state.written;
  // One line on the write test, whatever the run asked for, so its absence is never unexplained.
  if (!allowed) {
    nodes.push(make("div", { className: "notice-block" }, make("p", {}, writeLockedNote())));
  } else if (!state.writeRequested) {
    nodes.push(make("p", { className: "hint", i18n: "analysis-write-not-chosen" }));
  } else if (evaluation.write === null && evaluation.writeRefusal) {
    nodes.push(make("p", { className: "warning", textContent: evaluation.writeRefusal }));
  }
  const writing =
    state.writeRequested && evaluation.write !== null && !analysisWriteSkipped && allowed;

  if (writing && !state.written) {
    const p = evaluation.write!;
    const preview = trFill("analysis-write-preview", {
      id: p.id,
      size: p.size,
      old: p.oldHex,
      "old-label": p.oldLabel,
      new: p.newHex,
      "new-label": p.newLabel,
    });
    nodes.push(
      make("h3", { i18n: "analysis-write-preview-heading" }),
      instructionBox(`${preview}\n${tr("analysis-own-hardware")}\n\n${tr("analysis-do-reconnect")}`, true),
    );
    const write = make("button", { className: "primary", i18n: "btn-analysis-write" });
    analysisButton(write, status, render, "analysis_write");
    const skip = make("button", { i18n: "btn-analysis-skip-write" });
    skip.addEventListener("click", () => {
      analysisWriteSkipped = true;
      render(state);
    });
    nodes.push(make("div", { className: "row" }, accessBadge("write"), write, skip));
    return nodes;
  }

  if (writing && state.menuAfterWrite === null) {
    nodes.push(
      make("p", { i18n: "analysis-written" }),
      instructionBox(trFill("analysis-do-check", { setting: state.setting })),
      ...analysisMenuQuestion(state, status, render),
    );
    return nodes;
  }

  if (writing && !state.restored) {
    nodes.push(
      make("h3", { i18n: "btn-analysis-restore" }),
      instructionBox(tr("analysis-do-reconnect"), true),
    );
    const restore = make("button", { className: "primary", i18n: "btn-analysis-restore" });
    analysisButton(restore, status, render, "analysis_restore");
    nodes.push(make("div", { className: "row" }, accessBadge("write"), restore));
    return nodes;
  }

  if (writing && state.menuAfterRestore === null) {
    nodes.push(
      make("p", { i18n: state.survivedRestart ? "analysis-survived" : "analysis-not-survived" }),
      instructionBox(trFill("analysis-do-check", { setting: state.setting })),
      ...analysisMenuQuestion(state, status, render),
    );
    return nodes;
  }

  if ((state.evaluation?.candidates.length ?? 0) > 0) {
    nodes.push(...analysisCrossCheck(state, status, render));
  }
  if (state.saved !== null) {
    nodes.push(make("p", { textContent: trFill("analysis-report-saved", { path: state.saved }) }));
  }
  nodes.push(make("p", { className: "hint", i18n: "analysis-report-hint" }));
  const exportButton = make("button", { className: "primary", i18n: "btn-analysis-export" });
  exportButton.addEventListener("click", async () => {
    exportButton.disabled = true;
    try {
      const written = await invoke<string | null>("analysis_export");
      if (written === null) return;
      showDone(status, trFill("analysis-report-saved", { path: written }));
    } catch (e) {
      showError(status, e);
    } finally {
      exportButton.disabled = false;
    }
  });
  nodes.push(make("div", { className: "row" }, exportButton, analysisNewButton(status, render, false)));
  return nodes;
}
