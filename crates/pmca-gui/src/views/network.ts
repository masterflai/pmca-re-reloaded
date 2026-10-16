/** Network page: GPS assist data, Wi-Fi and streaming settings. */

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
  writing,
} from "../dom";
import { browseButton, registerDragDrop, setFileDrop } from "../drop";
import type { StreamingSettings, WifiAccessPoint } from "../dto";
import { tr } from "../i18n";
import { lowRiskCheck } from "./camera";

export function viewNetwork(host: HTMLElement): void {
  host.append(cardGps(), cardWifi(), cardStreaming());
}

/**
 * One row of the GPS device list: model, GPS variant, architecture, app support.
 * App support is `null` where it is not known.
 */
type GpsModel = [string, string, string, boolean | null];

/**
 * Cameras with a built-in GPS receiver, grouped as on the card.
 *
 * Model and architecture follow the OpenMemories device list, which names a
 * model family without saying which variant carries the receiver; that is
 * Sony's "V" suffix, given in the second column. App support is the list's
 * own column: whether the model runs PlayMemories Camera Apps.
 */
export const GPS_MODELS: [string, GpsModel[]][] = [
  [
    "gps-devices-compact",
    [
      ["DSC-HX5V", "DSC-HX5V", "CXD4115", false],
      ["DSC-HX9V", "DSC-HX9V", "CXD4132", false],
      ["DSC-HX20", "DSC-HX20V", "CXD4132", false],
      ["DSC-HX30", "DSC-HX30V", "CXD4132", false],
      ["DSC-HX50", "DSC-HX50V", "CXD4132", false],
      ["DSC-HX60", "DSC-HX60V", "CXD90014", true],
      ["DSC-HX90", "DSC-HX90V", "CXD90014", true],
      ["DSC-HX100V", "DSC-HX100V", "CXD4132", false],
      ["DSC-HX200V", "DSC-HX200V", "CXD4132", false],
      ["DSC-HX400", "DSC-HX400V", "CXD90014", true],
    ],
  ],
  [
    "gps-devices-slt",
    [
      ["SLT-A55", "SLT-A55V", "CXD4115", false],
      ["SLT-A65", "SLT-A65V", "CXD4132", false],
      ["SLT-A77", "SLT-A77V", "CXD4132", false],
      ["SLT-A99", "SLT-A99V", "CXD4132", false],
    ],
  ],
  [
    "gps-devices-action",
    [
      ["HDR-AS30V", "HDR-AS30V", "CXD4132", false],
      ["HDR-AS100V", "HDR-AS100V", "CXD90014", true],
      ["HDR-AS200V", "HDR-AS200V", "CXD90014", true],
      ["FDR-X1000V", "FDR-X1000V", "CXD90014", true],
    ],
  ],
  [
    "gps-devices-camcorder",
    [
      ["HDR-CX560", "HDR-CX560V", "CXD4132", false],
      ["HDR-CX700", "HDR-CX700V", "CXD4132", false],
      ["HDR-XR550", "HDR-XR550V", "CXD4120", false],
      ["HDR-PJ590V", "HDR-PJ590V", "CXD4132", false],
      ["HDR-PJ760V", "HDR-PJ760V", "CXD4132", false],
      ["HDR-PJ790V", "HDR-PJ790V", "CXD4132", false],
      ["HDR-TD10", "HDR-TD10", "CXD4132", false],
      ["HDR-TD20", "HDR-TD20V", "CXD4132", false],
    ],
  ],
];

/**
 * Models known to carry a GPS receiver that the OpenMemories device list does
 * not name. Nobody has confirmed that this application can talk to them, so
 * neither their architecture nor their app support is known.
 */
export const GPS_MODELS_UNTESTED: GpsModel[] = [
  ["DSC-HX7V", "DSC-HX7V", "\u2013", null],
  ["DSC-HX10V", "DSC-HX10V", "\u2013", null],
  ["DSC-TX100V", "DSC-TX100V", "\u2013", null],
  ["DSC-TX200V", "DSC-TX200V", "\u2013", null],
  ["DSC-TX300V", "DSC-TX300V", "\u2013", null],
  ["HDR-XR520V", "HDR-XR520V", "\u2013", null],
  ["HDR-CX550V", "HDR-CX550V", "\u2013", null],
  ["HDR-CX260V", "HDR-CX260V", "\u2013", null],
  ["HDR-PJ260V", "HDR-PJ260V", "\u2013", null],
  ["HDR-CX580V", "HDR-CX580V", "\u2013", null],
  ["HDR-PJ580V", "HDR-PJ580V", "\u2013", null],
  ["HDR-PJ710V", "HDR-PJ710V", "\u2013", null],
  ["HDR-CX760V", "HDR-CX760V", "\u2013", null],
  ["HDR-TD30V", "HDR-TD30V", "\u2013", null],
  ["HXR-NX5", "HXR-NX5", "\u2013", null],
  ["HXR-NX30", "HXR-NX30", "\u2013", null],
  ["HXR-NX70", "HXR-NX70", "\u2013", null],
];

