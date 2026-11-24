/** Camera information page (identity, model support, shutter count) and the compatibility page. */

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
} from "../dto";
import { currentLocale, isCommandError, modeName, tr, trFill } from "../i18n";
import { rememberCameraModel } from "../nav";
import { markCell } from "./network";
import { pluginText } from "./plugins";
import { drawReport, paperFor, saveReport, type ReportFormat } from "./shutter-report";

export function viewCameraInfo(host: HTMLElement): void {
  const status = statusLine();
  const details = make("div");
  const button = make("button", { className: "primary", i18n: "btn-get-info" });
  /** The camera last read, to compare a photo against and to fill in its serial number. */
  let connected: ConnectedCamera = { model: null, serial: null };

  button.addEventListener("click", async () => {
    button.disabled = true;
    showBusy(status, connectingText());
    details.replaceChildren();
    try {
      const info = await invoke<CameraSummary>("get_camera_info");
      connected = { model: info.modelName, serial: info.serial };
      rememberCameraModel(info.modelName);
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
      const support = make("div");
      details.append(support);
      if (info.modelName) {
        void supportLine(info.modelName)
          .then((line) =>
            support.append(
              fieldList([["info-support", line]]),
              make("p", { className: "hint", i18n: "info-support-hint" }),
            ),
          )
          .catch(() => undefined);
      }
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });

  host.append(
    card(
      make("h2", { i18n: "info-heading" }),
      make("p", { i18n: "info-intro" }),
      make("div", { className: "row" }, button),
      status,
      details,
    ),
    cardShutterCount(() => connected),
  );
}

/** The compatibility page: every device list at once, searchable, and what plugins add. */
export function viewCompatibility(host: HTMLElement): void {
  const search = make("input", { type: "search", className: "path-input" });
  const count = make("p", { className: "hint" });
  const all = deviceLists();
  const devices = filledLater(
    all,
    (lists) => compatGroups(lists.compat),
    make("p", { className: "hint", i18n: "devices-note" }),
  );
  const moreNote = make("p", { className: "hint", i18n: "devices-more-note" });
  const more = filledLater(all, (lists) => moreGroups(lists.more), moreNote);
  const page = card(
    make("h2", { i18n: "docs-compatibility" }),
    make("label", { className: "field-label", i18n: "devices-search" }),
    search,
    count,
    devices,
    more,
  );

  const apply = (): void => filterDevices(page, normalized(search.value), count, moreNote);
  search.addEventListener("input", apply);
  all.then(apply).catch(() => undefined);
  page.append(pluginDevices(apply));
  host.append(page);
}

/** A model name reduced for matching: lower case, α as a, no spaces or dashes. */
function normalized(text: string): string {
  return text.toLowerCase().replace(/alpha|α/g, "a").replace(/[^\p{L}\p{N}]/gu, "");
}

/**
 * Show only the rows whose model or sales name contains `query`, hide tables
 * (and their headings) left empty, and say how many rows remain.
 */
function filterDevices(page: HTMLElement, query: string, count: HTMLElement, moreNote: HTMLElement): void {
  let shown = 0;
  let total = 0;
  for (const table of page.querySelectorAll<HTMLTableElement>("table.device-table")) {
    let visible = 0;
    for (const row of table.tBodies[0]?.rows ?? []) {
      const match = (row.dataset.search ?? "").includes(query);
      row.hidden = !match;
      if (match) visible += 1;
      total += 1;
    }
    shown += visible;
    table.hidden = visible === 0;
    const heading = table.previousElementSibling;
    if (heading instanceof HTMLHeadingElement) heading.hidden = visible === 0;
  }
  moreNote.hidden = moreNote.parentElement?.querySelector("table:not([hidden])") === null;
  count.textContent = shown === 0 ? tr("devices-no-match") : trFill("devices-match-count", { shown, total });
}

/**
 * The rows installed plugins add, one table per plugin under its name.
 *
 * Kept apart from the built-in lists rather than merged into them: a row from
 * a plugin is its author's statement, and the reader should be able to tell
 * whose statement they are reading.
 */
function pluginDevices(loaded: () => void): HTMLElement {
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
              { model: row.model, salesName: null },
              row.arch === null ? markCell(null) : make("td", { textContent: row.arch }),
              markCell(row.apps),
              markCell(row.updater),
              markCell(row.service),
              make("td", { textContent: row.note ?? "" }),
            ]),
          ),
        );
      }
      loaded();
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
    rememberCameraModel(info.modelName);
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

const SUPPORT_MARK: Record<Support, string> = { yes: "✓", no: "✖", untested: "?", unknown: "?" };

