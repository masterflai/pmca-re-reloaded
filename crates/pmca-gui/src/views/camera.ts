/** Camera information page: identity, compatibility tables, shutter count and usage log. */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  chosen,
  clearStatusState,
  connectingText,
  fieldList,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  showStatus,
  statusLine,
} from "../dom";
import { filePicker } from "../drop";
import type { CameraSummary, PhotoSummary, PluginCompat, UsageLogOutcome } from "../dto";
import { currentLocale, isCommandError, modeName, tr, trFill } from "../i18n";
import { hostPlatform } from "../platform";
import { GPS_MODELS, GPS_MODELS_UNTESTED, deviceToggle, markCell } from "./network";
import { pluginText } from "./plugins";
import { DOCS_DEVICES_URL, cardServiceDiagnosis } from "./welcome";

export function viewCameraInfo(host: HTMLElement): void {
  const status = statusLine();
  const details = make("div");
  const button = make("button", { className: "primary", i18n: "btn-get-info" });
  /** Model of the camera last read, to compare a photo against. */
  let connectedModel: string | null = null;

  button.addEventListener("click", async () => {
    button.disabled = true;
    showBusy(status, connectingText());
    details.replaceChildren();
    try {
      const info = await invoke<CameraSummary>("get_camera_info");
      connectedModel = info.modelName;
      showStatus(status, "");
      details.append(
        fieldList([
          ["info-mode", modeName(info.mode)],
          ["info-model", info.modelName],
          ["info-product-code", info.modelCode],
          ["info-serial", info.serial],
          ["info-firmware", info.firmware],
          ["info-lens", info.lens],
        ]),
      );
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });

  // No shell or opener plugin is bundled, so the address is shown in full and
  // offered for copying. A link the webview silently refuses to follow would be
  // worse than plain text the user can paste into a browser.
  const linkStatus = statusLine();
  const copy = make("button", { i18n: "btn-copy-link" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(DOCS_DEVICES_URL);
      showDone(linkStatus, tr("msg-link-copied"));
    } catch (e) {
      showError(linkStatus, e);
    }
  });

  const devices = make(
    "div",
    { className: "gps-devices" },
    make("p", { className: "hint", i18n: "devices-note" }),
    ...COMPAT_MODELS.flatMap(([heading, rows]) => [make("h3", { i18n: heading }), compatTable(rows)]),
  );
  const more = make(
    "div",
    { className: "gps-devices" },
    ...MORE_MODELS.flatMap(([heading, rows]) => [make("h3", { i18n: heading }), moreTable(rows)]),
  );
  const devicesToggle = deviceToggle(devices);
  const moreToggle = deviceToggle(more, "btn-more-models", "btn-more-models-hide");

  host.append(
    card(button, status, details),
    ...(hostPlatform === "windows" ? [cardServiceDiagnosis()] : []),
    cardShutterCount(() => connectedModel),
    cardUsageLog(),
    card(
      make("h2", { i18n: "docs-compatibility" }),
      make("p", { i18n: "docs-compatibility-hint" }),
      make("p", { className: "doc-url", textContent: DOCS_DEVICES_URL }),
      make("div", { className: "row" }, devicesToggle, copy),
      linkStatus,
      devices,
      make("p", { className: "hint", i18n: "devices-more-note" }),
      moreToggle,
      more,
      pluginDevices(),
    ),
  );
}

/**
 * The rows installed plugins add, one table per plugin under its name.
 *
 * Kept apart from the built-in lists rather than merged into them: a row from
 * a plugin is its author's statement, and the reader should be able to tell
 * whose statement they are reading.
 */
function pluginDevices(): HTMLElement {
  const box = make("div", { className: "gps-devices" });
  invoke<PluginCompat[]>("plugins_compatibility")
    .then((lists) => {
      for (const list of lists) {
        box.append(
          make("h3", {}, make("span", { i18n: "plugin-compat-heading" }), `: ${pluginText(`plg-${list.plugin}-name`, list.name)}`),
          deviceRows(
            "device-table more",
            [...COMPAT_COLUMNS, "plugin-col-note"],
            list.entries.map((row) => [
              row.model,
              row.arch === null ? markCell(null) : make("td", { textContent: row.arch }),
              markCell(row.apps),
              markCell(row.updater),
              markCell(row.service),
              make("td", { textContent: row.note ?? "" }),
            ]),
          ),
        );
      }
    })
    .catch(() => undefined);
  return box;
}