/**
 * A yes/no/unknown cell: a green tick, a red cross or a question mark.
 *
 * The glyph is hidden from screen readers, which would announce it as "heavy
 * check mark"; they read the translated word beside it instead.
 */
export function markCell(value: boolean | null, detail?: string): HTMLTableCellElement {
  const [state, glyph, key] =
    value === null
      ? ["unknown", "?", "gps-unknown"]
      : value
        ? ["yes", "\u2714", "gps-yes"]
        : ["no", "\u2716", "gps-no"];
  return make(
    "td",
    { className: `mark ${state}` },
    make("span", { textContent: glyph, ariaHidden: "true" }),
    make("span", { className: "visually-hidden", i18n: key }),
    ...(detail ? [make("span", { className: "mark-detail", textContent: ` (${detail})` })] : []),
  );
}

/** The button that shows and hides a device list, which starts hidden. */
export function deviceToggle(
  devices: HTMLElement,
  show = "btn-gps-devices",
  hide = "btn-gps-devices-hide",
): HTMLButtonElement {
  devices.hidden = true;
  const toggle = make("button", { i18n: show });
  toggle.setAttribute("aria-expanded", "false");
  toggle.addEventListener("click", () => {
    devices.hidden = !devices.hidden;
    toggle.setAttribute("aria-expanded", String(!devices.hidden));
    toggle.dataset.i18n = devices.hidden ? show : hide;
    toggle.textContent = tr(toggle.dataset.i18n);
  });
  return toggle;
}

function gpsDeviceTable(rows: GpsModel[]): HTMLTableElement {
  const columns = ["info-model", "gps-col-variant", "gps-col-arch", "gps-col-module", "gps-col-apps"];
  return make(
    "table",
    { className: "device-table" },
    make("thead", {}, make("tr", {}, ...columns.map((key) => make("th", { scope: "col", i18n: key })))),
    make(
      "tbody",
      {},
      ...rows.map(([model, variant, arch, apps]) =>
        make(
          "tr",
          {},
          make("th", { scope: "row", textContent: model }),
          make("td", { textContent: variant }),
          make("td", { textContent: arch }),
          // Every model listed here has the receiver; that is why it is listed.
          markCell(true),
          markCell(apps),
        ),
      ),
    ),
  );
}

/**
 * GPS assist upload. Empty path means "download the current file from Sony".
 *
 * Unlike {@link filePicker} the field stays typeable: leaving it empty is the
 * usual answer here, and the file is small enough to name from memory.
 */
function cardGps(): HTMLElement {
  const status = statusLine();
  const path = make("input", {
    type: "text",
    className: "path-input",
    placeholder: "assistme.dat",
  });
  const accept = (file: string): void => {
    path.value = file;
  };
  const browse = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind: "gps" }),
    accept,
  );
  const zone = make(
    "div",
    { className: "dropzone file-picker" },
    make("span", { className: "file-picker-hint", i18n: "gps-hint" }),
    make("div", { className: "gps-picker-field" }, path, browse),
  );
  setFileDrop({ suffixes: [".dat"], accept });
  void registerDragDrop(zone);

  const run = make("button", { className: "primary", i18n: "btn-update-gps" });
  run.addEventListener("click", async () => {
    run.disabled = true;
    try {
      if (!(await lowRiskCheck(status, "gps"))) return;
      showBusy(status, tr("gps-writing"));
      await invoke("update_gps", { file: path.value.trim() || null });
      showDone(status, tr("msg-done"));
    } catch (e) {
      showError(status, e, () => run.click());
    } finally {
      run.disabled = false;
    }
  });

  const devices = make(
    "div",
    { className: "gps-devices" },
    make("p", { className: "hint", i18n: "gps-devices-note" }),
    ...GPS_MODELS.flatMap(([heading, rows]) => [
      make("h3", { i18n: heading }),
      gpsDeviceTable(rows),
    ]),
    make("h3", { i18n: "gps-devices-untested" }),
    make("p", { className: "hint", i18n: "gps-devices-untested-note" }),
    gpsDeviceTable(GPS_MODELS_UNTESTED),
  );

  return card(
    make("h2", { i18n: "gps-heading" }),
    zone,
    make("div", { className: "row" }, writing(run), deviceToggle(devices)),
    status,
    devices,
  );
}

/**
 * Access points stored for live streaming.
 *
 * These belong to the camera's live-streaming plug-in, not to the Wi-Fi the
 * camera uses for app installation, so a body without a streaming feature
 * refuses the whole card.
 *
 * Passphrases are stored by the camera in clear text; they are rendered as
 * password fields so a screenshot or a bystander does not leak them, with an
 * explicit reveal toggle.
 */
