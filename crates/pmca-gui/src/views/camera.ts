/** Camera information page (identity, shutter count, usage log) and the compatibility page. */

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
import type {
  CameraSummary,
  CompatModel,
  DeviceGroup,
  DeviceLists,
  MoreModel,
  PhotoSummary,
  PluginCompat,
  Support,
  UsageLogOutcome,
} from "../dto";
import { currentLocale, isCommandError, modeName, tr } from "../i18n";
import { hostPlatform } from "../platform";
import { deviceToggle, markCell } from "./network";
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

  host.append(
    card(button, status, details),
    ...(hostPlatform === "windows" ? [cardServiceDiagnosis()] : []),
    cardShutterCount(() => connectedModel),
    cardUsageLog(),
  );
}

/** The compatibility page: the device lists, open from the start, and what plugins add. */
export function viewCompatibility(host: HTMLElement): void {
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

  const all = deviceLists();
  const devices = filledLater(
    all,
    (lists) => compatGroups(lists.compat),
    make("p", { className: "hint", i18n: "devices-note" }),
  );
  const more = filledLater(all, (lists) => moreGroups(lists.more));
  const moreToggle = deviceToggle(more, "btn-more-models", "btn-more-models-hide");

  host.append(
    card(
      make("h2", { i18n: "docs-compatibility" }),
      devices,
      make("p", { className: "hint", i18n: "devices-more-note" }),
      moreToggle,
      more,
      pluginDevices(),
    ),
    card(
      make("p", { i18n: "docs-compatibility-hint" }),
      make("p", { className: "doc-url", textContent: DOCS_DEVICES_URL }),
      make("div", { className: "row" }, copy),
      linkStatus,
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

const COMPAT_COLUMNS = ["info-model", "gps-col-arch", "gps-col-apps", "nav-updater", "nav-service"];

/** A capability an operation needs, looked up in the device lists. */
type Capability = "updater" | "apps" | "gps";

/** The backend's verdict on a model; `support` is `null` for an unidentified camera. */
interface SupportCheck {
  support: Support | null;
  text: string;
}

/** The device lists, or the models with one capability; kept in the backend. */
export function deviceLists(capability: "updater" | "apps" | null = null): Promise<DeviceLists> {
  return invoke<DeviceLists>("device_lists", { capability });
}

/** A container that fills itself once `lists` has answered. */
function filledLater(lists: Promise<DeviceLists>, fill: (lists: DeviceLists) => Node[], ...head: Node[]): HTMLElement {
  const box = make("div", { className: "gps-devices" }, ...head);
  lists.then((answer) => box.append(...fill(answer))).catch(() => undefined);
  return box;
}

function compatGroups(groups: DeviceGroup<CompatModel>[]): Node[] {
  return groups.flatMap((g) => [make("h3", { i18n: g.heading }), compatTable(g.models)]);
}

function moreGroups(groups: DeviceGroup<MoreModel>[]): Node[] {
  return groups.flatMap((g) => [make("h3", { i18n: g.heading }), moreTable(g.models)]);
}

/**
 * The two lists of the compatibility page, cut down to the models that have the
 * given capability, under the given notes.
 */
export function capableDevices(capability: "updater" | "apps", ...notes: string[]): HTMLElement {
  return filledLater(
    deviceLists(capability),
    (lists) => [
      ...compatGroups(lists.compat),
      ...(lists.more.length === 0
        ? []
        : [make("p", { className: "hint", i18n: "devices-more-note" }), ...moreGroups(lists.more)]),
    ],
    ...notes.map((key) => make("p", { className: "hint", i18n: key })),
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

/** What the lists say about the camera, worked out by the backend. */
function supportCheck(model: string | null, capability: Capability): Promise<SupportCheck> {
  return invoke<SupportCheck>("device_support", { model, capability });
}

/** The sentence that tells the user what the lists say about their camera. */
export async function supportText(model: string | null, capability: Capability): Promise<string> {
  return (await supportCheck(model, capability)).text;
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
  const check = await supportCheck(model, capability);
  if (check.support === "yes") return true;
  const proceed = await chosen(
    `${check.text}\n\n${tr("check-proceed")}`,
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
    rows.map(({ model, arch, apps, updater }) => [
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
    rows.map(({ model, arch, apps, report }) => [
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