/**
 * One row of the compatibility list: model, architecture, the version of the
 * camera's app platform (`null` where it runs no PlayMemories Camera Apps) and
 * whether the updater shell can reach it. The service shell reaches every
 * listed model, so it needs no field.
 */
type CompatModel = [string, string, string | null, boolean];

/**
 * The device list of the original project (OpenMemories), built in so that it
 * is readable without a browser. Grouped as on the card; the GPS keys are
 * reused where the group is the same kind of camera.
 */
const COMPAT_MODELS: [string, CompatModel[]][] = [
  [
    "devices-group-ilc",
    [
      ["ILCA-77M2", "CXD90014", null, true],
      ["ILCA-99M2", "CXD90014", null, true],
      ["ILCE-5000", "CXD90014", "2.3", true],
      ["ILCE-5100", "CXD90014", "2.7", true],
      ["ILCE-6000", "CXD90014", "2.4", true],
      ["ILCE-6300", "CXD90014", "2.15", true],
      ["ILCE-6400", "CXD90045", null, false],
      ["ILCE-6500", "CXD90014", "2.17", true],
      ["ILCE-6600", "CXD90045", null, false],
      ["ILCE-7", "CXD90014", "2.3", true],
      ["ILCE-7C", "CXD90045", null, false],
      ["ILCE-7M2", "CXD90014", "2.10", true],
      ["ILCE-7M3", "CXD90045", null, false],
      ["ILCE-7R", "CXD90014", "2.3", true],
      ["ILCE-7RM2", "CXD90014", "2.14", true],
      ["ILCE-7RM3", "CXD90045", null, false],
      ["ILCE-7RM4", "CXD90045", null, false],
      ["ILCE-7RM4A", "CXD90045", null, false],
      ["ILCE-7S", "CXD90014", "2.6", true],
      ["ILCE-7SM2", "CXD90014", "2.14", true],
      ["ILCE-9", "CXD90045", null, false],
      ["ILCE-9M2", "CXD90045", null, false],
      ["ILCE-QX1", "CXD90014", "2.8", true],
      ["NEX-3", "CXD4115", null, true],
      ["NEX-3N", "CXD4132", null, true],
      ["NEX-5", "CXD4115", null, true],
      ["NEX-5N", "CXD4132", null, true],
      ["NEX-5R", "CXD4132", "1.1", true],
      ["NEX-5T", "CXD4132", "1.1", true],
      ["NEX-6", "CXD4132", "1.1", true],
      ["NEX-7", "CXD4132", null, true],
      ["NEX-C3", "CXD4115", null, true],
      ["NEX-F3", "CXD4132", null, true],
      ["NEX-VG10", "CXD4115", null, true],
      ["SLT-A33", "CXD4115", null, true],
      ["SLT-A37", "CXD4132", null, true],
      ["SLT-A55", "CXD4115", null, true],
      ["SLT-A57", "CXD4132", null, true],
      ["SLT-A58", "CXD4132", null, true],
      ["SLT-A65", "CXD4132", null, true],
      ["SLT-A77", "CXD4132", null, true],
      ["SLT-A99", "CXD4132", null, true],
      ["ZV-E10", "CXD90045", null, false],
    ],
  ],
  [
    "gps-devices-compact",
    [
      ["DSC-G1", "CXD4105", null, false],
      ["DSC-G3", "CXD4108", null, false],
      ["DSC-HX5V", "CXD4115", null, true],
      ["DSC-HX9V", "CXD4132", null, true],
      ["DSC-HX20", "CXD4132", null, true],
      ["DSC-HX30", "CXD4132", null, true],
      ["DSC-HX50", "CXD4132", null, true],
      ["DSC-HX60", "CXD90014", "2.4", true],
      ["DSC-HX90", "CXD90014", "2.13", true],
      ["DSC-HX99", "CXD90045", null, false],
      ["DSC-HX100V", "CXD4132", null, true],
      ["DSC-HX200V", "CXD4132", null, true],
      ["DSC-HX300", "CXD4132", null, true],
      ["DSC-HX350", "CXD90014", null, true],
      ["DSC-HX400", "CXD90014", "2.4", true],
      ["DSC-QX10", "CXD4132", null, true],
      ["DSC-QX100", "CXD4132", null, true],
      ["DSC-RX0", "CXD90014", null, true],
      ["DSC-RX0M2", "CXD90045", null, false],
      ["DSC-RX1", "CXD4132", null, true],
      ["DSC-RX1R", "CXD4132", null, true],
      ["DSC-RX1RM2", "CXD90014", "2.14", true],
      ["DSC-RX10", "CXD90014", null, true],
      ["DSC-RX10M2", "CXD90014", "2.14", true],
      ["DSC-RX10M3", "CXD90014", "2.15", true],
      ["DSC-RX10M4", "CXD90045", null, false],
      ["DSC-RX100", "CXD4132", null, true],
      ["DSC-RX100M2", "CXD4132", null, true],
      ["DSC-RX100M3", "CXD90014", "2.6", true],
      ["DSC-RX100M4", "CXD90014", "2.14", true],
      ["DSC-RX100M5", "CXD90014", "2.16", true],
      ["DSC-RX100M5A", "CXD90045", null, false],
      ["DSC-T100", "CXD4108", null, false],
      ["DSC-TX5", "CXD4115", null, true],
      ["DSC-TX7", "CXD4115", null, true],
      ["DSC-W90", "CXD4108", null, false],
      ["DSC-WX5", "CXD4115", null, true],
      ["DSC-WX60", "CXD4132", null, true],
      ["DSC-WX80", "CXD4132", null, true],
      ["DSC-WX100", "CXD4132", null, true],
      ["DSC-WX200", "CXD4132", null, true],
      ["DSC-WX220", "CXD90014", null, true],
      ["DSC-WX300", "CXD4132", null, true],
      ["DSC-WX350", "CXD90014", null, true],
      ["DSC-WX500", "CXD90014", "2.13", true],
      ["ZV-1", "CXD90045", null, false],
    ],
  ],
  [
    "gps-devices-action",
    [
      ["FDR-X1000V", "CXD90014", "2.12", true],
      ["FDR-X3000", "CXD90014", "2.15", true],
      ["HDR-AS10", "CXD4132", null, true],
      ["HDR-AS15", "CXD4132", null, true],
      ["HDR-AS30V", "CXD4132", null, true],
      ["HDR-AS50", "CXD90014", "2.15", true],
      ["HDR-AS100V", "CXD90014", "2.5", true],
      ["HDR-AS200V", "CXD90014", "2.12", true],
      ["HDR-AS300", "CXD90014", "2.15", true],
      ["HDR-AZ1", "CXD90014", "2.9", true],
    ],
  ],
  [
    "gps-devices-camcorder",
    [
      ["DCR-SX31", "MB8AC102", null, true],
      ["FDR-AX30", "CXD90014", "2.11", true],
      ["FDR-AX33", "CXD90014", "2.11", true],
      ["FDR-AX40", "CXD90014", "2.15", true],
      ["FDR-AX43", "CXD90014", null, true],
      ["FDR-AX45", "CXD90014", null, true],
      ["FDR-AX53", "CXD90014", "2.15", true],
      ["FDR-AX55", "CXD90014", "2.15", true],
      ["FDR-AX60", "CXD90014", null, true],
      ["FDR-AX100", "CXD90014", null, true],
      ["FDR-AX700", "CXD90045", null, false],
      ["FDR-AXP33", "CXD90014", "2.11", true],
      ["FDR-AXP35", "CXD90014", "2.11", true],
      ["FDR-AXP55", "CXD90014", "2.15", true],
      ["HDR-CX106", "CXD4105", null, true],
      ["HDR-CX240", "CXD90014", null, true],
      ["HDR-CX405", "CXD90014", "2.11", true],
      ["HDR-CX450", "CXD90014", "2.15", true],
      ["HDR-CX560", "CXD4132", null, true],
      ["HDR-CX625", "CXD90014", "2.15", true],
      ["HDR-CX670", "CXD90014", "2.11", true],
      ["HDR-CX675", "CXD90014", "2.15", true],
      ["HDR-CX680", "CXD90014", "2.15", true],
      ["HDR-CX700", "CXD4132", null, true],
      ["HDR-CX900", "CXD90014", null, true],
      ["HDR-MV1", "CXD4132", null, true],
      ["HDR-PJ240", "CXD90014", null, true],
      ["HDR-PJ410", "CXD90014", "2.11", true],
      ["HDR-PJ540", "CXD90014", null, true],
      ["HDR-PJ590V", "CXD4132", null, true],
      ["HDR-PJ670", "CXD90014", "2.11", true],
      ["HDR-PJ675", "CXD90014", "2.15", true],
      ["HDR-PJ760V", "CXD4132", null, true],
      ["HDR-PJ790V", "CXD4132", null, true],
      ["HDR-PJ800", "CXD90014", null, true],
      ["HDR-PJ810", "CXD90014", null, true],
      ["HDR-SR1", "CXD4105", null, false],
      ["HDR-TD10", "CXD4132", null, true],
      ["HDR-TD20", "CXD4132", null, true],
      ["HDR-UX1", "CXD4105", null, false],
      ["HDR-XR550", "CXD4120", null, true],
    ],
  ],
  [
    "devices-group-remote",
    [
      ["RM-LVR1", "CXD90014", null, true],
      ["RM-LVR2", "CXD90014", null, true],
      ["RM-LVR3", "CXD90014", null, true],
    ],
  ],
];

