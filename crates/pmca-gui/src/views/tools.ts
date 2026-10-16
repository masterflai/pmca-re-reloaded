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
import { browseButton } from "../drop";
import type { ToolOutcome } from "../dto";
import { tr } from "../i18n";
import { pluginToolCards } from "./plugins";

/**
 * The file tools.
 *
 * Five operations that need no camera at all — they only ever read and write
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
      // The converters ask for their output file in a dialog of the backend.
      const outcome = await invoke<ToolOutcome | null>("run_tool", {
        operation: operation.value,
        input,
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
      make("div", { className: "row" }, inputPath, browseInput),
      make("div", { className: "row" }, runButton),
      status,
      report,
    ),
  );
  pluginToolCards(host);
}
