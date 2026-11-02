/** Reference check: verify a known finding against the connected camera. */

import { invoke } from "@tauri-apps/api/core";
import type { ReferenceEntry } from "../bindings/ReferenceEntry";
import type { ReferenceResult } from "../bindings/ReferenceResult";
import type { ReferenceState } from "../bindings/ReferenceState";
import {
  accessBadge,
  card,
  clearStatusState,
  confirmed,
  connectingText,
  make,
  showBusy,
  showError,
  showErrorKey,
  statusLine,
  withProgressTarget,
} from "../dom";
import { tr, trFill } from "../i18n";

type RefRender = (state: ReferenceState | null) => void;

/** A fold-away explanation of how the check works and why it is low-risk. */
function referenceHelp(): HTMLDetailsElement {
  return make(
    "details",
    { className: "reference-help" },
    make("summary", { i18n: "reference-help-toggle" }),
    make("h3", { i18n: "reference-help-how-heading" }),
    make("p", { i18n: "reference-help-how" }),
    make("h3", { i18n: "reference-help-safety-heading" }),
    make("p", { i18n: "reference-help-safety-intro" }),
    make(
      "ul",
      {},
      make("li", { i18n: "reference-help-safe-own" }),
      make("li", { i18n: "reference-help-safe-optin" }),
      make("li", { i18n: "reference-help-safe-rollback" }),
      make("li", { i18n: "reference-help-safe-guard" }),
      make("li", { i18n: "reference-help-safe-small" }),
      make("li", { i18n: "reference-help-safe-noflash" }),
    ),
  );
}

const FORM_KEY = "pmca.reference.form";

function blankEntry(): ReferenceEntry {
  return { id: "", label: "", variants: [{ label: "", hex: "" }] };
}

/** The findings last typed, so leaving and returning does not lose them. */
function storedEntries(): ReferenceEntry[] {
  try {
    const raw: unknown = JSON.parse(localStorage.getItem(FORM_KEY) ?? "null");
    if (Array.isArray(raw) && raw.length > 0) return raw as ReferenceEntry[];
  } catch {
    /* only a convenience */
  }
  return [blankEntry()];
}

function saveEntries(entries: ReferenceEntry[]): void {
  try {
    localStorage.setItem(FORM_KEY, JSON.stringify(entries));
  } catch {
    /* only a convenience */
  }
}

export function viewReference(host: HTMLElement): void {
  const status = statusLine();
  const stage = make("div", { className: "analysis-stage" });
  host.append(
    card(
      make("h2", { i18n: "reference-title" }),
      make(
        "div",
        { className: "notice-block" },
        make("p", { i18n: "reference-intro" }),
        make("p", { i18n: "reference-own-hardware" }),
      ),
      referenceHelp(),
      stage,
      status,
    ),
  );
  const render: RefRender = (state) => {
    const showForm = state === null || state.result === null;
    stage.replaceChildren(
      ...(showForm ? referenceForm(status, render, state) : referenceResult(state, status, render)),
    );
  };
  invoke<ReferenceState | null>("reference_state")
    .then(render)
    .catch((e) => {
      render(null);
      showError(status, e);
    });
}