/**
 * What the community reports about using a model with the original tool:
 * `works`, `disputed` where failures were reported that may no longer hold,
 * or `null` where nobody has reported either way.
 */
type CommunityReport = "works" | "disputed" | null;

/**
 * One row of the list of further Sony models: model, architecture (`null`
 * where no source names it), whether the camera runs PlayMemories Camera Apps
 * (`null` where unknown) and the community report. Nothing is known about the
 * updater shell on these models, so it is shown as unavailable; the service
 * shell is assumed, as on the models the original project lists, unless the
 * community reports that it fails.
 */
type MoreModel = [string, string | null, boolean | null, CommunityReport];

/**
 * Sony cameras since 2003 that the original project does not list: the
 * interchangeable-lens bodies and the RX, ZV and HX series, plus other models
 * the community reports on. Community reports come from the issues of the
 * original repository; claims by paid conversion services are not counted, since
 * those services use methods of their own.
 */
const MORE_MODELS: [string, MoreModel[]][] = [
  [
    "devices-group-ilc",
    [
      ["DSLR-A100", null, false, null],
      ["DSLR-A200", null, false, null],
      ["DSLR-A230", null, false, null],
      ["DSLR-A290", null, false, null],
      ["DSLR-A300", null, false, null],
      ["DSLR-A330", null, false, null],
      ["DSLR-A350", null, false, null],
      ["DSLR-A380", null, false, null],
      ["DSLR-A390", null, false, null],
      ["DSLR-A450", null, false, null],
      ["DSLR-A500", null, false, null],
      ["DSLR-A550", null, false, null],
      ["DSLR-A560", null, false, null],
      ["DSLR-A580", null, false, null],
      ["DSLR-A700", null, false, null],
      ["DSLR-A850", null, false, null],
      ["DSLR-A900", null, false, null],
      ["ILCA-68", null, false, null],
      ["ILCE-1", "CXD90057", false, "works"],
      ["ILCE-1M2", "CXD90057", false, "works"],
      ["ILCE-3000", null, false, null],
      ["ILCE-3500", null, false, null],
      ["ILCE-6100", "CXD90045", false, "works"],
      ["ILCE-6700", "CXD90057", false, "works"],
      ["ILCE-7CM2", "CXD90057", false, "works"],
      ["ILCE-7CR", "CXD90057", false, "works"],
      ["ILCE-7M4", "CXD90057", false, "works"],
      ["ILCE-7M5", "CXD90057", false, "disputed"],
      ["ILCE-7RM3A", "CXD90045", false, null],
      ["ILCE-7RM5", "CXD90057", false, "works"],
      ["ILCE-7RM6", "CXD90057", false, "disputed"],
      ["ILCE-7SM3", "CXD90057", false, "works"],
      ["ILCE-9M3", "CXD90057", false, "works"],
      ["ILME-FX2", null, false, "disputed"],
      ["ILME-FX3", null, false, "works"],
      ["ILME-FX30", null, false, "works"],
      ["ILME-FX3A", null, false, "works"],
      ["ILME-FX6", null, false, "disputed"],
      ["NEX-VG20", null, false, null],
      ["NEX-VG30", null, false, null],
      ["NEX-VG900", null, false, null],
      ["SLT-A35", null, false, null],
      ["ZV-E1", "CXD90057", false, "works"],
      ["ZV-E10M2", "CXD90057", false, "works"],
    ],
  ],
  [
    "gps-devices-compact",
    [
      ["DSC-HX1", null, false, null],
      ["DSC-HX7V", null, false, "works"],
      ["DSC-HX10V", null, false, "works"],
      ["DSC-HX80", null, true, "works"],
      ["DSC-HX95", null, false, null],
      ["DSC-QX30", null, null, null],
      ["DSC-RX1RM3", "CXD90057", false, "works"],
      ["DSC-RX100M6", "CXD90045", false, "works"],
      ["DSC-RX100M7", "CXD90045", false, "works"],
      ["DSC-TX300V", null, false, "works"],
      ["DSC-W830", null, false, "disputed"],
      ["DSC-WX30", null, false, null],
      ["DSC-WX170", null, false, "disputed"],
      ["DSC-WX800", "CXD90045", false, "works"],
      ["ZV-1F", "CXD90057", false, "disputed"],
      ["ZV-1M2", "CXD90045", false, "works"],
    ],
  ],
  ["gps-devices-camcorder", [["HDR-PJ30E", null, false, "works"]]],
];

