/** Firmware update page. */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  chosen,
  connectingText,
  fieldList,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  showStatus,
  statusLine,
  writing,
} from "../dom";
import { filePicker } from "../drop";
import type { FirmwareOutcome } from "../dto";
import { tr, trFill } from "../i18n";
import { IN_APP_INSTALL_MODE, capableDevices, identifyModel, supportText } from "./camera";
import { deviceToggle } from "./network";

/**
 * Offer to finish an interrupted re-flash that left the camera reporting 9.99.
 *
 * `onDone` re-checks the pending record so the card disappears once the real
 * version has been written back.
 */
function pendingRestoreCard(version: string, onDone: () => void): HTMLElement {
  const status = statusLine();
  const button = make("button", { className: "primary", i18n: "btn-firmware-restore-version" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      showBusy(status, connectingText());
      const restored = await invoke<string>("restore_firmware_version");
      showDone(status, trFill("firmware-reflash-restored", { version: restored }));
      onDone();
    } catch (e) {
      showError(status, e);
      button.disabled = false;
    }
  });
  return card(
    make(
      "div",
      { className: "notice-block" },
      make("p", { textContent: trFill("firmware-reflash-pending", { version }) }),
    ),
    make("div", { className: "row" }, button),
    status,
  );
}

/**
 * Expert tool: write an arbitrary reported version by hand over service mode.
 *
 * The CLI's `--set-version`; here for parity, e.g. to force a camera stuck on
 * the 9.99 sentinel back to its genuine version when no pending record exists.
 */
function setVersionCard(onDone: () => void): HTMLElement {
  const status = statusLine();
  const input = make("input", { type: "text" });
  input.placeholder = "1.10";
  const button = make("button", { className: "primary", i18n: "btn-firmware-set-version" });
  button.addEventListener("click", async () => {
    const version = input.value.trim();
    if (version === "") return;
    button.disabled = true;
    try {
      showBusy(status, connectingText());
      const written = await invoke<string>("set_firmware_version", { version });
      showDone(status, trFill("firmware-reflash-restored", { version: written }));
      onDone();
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });
  return card(
    make("p", { className: "field-label", i18n: "firmware-set-version-label" }),
    make("p", { className: "hint", i18n: "firmware-set-version-hint" }),
    make("div", { className: "row" }, input, writing(button)),
    status,
  );
}

export function viewFirmware(host: HTMLElement): void {
  const status = statusLine();
  const details = make("div");
  const datFile = filePicker(status, "firmware", [".dat"], "firmware-drop-hint");

  const check = make("button", { i18n: "btn-firmware-check" });
  const flash = make("button", { className: "primary", i18n: "btn-firmware-flash" });

  // Raises the image's version to the maximum so the camera accepts the same
  // (or a lower) version again; the reported version is unchanged.
  const reflash = make("input", { type: "checkbox" });
  const reflashField = make(
    "label",
    { className: "field-label" },
    reflash,
    " ",
    make("span", { i18n: "firmware-reflash" }),
  );

  // A fold-away "what it does / when to use it" box, like the Tweaks info
  // disclosure: an advanced, destructive option explained in place, without a
  // permanent block of warning text.
  const reflashInfo = make(
    "details",
    { className: "firmware-reflash-info" },
    make("summary", { i18n: "btn-firmware-reflash-info" }),
    make("p", { i18n: "firmware-reflash-hint" }),
    make("p", { i18n: "firmware-reflash-when" }),
  );

  /**
   * Run one pass over the camera.
   *
   * With `doFlash` false the backend only opens the camera and asks it whether
   * it accepts the image: nothing is written, and the camera is not rebooted
   * into updater mode. That makes it safe to call before the confirmation.
   */
  const run = async (doFlash: boolean): Promise<FirmwareOutcome | null> => {
    const file = datFile.path();
    if (file === "") {
      showErrorKey(status, "firmware-file-required");
      return null;
    }
    check.disabled = true;
    flash.disabled = true;
    details.replaceChildren();
    showBusy(status, connectingText());
    try {
      const outcome = await invoke<FirmwareOutcome>("flash_firmware", {
        file,
        flash: doFlash,
        confirm: doFlash,
        reflash: reflash.checked,
      });
      details.replaceChildren(
        fieldList([
          ["firmware-current-version", outcome.currentVersion],
          ["firmware-new-version", outcome.newVersion],
        ]),
      );
      showDone(status, tr(outcome.flashed ? "msg-done" : "firmware-check-ok"));
      return outcome;
    } catch (e) {
      showError(status, e);
      return null;
    } finally {
      check.disabled = false;
      flash.disabled = false;
    }
  };

  check.addEventListener("click", () => void run(false));

  flash.addEventListener("click", async () => {
    if (datFile.path() === "") {
      showErrorKey(status, "firmware-file-required");
      return;
    }
    // Two gates before anything is written. The camera is identified and the
    // image validated read-only first, so the one confirmation can say whether
    // the lists vouch for this camera and name the exact version transition
    // being authorised — a wrong or interrupted write is not recoverable.
    check.disabled = true;
    flash.disabled = true;
    const model = await identifyModel(status, "updater");
    check.disabled = false;
    flash.disabled = false;
    if (model === undefined || model === IN_APP_INSTALL_MODE) return;
    const preview = await run(false);
    if (!preview) return;
    const question = [
      await supportText(model, "updater"),
      tr("firmware-warning"),
      tr("firmware-untested"),
      ...(reflash.checked ? [tr("firmware-reflash-hint")] : []),
      `${preview.currentVersion} \u2192 ${preview.newVersion}`,
      tr("firmware-own-device"),
    ].join("\n\n");
    if (!(await chosen(question, tr("btn-firmware-accept"), tr("btn-cancel"), true))) {
      showStatus(status, tr("firmware-cancelled"));
      return;
    }
    await run(true);
  });

  const devices = capableDevices("updater", "firmware-models-hint", "devices-note");

  // Plan B: if an earlier re-flash left the camera on 9.99, offer to finish it.
  const pending = make("div", { hidden: true });
  const refreshPending = (): void => {
    invoke<string | null>("firmware_pending_version")
      .then((version) => {
        pending.hidden = version === null;
        pending.replaceChildren(version === null ? "" : pendingRestoreCard(version, refreshPending));
      })
      .catch(() => undefined);
  };
  refreshPending();

  host.append(
    pending,
    card(
      make("p", { className: "warning", i18n: "firmware-warning" }),
      make("label", { className: "field-label", i18n: "firmware-dat-path" }),
      datFile.element,
      reflashField,
      reflashInfo,
      make("div", { className: "row" }, check, writing(flash), deviceToggle(devices)),
      status,
      details,
      devices,
    ),
    setVersionCard(refreshPending),
  );
}
