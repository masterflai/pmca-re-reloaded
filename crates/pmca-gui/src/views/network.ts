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
import type { GpsModel, StreamingSettings, WifiAccessPoint } from "../dto";
import { tr } from "../i18n";
import { deviceLists, lowRiskCheck } from "./camera";

export function viewNetwork(host: HTMLElement): void {
  host.append(cardGps(), cardWifi(), cardStreaming());
}

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
      ...rows.map(({ model, variant, arch, apps }) =>
        make(
          "tr",
          {},
          make("th", { scope: "row", textContent: model }),
          make("td", { textContent: variant }),
          make("td", { textContent: arch ?? "\u2013" }),
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
    make("div", { className: "gps-picker-field" }, browse, path),
    make("span", { className: "file-picker-hint", i18n: "gps-hint" }),
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
  );
  deviceLists()
    .then((lists) =>
      devices.append(
        ...lists.gps.flatMap((g) => [make("h3", { i18n: g.heading }), gpsDeviceTable(g.models)]),
        make("h3", { i18n: "gps-devices-untested" }),
        make("p", { className: "hint", i18n: "gps-devices-untested-note" }),
        gpsDeviceTable(lists.gpsUntested),
      ),
    )
    .catch(() => undefined);

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