const COMPAT_COLUMNS = ["info-model", "gps-col-arch", "gps-col-apps", "nav-updater", "nav-service"];

/** A capability an operation needs, looked up in the device lists. */
type Capability = "updater" | "apps" | "gps";

/**
 * What the device lists say about a model and one capability: `yes`, `no`,
 * `untested` (listed, but nobody has confirmed it either way) or `unknown`
 * (the model is in no list).
 */
type Support = "yes" | "no" | "untested" | "unknown";

function compatSupport([, , apps, updater]: CompatModel, capability: "updater" | "apps"): Support {
  return (capability === "apps" ? apps !== null : updater) ? "yes" : "no";
}

function moreSupport([, , apps]: MoreModel, capability: "updater" | "apps"): Support {
  if (capability === "updater") return "no";
  return apps === null ? "untested" : apps ? "yes" : "no";
}

/**
 * Find a model in a grouped list.
 *
 * Several lists name a model family (DSC-HX60) where the camera may report the
 * variant with Sony's GPS suffix (DSC-HX60V), so a name ending in V is also
 * tried without it.
 */
function findModel<T extends [string, ...unknown[]]>(
  groups: [string, T[]][],
  model: string,
): T | undefined {
  const rows = groups.flatMap(([, list]) => list);
  const key = model.trim().toUpperCase();
  const find = (name: string): T | undefined => rows.find(([m]) => m.toUpperCase() === name);
  return find(key) ?? (key.endsWith("V") ? find(key.slice(0, -1)) : undefined);
}

