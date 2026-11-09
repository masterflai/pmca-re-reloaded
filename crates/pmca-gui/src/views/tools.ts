/** File tools page. */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  clearStatusState,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  statusLine,
} from "../dom";
import { browseButton, registerDragDrop, setFileDrop } from "../drop";
import type { ToolOutcome } from "../dto";
import { tr } from "../i18n";
import { pluginToolCards } from "./plugins";

/**
 * The file tools.
 *
 * Six operations that need no camera at all — they only ever read and write
 * files on this computer. They lived in the console alone because the console
 * used to be the only front-end, not because there was a reason for it.
 *
 * No gate: nothing here can reach a camera, so there is nothing to protect
 * against.
 */
export function viewTools(host: HTMLElement): void {
  const status = statusLine();
  const report = make("pre", { className: "shell-output" });

  /** The file dialog filter that suits each operation's input. */
  const INPUT_KIND: Record<string, string> = {
    apk2spk: "apk",
    spk2apk: "spk",
    "print-backup": "usage-log",
    "print-usagelog": "usage-log",
    "guess-firmware": "firmware",
    "xpd-checksum": "any",
  };

  const operation = make("select", { className: "field-select" });
  for (const [value, key] of [
    ["apk2spk", "tools-apk2spk"],
    ["spk2apk", "tools-spk2apk"],
    ["print-backup", "tools-print-backup"],
    ["print-usagelog", "tools-print-usagelog"],
    ["guess-firmware", "tools-guess-firmware"],
    ["xpd-checksum", "tools-xpd-checksum"],
  ] as const) {
    operation.append(make("option", { value, i18n: key }));
  }

  const inputPath = make("input", { type: "text", className: "path-input" });

  const browseInput = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind: INPUT_KIND[operation.value] ?? "any" }),
    (path) => {
      inputPath.value = path;
    },
  );

  // Drop suffixes per file kind; an empty string matches any name, for "any".
  const INPUT_SUFFIX: Record<string, string[]> = {
    apk: [".apk"],
    spk: [".spk"],
    "usage-log": [".bin"],
    firmware: [".dat"],
    any: [""],
  };
  const claimDrop = (): void =>
    setFileDrop({
      suffixes: INPUT_SUFFIX[INPUT_KIND[operation.value] ?? "any"] ?? [""],
      accept: (path) => {
        inputPath.value = path;
      },
    });
  operation.addEventListener("change", claimDrop);
  claimDrop();
  const dropzone = make(
    "div",
    { className: "dropzone tools-drop" },
    make("span", { className: "hint", i18n: "tools-drop-hint" }),
    make("div", { className: "row" }, browseInput, inputPath),
  );
  void registerDragDrop(dropzone);

  // The two converters write an output file; the others only print a report.
  const CONVERTER: Record<string, { kind: string; ext: string }> = {
    apk2spk: { kind: "spk", ext: "spk" },
    spk2apk: { kind: "apk", ext: "apk" },
  };
  const outputPath = make("input", { type: "text", className: "path-input" });
  const outputBrowse = browseButton(
    status,
    () => {
      const conv = CONVERTER[operation.value];
      if (!conv) return Promise.resolve(null);
      const stem = (inputPath.value.trim().split(/[\\/]/).pop() ?? "").replace(/\.[^.]*$/, "");
      return invoke<string | null>("pick_save_path", {
        kind: conv.kind,
        name: `${stem || "output"}.${conv.ext}`,
      });
    },
    (path) => {
      outputPath.value = path;
    },
  );
  const outputField = make(
    "div",
    {},
    make("label", { className: "field-label", i18n: "tools-output" }),
    make("div", { className: "row" }, outputBrowse, outputPath),
  );
  const updateOutput = (): void => {
    outputField.hidden = !CONVERTER[operation.value];
  };
  operation.addEventListener("change", updateOutput);
  updateOutput();

  const runButton = make("button", { className: "primary", i18n: "btn-service-run" });
  runButton.addEventListener("click", async () => {
    const input = inputPath.value.trim();
    if (input === "") {
      showErrorKey(status, "tools-file-required");
      return;
    }
    runButton.disabled = true;
    report.replaceChildren();
    showBusy(status, tr("msg-working"));
    try {
      // A converter's output: the field when set, else the backend's dialog.
      const outcome = await invoke<ToolOutcome | null>("run_tool", {
        operation: operation.value,
        input,
        output: outputPath.value.trim() || null,
      });
      if (outcome === null) {
        clearStatusState(status);
        status.textContent = "";
        return;
      }
      showDone(status, outcome.message);
      if (outcome.report !== null) report.textContent = outcome.report.trimEnd();
    } catch (e) {
      showError(status, e);
    } finally {
      runButton.disabled = false;
    }
  });

  host.append(
    card(
      make("h2", { i18n: "nav-tools" }),
      make("p", { i18n: "tools-hint" }),
      make("label", { className: "field-label", i18n: "tools-operation" }),
      operation,
      make("label", { className: "field-label", i18n: "tools-input" }),
      dropzone,
      outputField,
      make("div", { className: "row" }, runButton),
      status,
      report,
    ),
  );
  pluginToolCards(host);
}