/** The findings editor, plus the import and check buttons. */
function referenceForm(status: HTMLElement, render: RefRender, state: ReferenceState | null): Node[] {
  let entries: ReferenceEntry[] =
    state && state.entries.length > 0
      ? state.entries.map((e) => ({ id: e.id, label: e.label, variants: e.variants.map((v) => ({ ...v })) }))
      : storedEntries();
  const list = make("div", { className: "reference-list" });

  const entryCard = (entry: ReferenceEntry, index: number): HTMLElement => {
    const setting = make("input", { type: "text", className: "path-input", value: entry.label });
    setting.addEventListener("input", () => {
      entry.label = setting.value;
      saveEntries(entries);
    });
    const id = make("input", { type: "text", className: "mono path-input", value: entry.id });
    id.addEventListener("input", () => {
      entry.id = id.value;
      saveEntries(entries);
    });
    const variants = make("div", {});
    const variantRow = (variant: { label: string; hex: string }, vi: number): HTMLElement => {
      const label = make("input", { type: "text", className: "path-input", value: variant.label });
      label.addEventListener("input", () => {
        variant.label = label.value;
        saveEntries(entries);
      });
      const hex = make("input", { type: "text", className: "mono path-input", value: variant.hex });
      hex.addEventListener("input", () => {
        variant.hex = hex.value;
        saveEntries(entries);
      });
      const del = make("button", { i18n: "btn-reference-remove-value" });
      del.addEventListener("click", () => {
        entry.variants.splice(vi, 1);
        if (entry.variants.length === 0) entry.variants.push({ label: "", hex: "" });
        rebuildVariants();
        saveEntries(entries);
      });
      return make("div", { className: "row" }, label, hex, del);
    };
    const rebuildVariants = (): void =>
      variants.replaceChildren(...entry.variants.map((v, vi) => variantRow(v, vi)));
    const addVariant = make("button", { i18n: "btn-reference-add-value" });
    addVariant.addEventListener("click", () => {
      entry.variants.push({ label: "", hex: "" });
      rebuildVariants();
      saveEntries(entries);
    });
    const removeEntry = make("button", { i18n: "btn-reference-remove-finding" });
    removeEntry.addEventListener("click", () => {
      entries.splice(index, 1);
      if (entries.length === 0) entries.push(blankEntry());
      rebuild();
    });
    rebuildVariants();
    return card(
      make("label", { className: "field-label", i18n: "reference-setting-label" }),
      setting,
      make("label", { className: "field-label", i18n: "reference-id-label" }),
      id,
      make("label", { className: "field-label", i18n: "reference-values-label" }),
      variants,
      make("div", { className: "row" }, addVariant, removeEntry),
    );
  };

  const rebuild = (): void => {
    list.replaceChildren(...entries.map((e, i) => entryCard(e, i)));
    saveEntries(entries);
  };
  rebuild();

  const addEntry = make("button", { i18n: "btn-reference-add-finding" });
  addEntry.addEventListener("click", () => {
    entries.push(blankEntry());
    rebuild();
  });

  const importButton = make("button", { i18n: "btn-reference-import" });
  importButton.addEventListener("click", async () => {
    try {
      const path = await invoke<string | null>("pick_file", { kind: "analysis" });
      if (path === null) return;
      const imported = await invoke<ReferenceEntry[]>("reference_import", { path });
      // Combine, not replace: imported findings join whatever was entered by
      // hand, dropping only a leftover blank row.
      const kept = entries.filter(
        (e) => e.id.trim() !== "" || e.variants.some((v) => v.hex.trim() !== ""),
      );
      entries = [...kept, ...imported];
      rebuild();
    } catch (e) {
      showError(status, e);
    }
  });

  const check = make("button", { className: "primary", i18n: "btn-reference-check" });
  check.addEventListener("click", async () => {
    const cleaned: ReferenceEntry[] = entries
      .map((e) => ({
        id: e.id.trim(),
        label: e.label.trim(),
        variants: e.variants
          .filter((v) => v.hex.trim() !== "")
          .map((v) => ({ label: v.label.trim(), hex: v.hex.trim() })),
      }))
      .filter((e) => e.id !== "" && e.variants.length > 0);
    if (cleaned.length === 0) {
      showErrorKey(status, "reference-check-empty");
      return;
    }
    check.disabled = true;
    try {
      showBusy(status, connectingText());
      await invoke<ReferenceState>("reference_start", { entries: cleaned });
      const next = await withProgressTarget(status, () => invoke<ReferenceState>("reference_check"));
      clearStatusState(status);
      status.textContent = "";
      render(next);
    } catch (e) {
      showError(status, e);
    } finally {
      check.disabled = false;
    }
  });

  return [
    make("h2", { i18n: "reference-start-heading" }),
    // Import first: filling the list from a shared report is the quick path.
    make("h3", { i18n: "reference-from-report" }),
    make("div", { className: "row" }, importButton),
    // The same list can also be filled, or extended, by hand.
    make("h3", { i18n: "reference-manual" }),
    make("p", { className: "hint", i18n: "reference-id-hint" }),
    list,
    make("div", { className: "row" }, addEntry),
    make("div", { className: "row" }, check),
  ];
}

/** The outcome: the camera, a row per finding, and the write test if under way. */
function referenceResult(state: ReferenceState, status: HTMLElement, render: RefRender): Node[] {
  const result = state.result!;
  const c = result.camera;
  const title = [c.modelName ?? "?", c.firmware ? `FW ${c.firmware}` : null, c.region]
    .filter((part) => part)
    .join(" · ");
  const nodes: Node[] = [
    make("h2", { i18n: "reference-result-heading" }),
    make("div", { className: "analysis-caption", textContent: title }),
  ];
  if (!result.fullImage) {
    nodes.push(make("div", { className: "notice-block" }, make("p", { i18n: "reference-partial-note" })));
  }
  if (state.write !== null) {
    nodes.push(...referenceWritePanel(state, status, render));
  } else {
    const table = make("table", { className: "analysis-table" });
    result.results.forEach((r, i) => table.append(resultRow(r, i, state, status, render)));
    nodes.push(make("div", { className: "analysis-property" }, table));
  }
  const newButton = make("button", { i18n: "btn-reference-new" });
  newButton.addEventListener("click", async () => {
    try {
      await invoke("reference_discard");
      clearStatusState(status);
      status.textContent = "";
      render(null);
    } catch (e) {
      showError(status, e);
    }
  });
  nodes.push(make("div", { className: "row" }, newButton));
  return nodes;
}