/**
 * Look a model up for one capability.
 *
 * Every check and every device list shown on a page answers from here, so a
 * page can never offer a list that disagrees with its own check. The two lists
 * of the camera info page carry the updater and app columns; GPS has a list of
 * its own, because those lists name model families where only the V variant
 * carries the receiver.
 */
function support(model: string, capability: Capability): Support {
  const listed = findModel(COMPAT_MODELS, model);
  const more = listed ? undefined : findModel(MORE_MODELS, model);
  if (capability === "gps") {
    const key = model.trim().toUpperCase();
    if (GPS_MODELS.some(([, rows]) => rows.some(([, variant]) => variant === key))) return "yes";
    if (GPS_MODELS_UNTESTED.some(([, variant]) => variant === key)) return "untested";
    return listed || more ? "no" : "unknown";
  }
  if (listed) return compatSupport(listed, capability);
  return more ? moreSupport(more, capability) : "unknown";
}

/**
 * The two lists of the camera info page, cut down to the models that have the
 * given capability, under the given notes.
 */
export function capableDevices(capability: "updater" | "apps", ...notes: string[]): HTMLElement {
  const listed = COMPAT_MODELS.map(
    ([heading, rows]) =>
      [heading, rows.filter((row) => compatSupport(row, capability) === "yes")] as const,
  ).filter(([, rows]) => rows.length > 0);
  const more = MORE_MODELS.map(
    ([heading, rows]) =>
      [heading, rows.filter((row) => moreSupport(row, capability) === "yes")] as const,
  ).filter(([, rows]) => rows.length > 0);
  return make(
    "div",
    { className: "gps-devices" },
    ...notes.map((key) => make("p", { className: "hint", i18n: key })),
    ...listed.flatMap(([heading, rows]) => [make("h3", { i18n: heading }), compatTable(rows)]),
    ...(more.length === 0
      ? []
      : [
          make("p", { className: "hint", i18n: "devices-more-note" }),
          ...more.flatMap(([heading, rows]) => [make("h3", { i18n: heading }), moreTable(rows)]),
        ]),
  );
}

