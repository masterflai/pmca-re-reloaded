/** Backing up the apps installed on the camera (Apps page). */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  confirmed,
  make,
  showBusy,
  showDone,
  showError,
  showStatus,
  statusLine,
  withProgressTarget,
} from "../dom";
import type { AppBackupOutcome } from "../dto";
import { tr, trFill } from "../i18n";
import { IN_APP_INSTALL_MODE, identifyModel, supportText } from "./camera";

/**
 * Copy every installed app off the camera, through the updater payload.
 *
 * The camera restarts into updater mode for it, so the button asks first, as
 * the updater shell does. Restoring is an ordinary installation of a saved
 * file, which the card above already offers.
 */
export function cardAppBackup(): HTMLElement {
  const status = statusLine();
  const result = make("div");
  const backup = make("button", { className: "primary", i18n: "btn-app-backup" });

  backup.addEventListener("click", async () => {
    backup.disabled = true;
    result.replaceChildren();
    try {
      const model = await identifyModel(status, "updater");
      if (model === undefined || model === IN_APP_INSTALL_MODE) return;
      const question = `${await supportText(model, "updater")}\n\n${tr("app-backup-confirm")}`;
      if (!(await confirmed(question, tr("btn-confirm-app-backup")))) {
        showStatus(status, tr("app-backup-cancelled"));
        return;
      }
      showBusy(status, tr("msg-connecting"));
      const outcome = await withProgressTarget(status, () =>
        invoke<AppBackupOutcome>("backup_apps", { confirm: true }),
      );
      showDone(
        status,
        outcome.apps.length === 0
          ? tr("app-backup-none")
          : trFill("app-backup-done", { count: outcome.apps.length }),
      );
      result.append(...outcomeView(outcome));
    } catch (e) {
      showError(status, e);
    } finally {
      backup.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "app-backup-heading" }),
    make("p", { i18n: "app-backup-intro" }),
    make("p", { className: "hint", i18n: "app-backup-own-camera" }),
    make("div", { className: "row" }, backup),
    status,
    result,
    make("p", { className: "hint", i18n: "app-backup-restore-hint" }),
  );
}

function outcomeView(outcome: AppBackupOutcome): Node[] {
  if (outcome.apps.length === 0) return [];
  const rows = outcome.apps.map((app) =>
    make(
      "tr",
      {},
      make("th", { scope: "row", textContent: app.package }),
      make("td", { textContent: app.version ?? "–" }),
      make("td", {}, make("code", { textContent: app.file })),
      make("td", { i18n: app.alreadySaved ? "app-backup-state-kept" : "app-backup-state-new" }),
    ),
  );
  const table = make(
    "table",
    { className: "store-table" },
    make(
      "thead",
      {},
      make(
        "tr",
        {},
        ...["store-col-app", "store-col-version", "app-backup-col-file", "store-col-status"].map(
          (key) => make("th", { scope: "col", i18n: key }),
        ),
      ),
    ),
    make("tbody", {}, ...rows),
  );
  return [make("div", { className: "store-list" }, table), folderRow(outcome.folder)];
}

/** The backup folder with a copy button: the program opens no file manager. */
function folderRow(folder: string): HTMLElement {
  const status = statusLine();
  const copy = make("button", { i18n: "btn-copy-path" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(folder);
      showDone(status, tr("msg-path-copied"));
    } catch (e) {
      showError(status, e);
    }
  });
  return make(
    "div",
    {},
    make(
      "div",
      { className: "row address-row" },
      make("span", { className: "hint", i18n: "app-backup-folder" }),
      make("code", { textContent: folder }),
      copy,
    ),
    status,
  );
}