/** One finding's result, with a write-test button when it can be offered. */
function resultRow(
  r: ReferenceResult,
  index: number,
  state: ReferenceState,
  status: HTMLElement,
  render: RefRender,
): HTMLElement {
  const value = r.hex ?? (r.size !== null ? `${r.size} B` : "");
  const verdict = !r.present
    ? tr("reference-absent")
    : r.matches !== null
      ? trFill("reference-matches", { label: r.matches, value })
      : trFill("reference-present-nomatch", { value });
  const attrText = r.attrNames.length > 0 ? ` · attr ${r.attr} (${r.attrNames.join(", ")})` : "";
  const cell = make(
    "td",
    {},
    make("div", { textContent: r.label || tr("reference-unnamed") }),
    make("div", { className: "hint", textContent: verdict + attrText }),
  );
  const actions = make("td");
  if (r.writable) {
    const button = make("button", { i18n: "btn-reference-write" });
    button.addEventListener("click", () => startWriteTest(index, state, status, render));
    actions.append(accessBadge("write"), " ", button);
  }
  return make("tr", {}, make("th", { className: "mono", textContent: r.id }), cell, actions);
}

/** Write one finding's other value, after two confirmations. */
async function startWriteTest(
  index: number,
  state: ReferenceState,
  status: HTMLElement,
  render: RefRender,
): Promise<void> {
  const entry = state.entries[index];
  const current = state.result!.results[index];
  let variant = entry.variants.findIndex((v) => v.label !== current.matches);
  if (variant < 0) variant = 0;
  const chosen = entry.variants[variant];
  const question = trFill("reference-write-confirm", {
    setting: entry.label,
    label: chosen.label,
  });
  if (!(await confirmed(question, tr("btn-reference-write")))) return;
  if (!(await confirmed(tr("reference-write-confirm-2"), tr("btn-reference-write")))) return;
  try {
    showBusy(status, connectingText());
    const next = await withProgressTarget(status, () =>
      invoke<ReferenceState>("reference_write", { index, variant }),
    );
    clearStatusState(status);
    status.textContent = "";
    render(next);
  } catch (e) {
    showError(status, e);
  }
}

/** The write test in progress: menu check, restore, menu check, verdict. */
function referenceWritePanel(state: ReferenceState, status: HTMLElement, render: RefRender): Node[] {
  const w = state.write!;
  const nodes: Node[] = [
    make("h3", {}, accessBadge("write"), " ", make("span", { i18n: "reference-write-heading" })),
    make("p", {
      textContent: trFill("reference-write-done", {
        setting: w.label,
        label: w.variantLabel,
        value: w.writtenHex,
      }),
    }),
  ];
  if (w.menuAfterWrite === null) {
    nodes.push(...menuQuestion(status, render, false));
  } else if (!w.restored) {
    const button = make("button", { className: "primary", i18n: "btn-reference-restore" });
    button.addEventListener("click", async () => {
      button.disabled = true;
      try {
        showBusy(status, connectingText());
        const next = await withProgressTarget(status, () =>
          invoke<ReferenceState>("reference_restore"),
        );
        clearStatusState(status);
        status.textContent = "";
        render(next);
      } catch (e) {
        showError(status, e);
        button.disabled = false;
      }
    });
    nodes.push(
      make("p", { textContent: trFill("reference-menu-was", { observed: w.menuAfterWrite }) }),
      make("p", { i18n: "reference-restore-intro" }),
      make("div", { className: "row" }, accessBadge("write"), button),
    );
  } else if (w.menuAfterRestore === null) {
    nodes.push(...menuQuestion(status, render, true));
  } else {
    nodes.push(
      make("p", { i18n: w.survivedRestart ? "reference-survived" : "reference-not-survived" }),
    );
  }
  return nodes;
}

/** Ask what the camera's menu shows now. */
function menuQuestion(status: HTMLElement, render: RefRender, afterRestore: boolean): Node[] {
  const input = make("input", { type: "text", className: "path-input" });
  const confirm = make("button", { className: "primary", i18n: "btn-reference-menu-confirm" });
  confirm.addEventListener("click", async () => {
    const observed = input.value.trim();
    if (observed === "") {
      showErrorKey(status, "reference-menu-question");
      return;
    }
    confirm.disabled = true;
    try {
      const next = await invoke<ReferenceState>("reference_menu", { observed });
      clearStatusState(status);
      status.textContent = "";
      render(next);
    } catch (e) {
      showError(status, e);
      confirm.disabled = false;
    }
  });
  return [
    make("p", { i18n: afterRestore ? "reference-menu-after-restore" : "reference-menu-after-write" }),
    make("label", { className: "row" }, accessBadge("user"), input),
    make("div", { className: "row" }, confirm),
  ];
}