/** {@link identifyModel}: the camera is already in app-install mode. */
export const IN_APP_INSTALL_MODE = Symbol("app-install-mode");

/**
 * Identify the attached camera for a compatibility check.
 *
 * `undefined` means the operation cannot go ahead at all — there is no camera,
 * more than one, or one left in app-install mode, which cannot be asked for its
 * model and which only a replug and power cycle get out of — and the reason is
 * already on `status`. App installations are the exception to the last case:
 * that mode is exactly the one they need, and a camera in it has shown that it
 * takes apps, so {@link IN_APP_INSTALL_MODE} comes back instead. `null` means a
 * camera is there but its model could not be read; that must not stop the
 * operation, which may well work in a mode the identification cannot.
 */
export async function identifyModel(
  status: HTMLElement,
  capability: Capability,
): Promise<string | null | undefined | typeof IN_APP_INSTALL_MODE> {
  showBusy(status, connectingText());
  try {
    const info = await invoke<CameraSummary>("get_camera_info");
    showStatus(status, "");
    return info.modelName;
  } catch (e) {
    if (isCommandError(e) && e.code === "usb-app-install-mode" && capability === "apps") {
      showStatus(status, "");
      return IN_APP_INSTALL_MODE;
    }
    if (
      isCommandError(e) &&
      ["usb-no-device", "usb-too-many-devices", "usb-app-install-mode"].includes(e.code)
    ) {
      showError(status, e);
      return undefined;
    }
    showStatus(status, "");
    return null;
  }
}

/** The sentence that tells the user what the lists say about their camera. */
export function supportText(model: string | null, capability: Capability): string {
  if (model === null) return tr("check-unidentified");
  return trFill(`check-${capability}-${support(model, capability)}`, { model });
}

/**
 * Check the attached camera before an operation that carries little risk.
 *
 * Only a camera the lists do not vouch for gets a dialog, and it can still go
 * ahead: the lists are what is known, not what is possible. Resolves `true`
 * when the operation should run.
 */
export async function lowRiskCheck(status: HTMLElement, capability: "apps" | "gps"): Promise<boolean> {
  const model = await identifyModel(status, capability);
  if (model === undefined) return false;
  if (model === IN_APP_INSTALL_MODE) return true;
  if (model !== null && support(model, capability) === "yes") return true;
  const proceed = await chosen(
    `${supportText(model, capability)}\n\n${tr("check-proceed")}`,
    tr("btn-continue"),
    tr("btn-cancel"),
    true,
  );
  if (!proceed) showStatus(status, tr("check-cancelled"));
  return proceed;
}

function compatTable(rows: CompatModel[]): HTMLTableElement {
  return deviceRows(
    "device-table compat",
    COMPAT_COLUMNS,
    rows.map(([model, arch, apps, updater]) => [
      model,
      make("td", { textContent: arch }),
      markCell(apps !== null, apps ?? undefined),
      markCell(updater),
      markCell(true),
    ]),
  );
}

/**
 * The compatibility columns plus one for community reports. A disputed report
 * leaves both the service shell and the confirmation open.
 */
function moreTable(rows: MoreModel[]): HTMLTableElement {
  return deviceRows(
    "device-table more",
    [...COMPAT_COLUMNS, "devices-col-confirmed"],
    rows.map(([model, arch, apps, report]) => [
      model,
      arch === null ? markCell(null) : make("td", { textContent: arch }),
      markCell(apps),
      markCell(false),
      markCell(report === "disputed" ? null : true),
      markCell(report === "works" ? true : null),
    ]),
  );
}

function deviceRows(
  className: string,
  columns: string[],
  rows: [string, ...HTMLTableCellElement[]][],
): HTMLTableElement {
  return make(
    "table",
    { className },
    make("thead", {}, make("tr", {}, ...columns.map((key) => make("th", { scope: "col", i18n: key })))),
    make(
      "tbody",
      {},
      ...rows.map(([model, ...cells]) =>
        make("tr", {}, make("th", { scope: "row", textContent: model }), ...cells),
      ),
    ),
  );
}

/**
 * Read the shutter count from a photo the camera took.
 *
 * The count is not something the camera reports over USB; it is written into
 * every image instead. The newest image therefore carries the current figure,
 * which is why the user is asked for that one.
 *
 * `connectedModel` is read at click time, so a camera queried after the photo
 * was chosen still takes part in the comparison.
 */
