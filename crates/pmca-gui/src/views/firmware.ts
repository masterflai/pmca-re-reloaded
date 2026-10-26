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
import { tr } from "../i18n";
import { IN_APP_INSTALL_MODE, capableDevices, identifyModel, supportText } from "./camera";
import { deviceToggle } from "./network";

export function viewFirmware(host: HTMLElement): void {
  const status = statusLine();
  const details = make("div");
  const datFile = filePicker(status, "firmware", [".dat"], "firmware-drop-hint");

  const check = make("button", { i18n: "btn-firmware-check" });
  const flash = make("button", { className: "primary", i18n: "btn-firmware-flash" });

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
  host.append(
    card(
      make("p", { className: "warning", i18n: "firmware-warning" }),
      make("label", { className: "field-label", i18n: "firmware-dat-path" }),
      datFile.element,
      make("div", { className: "row" }, check, writing(flash), deviceToggle(devices)),
      status,
      details,
      devices,
    ),
  );
}