/** One line on what the lists say about a model, e.g. "Apps ✓ · Updater shell ✖ · GPS ?". */
async function supportLine(model: string): Promise<string> {
  const capabilities: [Capability, string][] = [
    ["apps", "gps-col-apps"],
    ["updater", "nav-updater"],
    ["gps", "nav-gps"],
  ];
  const checks = await Promise.all(capabilities.map(([capability]) => supportCheck(model, capability)));
  return capabilities
    .map(([, label], i) => `${tr(label)} ${SUPPORT_MARK[checks[i].support ?? "unknown"]}`)
    .join(" · ");
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
    rows.map(({ model, salesName, arch, apps, updater }) => [
      { model, salesName },
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
    rows.map(({ model, salesName, arch, apps, report }) => [
      { model, salesName },
      arch === null ? markCell(null) : make("td", { textContent: arch }),
      markCell(apps),
      markCell(false),
      markCell(report === "disputed" ? null : true),
      markCell(report === "works" ? true : null),
    ]),
  );
}

/** A row's model, with the name Sony sells it under where that differs. */
interface ModelName {
  model: string;
  salesName: string | null;
}

/**
 * What a search matches a row against: the model, its sales name and, for an α
 * body, the short form people write (a7m3 for ILCE-7M3).
 */
function searchKeys({ model, salesName }: ModelName): string {
  const keys = [model, salesName ?? ""];
  if (salesName?.startsWith("α")) keys.push(`a${model.slice(model.indexOf("-") + 1)}`);
  return keys.map(normalized).join(" ");
}

function deviceRows(
  className: string,
  columns: string[],
  rows: [ModelName, ...HTMLTableCellElement[]][],
): HTMLTableElement {
  return make(
    "table",
    { className },
    make("thead", {}, make("tr", {}, ...columns.map((key) => make("th", { scope: "col", i18n: key })))),
    make(
      "tbody",
      {},
      ...rows.map(([name, ...cells]) => {
        const head = make("th", { scope: "row", textContent: name.model });
        if (name.salesName) head.append(make("span", { className: "sales-name", textContent: name.salesName }));
        const row = make("tr", {}, head, ...cells);
        row.dataset.search = searchKeys(name);
        return row;
      }),
    ),
  );
}

/** Model and serial number of the camera last read on this page. */
interface ConnectedCamera {
  model: string | null;
  serial: string | null;
}

/**
 * Read the shutter count from a photo the camera took.
 *
 * The count is not something the camera reports over USB; it is written into
 * every image instead. The newest image therefore carries the current figure,
 * which is why the user is asked for that one.
 *
 * `connected` is read at click time, so a camera queried after the photo was
 * chosen still takes part. Its serial number stands in where the file has
 * none, but only for a photo of the same model.
 */
function cardShutterCount(connected: () => ConnectedCamera): HTMLElement {
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
      const evaluated = new Date().toLocaleString(currentLocale, { dateStyle: "medium", timeStyle: "short" });
      const camera = connected();
      const sameModel =
        camera.model !== null && result.model !== null && camera.model.toUpperCase() === result.model.toUpperCase();
      const serial = result.serial ?? (sameModel ? camera.serial : null);
      const model = result.model && result.salesName ? `${result.model} (${result.salesName})` : result.model;
      details.append(
        fieldList([
          ["shutter-file", result.fileName],
          ["shutter-taken", result.taken],
          ["shutter-count", result.shutterCount?.toLocaleString(currentLocale) ?? null],
          ["shutter-type", result.shutter ? { key: `shutter-type-${result.shutter}` } : null],
          ["info-model", model],
          ["info-serial", serial ?? { key: "shutter-serial-unknown" }],
          ["info-firmware", result.software],
          ["info-lens", result.lens],
          ["shutter-evaluated", evaluated],
        ]),
      );
      const notes: string[] = [];
      if (result.countUnavailable) notes.push(`shutter-${result.countUnavailable}`);
      if (result.shutter === "electronic") notes.push("shutter-electronic-hint");
      for (const key of notes) details.append(make("p", { className: "hint", i18n: key }));
      if (camera.model && result.model && !sameModel) {
        notes.push("shutter-model-mismatch");
        details.append(
          make(
            "div",
            { className: "notice-block" },
            make("p", { i18n: "shutter-model-mismatch" }),
          ),
        );
      }
      details.append(reportControls(result, serial, evaluated, notes));
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
 * Format choice, the serial-number opt-in and the save button for the report.
 *
 * The serial number stays off the saved file unless the user ticks the box: a
 * report is often passed on, the window is not.
 */
function reportControls(
  photo: PhotoSummary,
  serial: string | null,
  evaluated: string,
  noteKeys: string[],
): HTMLElement {
  const status = statusLine();
  const format = make("select", { className: "field-select" });
  for (const [value, label] of [
    ["pdf", "PDF"],
    ["jpeg", "JPEG"],
    ["png", "PNG"],
  ]) {
    format.append(make("option", { value, textContent: label }));
  }
  const withSerial = make("input", { type: "checkbox", disabled: serial === null });
  const save = make("button", { i18n: "btn-shutter-report" });
  save.addEventListener("click", async () => {
    save.disabled = true;
    showStatus(status, "");
    try {
      const paper = paperFor(currentLocale);
      const page = await drawReport(
        { photo, serial: withSerial.checked ? serial : null, evaluated, notes: noteKeys.map(tr) },
        paper,
      );
      const day = new Date().toISOString().slice(0, 10);
      const name = `shutter-count_${photo.model ?? "camera"}_${day}`;
      const path = await saveReport(page, format.value as ReportFormat, paper, name);
      if (path !== null) showDone(status, trFill("tools-written", { path }));
    } catch (e) {
      showError(status, e);
    } finally {
      save.disabled = false;
    }
  });
  return make(
    "div",
    { className: "shutter-report" },
    make("label", { className: "field-label", i18n: "shutter-report-format" }),
    format,
    make("label", { className: "row" }, withSerial, make("span", { i18n: "shutter-report-serial" })),
    make("div", { className: "row" }, save),
    status,
  );
}