function cardShutterCount(connectedModel: () => string | null): HTMLElement {
  const status = statusLine();
  const details = make("div");
  const photo = filePicker(
    status,
    "photo",
    [".arw", ".jpg", ".jpeg", ".hif", ".heif", ".heic"],
    "shutter-drop-hint",
  );
  const read = make("button", { className: "primary", i18n: "btn-shutter-read" });

  read.addEventListener("click", async () => {
    const file = photo.path();
    details.replaceChildren();
    if (file === "") {
      showErrorKey(status, "shutter-file-required");
      return;
    }
    read.disabled = true;
    showStatus(status, "");
    try {
      const result = await invoke<PhotoSummary>("read_photo", { file });
      details.append(
        fieldList([
          ["shutter-file", result.fileName],
          ["shutter-taken", result.taken],
          ["shutter-count", result.shutterCount?.toLocaleString(currentLocale) ?? null],
          ["shutter-type", result.shutter ? { key: `shutter-type-${result.shutter}` } : null],
          ["info-model", result.model],
          ["info-firmware", result.software],
          ["info-lens", result.lens],
        ]),
      );
      if (result.countUnavailable) {
        details.append(
          make("p", { className: "hint", i18n: `shutter-${result.countUnavailable}` }),
        );
      }
      if (result.shutter === "electronic") {
        details.append(make("p", { className: "hint", i18n: "shutter-electronic-hint" }));
      }
      const connected = connectedModel();
      if (connected && result.model && connected.toUpperCase() !== result.model.toUpperCase()) {
        details.append(
          make(
            "div",
            { className: "notice-block" },
            make("p", { i18n: "shutter-model-mismatch" }),
          ),
        );
      }
    } catch (e) {
      showError(status, e);
    } finally {
      read.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "shutter-heading" }),
    make("p", { i18n: "shutter-hint" }),
    make("p", { className: "hint", i18n: "shutter-source" }),
    photo.element,
    make("div", { className: "row" }, read),
    status,
    details,
  );
}

/**
 * Download the camera's usage log to a file.
 *
 * The log's structure is undocumented, so there is nothing to render: the file
 * is written as-is and only its size is reported. Showing a decoded view would
 * be inventing a meaning the format does not guarantee.
 */
function cardUsageLog(): HTMLElement {
  const status = statusLine();
  const save = make("button", { i18n: "btn-usage-log" });

  save.addEventListener("click", async () => {
    save.disabled = true;
    showBusy(status, connectingText());
    try {
      // The backend asks for the destination before it touches the camera.
      const written = await invoke<UsageLogOutcome | null>("save_usage_log");
      if (written === null) {
        clearStatusState(status);
        status.textContent = "";
        return;
      }
      showDone(status, `${tr("usage-log-written")} ${written.path} (${written.bytes})`);
    } catch (e) {
      showError(status, e);
    } finally {
      save.disabled = false;
    }
  });

  const info = make(
    "div",
    { className: "primer" },
    make("h3", { i18n: "usage-log-info-what-heading" }),
    make("p", { i18n: "usage-log-info-what" }),
    make("p", { i18n: "usage-log-info-read" }),
    make("h3", { i18n: "usage-log-info-flush-heading" }),
    make("p", { i18n: "usage-log-info-flush" }),
    make("h3", { i18n: "usage-log-info-content-heading" }),
    make("p", { i18n: "usage-log-info-shutter" }),
    make("p", { i18n: "usage-log-info-clock" }),
    make("p", { i18n: "usage-log-info-unknown" }),
    make("h3", { i18n: "usage-log-info-privacy-heading" }),
    make("p", { i18n: "usage-log-info-privacy" }),
  );
  info.hidden = true;

  const toggle = make("button", { i18n: "btn-usage-log-info" });
  toggle.setAttribute("aria-expanded", "false");
  toggle.addEventListener("click", () => {
    info.hidden = !info.hidden;
    toggle.setAttribute("aria-expanded", String(!info.hidden));
    toggle.dataset.i18n = info.hidden ? "btn-usage-log-info" : "btn-usage-log-info-hide";
    toggle.textContent = tr(toggle.dataset.i18n);
  });

  return card(
    make("h2", { i18n: "usage-log-heading" }),
    make("p", { i18n: "usage-log-hint" }),
    make("div", { className: "row" }, save),
    status,
    make("div", { className: "row" }, toggle),
    info,
  );
}