function cardWifi(): HTMLElement {
  const status = statusLine();
  const list = make("div", { className: "app-list" });
  const multi = make("input", { type: "checkbox" });
  let current: WifiAccessPoint[] = [];

  const save = make("button", { className: "primary", i18n: "btn-save" });
  save.disabled = true;

  const render = (): void => {
    list.replaceChildren();
    // Writing replaces the whole list, so saving an empty one would erase every
    // configured network. The backend refuses it too.
    save.disabled = current.length === 0;
    if (current.length === 0) {
      list.append(make("p", { i18n: "wifi-none" }));
      return;
    }
    current.forEach((ap, index) => {
      const ssid = make("input", { type: "text", value: ap.ssid, placeholder: "SSID" });
      ssid.addEventListener("input", () => {
        current[index].ssid = ssid.value;
      });
      const key = make("input", { type: "password", value: ap.key });
      key.addEventListener("input", () => {
        current[index].key = key.value;
      });
      const reveal = make("button", { i18n: "btn-reveal" });
      reveal.addEventListener("click", () => {
        key.type = key.type === "password" ? "text" : "password";
      });
      const remove = make("button", { i18n: "btn-remove" });
      remove.addEventListener("click", () => {
        current.splice(index, 1);
        render();
      });
      list.append(make("div", { className: "app-entry" }, ssid, key, reveal, remove));
    });
  };

  const load = make("button", { i18n: "btn-refresh" });
  load.addEventListener("click", async () => {
    load.disabled = true;
    showBusy(status, tr("wifi-read"));
    try {
      current = await invoke<WifiAccessPoint[]>("list_wifi", { multi: multi.checked });
      render();
      showStatus(status, tr("wifi-secret-warning"));
    } catch (e) {
      showError(status, e, () => load.click());
    } finally {
      load.disabled = false;
    }
  });

  const add = make("button", { i18n: "btn-add" });
  add.addEventListener("click", () => {
    current.push({ keyType: 0, ssid: "", key: "" });
    render();
  });

  save.addEventListener("click", async () => {
    if (!(await confirmed(tr("wifi-confirm"), tr("btn-confirm-wifi")))) return;
    save.disabled = true;
    load.disabled = true;
    showBusy(status, tr("wifi-writing"));
    try {
      current = await invoke<WifiAccessPoint[]>("save_wifi", {
        multi: multi.checked,
        accessPoints: current,
      });
      render();
      showDone(status, tr("wifi-written"));
    } catch (e) {
      showError(status, e);
    } finally {
      load.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "wifi-heading" }),
    make("p", { i18n: "wifi-hint" }),
    make("p", { className: "warning", i18n: "wifi-secret-warning" }),
    make(
      "div",
      { className: "row" },
      make("label", { className: "app-entry" }, multi, make("span", { i18n: "wifi-multi" })),
      load,
      add,
      writing(save),
    ),
    status,
    list,
  );
}

/**
 * Live-streaming settings.
 *
 * The camera exposes far more fields here than a form can usefully present, so
 * the settings are edited as JSON — the same document the `stream` CLI command
 * reads and writes.
 */
function cardStreaming(): HTMLElement {
  const status = statusLine();
  const editor = make("textarea", { className: "json-editor", rows: 16, spellcheck: false });

  const save = make("button", { className: "primary", i18n: "btn-save" });
  save.disabled = true;

  const load = make("button", { i18n: "btn-refresh" });
  load.addEventListener("click", async () => {
    load.disabled = true;
    showBusy(status, tr("stream-read"));
    try {
      const settings = await invoke<StreamingSettings>("get_streaming_settings");
      editor.value = JSON.stringify(settings, null, 2);
      save.disabled = false;
      showStatus(status, tr("net-secret-warning"));
    } catch (e) {
      showError(status, e, () => load.click());
    } finally {
      load.disabled = false;
    }
  });

  save.addEventListener("click", async () => {
    // Parse before prompting so a typo is reported without a scary dialog.
    let settings: StreamingSettings;
    try {
      settings = JSON.parse(editor.value) as StreamingSettings;
    } catch (e) {
      showError(status, `${tr("stream-invalid-json")} ${String(e)}`);
      return;
    }
    if (!(await confirmed(tr("stream-confirm"), tr("btn-confirm-stream")))) return;
    save.disabled = true;
    load.disabled = true;
    showBusy(status, tr("stream-writing"));
    try {
      const written = await invoke<StreamingSettings>("save_streaming_settings", { settings });
      editor.value = JSON.stringify(written, null, 2);
      showDone(status, tr("stream-written"));
    } catch (e) {
      showError(status, e);
    } finally {
      load.disabled = false;
      save.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "stream-heading" }),
    make("p", { className: "warning", i18n: "net-secret-warning" }),
    make("div", { className: "row" }, load, writing(save)),
    status,
    editor,
  );
}
