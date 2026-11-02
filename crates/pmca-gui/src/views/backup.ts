/** Backup page: save the camera's Backup.bin to a file and write one back. */

import { invoke } from "@tauri-apps/api/core";
import type { BackupRestoreOutcome } from "../bindings/BackupRestoreOutcome";
import type { BackupSaveOutcome } from "../bindings/BackupSaveOutcome";
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
} from "../dom";
import { filePicker } from "../drop";
import { tr, trFill } from "../i18n";

export function viewBackup(host: HTMLElement): void {
  host.append(
    card(
      make("h2", { i18n: "backup-title" }),
      make(
        "div",
        { className: "notice-block" },
        make("p", { i18n: "backup-intro" }),
        make("p", { className: "hint", i18n: "backup-own-camera" }),
      ),
    ),
    createCard(),
    restoreCard(),
  );
}

/** Save the whole backup image to a file the user picks. */
function createCard(): HTMLElement {
  const status = statusLine();
  const button = make("button", { className: "primary", i18n: "btn-backup-save" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      showBusy(status, connectingText());
      const outcome = await withProgressTarget(status, () =>
        invoke<BackupSaveOutcome | null>("backup_save"),
      );
      if (outcome === null) {
        clearStatusState(status);
        status.textContent = "";
      } else {
        showDone(status, trFill("backup-saved-at", { bytes: outcome.bytes, path: outcome.path }));
      }
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });
  return card(
    make("h3", { i18n: "backup-create-heading" }),
    make("p", { i18n: "backup-create-intro" }),
    make("p", { className: "hint", i18n: "backup-large-hint" }),
    make("div", { className: "row" }, accessBadge("read"), button),
    status,
  );
}

/** Write a backup image the user picks back to the connected camera. */
function restoreCard(): HTMLElement {
  const status = statusLine();
  const file = filePicker(status, "backup", [".bin"], "backup-drop-hint");
  const button = make("button", { i18n: "btn-backup-restore" });
  button.addEventListener("click", async () => {
    const path = file.path();
    if (path === "") {
      showErrorKey(status, "backup-restore-no-file");
      return;
    }
    // Two confirmations: writing a whole image is the most invasive thing the
    // program does, and the second asks after the risk has been named.
    if (!(await confirmed(trFill("backup-restore-confirm", { path }), tr("btn-backup-restore")))) {
      return;
    }
    if (!(await confirmed(tr("backup-restore-confirm-2"), tr("btn-backup-restore")))) return;
    button.disabled = true;
    try {
      showBusy(status, connectingText());
      const outcome = await withProgressTarget(status, () =>
        invoke<BackupRestoreOutcome>("backup_restore", { path }),
      );
      showDone(status, trFill("backup-restored-done", { bytes: outcome.bytes }));
      if (outcome.safetyCopy !== null) {
        status.append(
          " ",
          make("span", {
            className: "hint",
            textContent: trFill("backup-safety-copy", { path: outcome.safetyCopy }),
          }),
        );
      }
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });
  return card(
    make("h3", {}, accessBadge("write"), " ", make("span", { i18n: "backup-restore-heading" })),
    make("p", { i18n: "backup-restore-intro" }),
    make("p", { className: "warning", i18n: "backup-restore-warning" }),
    file.element,
    make("div", { className: "row" }, accessBadge("write"), button),
    status,
  );
}
