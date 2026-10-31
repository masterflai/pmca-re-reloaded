/** App installation page. */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  chosen,
  fieldList,
  make,
  showBusy,
  showDone,
  showError,
  showStateKey,
  showStatus,
  statusLine,
  withProgressTarget,
} from "../dom";
import { filePicker } from "../drop";
import type {
  CommunityApp,
  CommunityList,
  DownloadedApp,
  InstallOutcome,
  InstalledDevice,
  SourcesState,
} from "../dto";
import { errorText, tr, trFill } from "../i18n";
import { cardAppBackup } from "./app-backup";
import { capableDevices, lowRiskCheck } from "./camera";
import { deviceToggle } from "./network";
import { disclosure, entryDetails, sourceRepo, sourceSearch, starsBadge } from "./sources";

export function viewInstall(host: HTMLElement): void {
  const status = statusLine();
  const details = make("div");

  // Sony retired the official store; every one of its endpoints answers 404.
  // Say so up front instead of offering a button that cannot work.
  const officialNotice = make(
    "div",
    { className: "notice-block" },
    make("p", { i18n: "store-discontinued" }),
    make("p", { i18n: "store-use-own-apk" }),
  );

  // Tauri hands us real filesystem paths for dropped files, so an .apk can be
  // installed straight from disk without copying it through the IPC channel.
  const apkFile = filePicker(status, "apk", [".apk"], "install-drop-hint");

  /** Render the result of installing an .apk file. */
  const showOutcome = (outcome: InstallOutcome): void => {
    showDone(status, tr("install-done"));
    details.replaceChildren();
    if (outcome.apk) {
      details.append(fieldList([["apk-package", outcome.apk.package]]));
    }
    if (outcome.device) details.append(deviceFields(outcome.device));
  };

  const install = make("button", { className: "primary", i18n: "btn-install-file" });
  const appDevices = capableDevices("apps", "devices-note");
  install.addEventListener("click", async () => {
    lock(true);
    details.replaceChildren();
    try {
      const apk = apkFile.path();
      // Without a file the camera is only served an empty task: nothing to check.
      if (apk !== "" && !(await lowRiskCheck(status, "apps"))) return;
      showBusy(status, tr("install-starting"));
      const outcome = await invoke<InstallOutcome>("install_app", {
        apk: apk === "" ? null : apk,
      });
      showOutcome(outcome);
      const apps = cameraApps(outcome);
      if (apps) showInstalled(apps);
    } catch (e) {
      showError(status, e);
    } finally {
      lock(false);
    }
  });

  // Community store: a table to pick any number of apps from, installed one
  // after the other with a status of its own per row.
  const storeStatus = statusLine();
  const storeDetails = make("div");
  const loadCommunity = make("button", { i18n: "btn-load-apps" });
  // A source added from here is read at once, so its apps join the list.
  const search = sourceSearch(true, () => {
    if (!running) loadCommunity.click();
  });
  const sync = make("button", { i18n: "btn-store-sync", disabled: true });
  const countLabel = make("span");
  const installSelected = make(
    "button",
    { className: "primary", disabled: true },
    make("span", { i18n: "btn-install-selected" }),
    countLabel,
  );
  const stop = make("button", { i18n: "btn-store-stop", hidden: true });
  const retry = make("button", { i18n: "btn-store-retry", hidden: true });
  const selectAll = make("input", { type: "checkbox", disabled: true });
  const sourceFilter = make("select", { className: "field-select", id: "store-source-filter" });
  // Only worth offering once the list comes from more than one source.
  const filterBlock = make(
    "div",
    { hidden: true },
    make("label", {
      className: "field-label",
      htmlFor: "store-source-filter",
      i18n: "store-filter-source",
    }),
    sourceFilter,
  );
  const tbody = make("tbody");
  const table = make(
    "table",
    { className: "store-table", hidden: true },
    make(
      "thead",
      {},
      make(
        "tr",
        {},
        make(
          "th",
          { scope: "col" },
          make(
            "label",
            {},
            selectAll,
            make("span", { className: "visually-hidden", i18n: "store-select-all" }),
          ),
        ),
        ...[
          "store-col-app",
          "store-col-version",
          "store-col-author",
          "store-col-source",
          "store-col-camera",
          "store-col-status",
        ].map((key) => make("th", { scope: "col", i18n: key })),
      ),
    ),
    tbody,
  );

  /** How the last batch ended for one app. */
  type BatchResult = "done" | "failed" | "skipped" | "not-run";

  interface StoreRow {
    app: CommunityApp;
    check: HTMLInputElement;
    version: HTMLTableCellElement;
    camera: HTMLTableCellElement;
    status: HTMLElement;
    result: BatchResult | null;
    appRow: HTMLTableRowElement;
    detailRow: HTMLTableRowElement;
    name: HTMLTableCellElement;
  }

  let rows: StoreRow[] = [];
  let running = false;
  let stopRequested = false;
  /** The camera's installed apps, once a task has reported them. */
  let known: CameraApps | null = null;
  /** Bumped on every list load, so a lookup for an older list stops. */
  let listGeneration = 0;

  /**
   * Look up each app's newest store version, one app at a time.
   *
   * The list itself does not carry versions; most apps publish on GitHub,
   * whose API is rate-limited, so the lookups run in sequence rather than all
   * at once. The backend keeps the answers, and a later download reuses them.
   */
  const fillVersions = async (generation: number): Promise<void> => {
    for (const row of rows) {
      if (generation !== listGeneration) return;
      if (row.app.installable) {
        try {
          const version = await invoke<string>("community_app_version", {
            source: row.app.source,
            package: row.app.package,
          });
          if (generation === listGeneration && row.version.textContent === "…") {
            row.version.textContent = version;
          }
        } catch (e) {
          if (generation !== listGeneration) return;
          row.version.textContent = "–";
          row.version.title = errorText(e);
        }
      }
      await markArchived(row, generation);
    }
  };

  /** Flag an app whose GitHub repository its developer has archived. */
  const markArchived = async (row: StoreRow, generation: number): Promise<void> => {
    const id = row.app.repository?.replace(/^https:\/\/github\.com\//i, "");
    if (!id) return;
    const repo = await sourceRepo(id);
    if (generation !== listGeneration || !repo?.archived) return;
    const badge = make("span", { className: "archived-badge", i18n: "sources-archived" });
    row.name.querySelector("label")?.after(badge);
    const note = row.detailRow.querySelector<HTMLElement>(".archived-note");
    if (note) note.hidden = false;
  };

  /** Fill the camera's version column from the last report. */
  const showInstalled = (apps: CameraApps): void => {
    known = apps;
    for (const row of rows) setCameraCell(row);
  };

  const setCameraCell = (row: StoreRow): void => {
    const cell = row.camera;
    delete cell.dataset.i18n;
    cell.textContent = "";
    if (!known) return;
    const version = known.get(row.app.package);
    if (version === undefined) {
      cell.dataset.i18n = "store-camera-absent";
      cell.textContent = tr("store-camera-absent");
    } else if (version === null) {
      cell.dataset.i18n = "store-state-installed";
      cell.textContent = tr("store-state-installed");
    } else {
      cell.textContent = version;
    }
  };

  const updateSelection = (): void => {
    const selectable = rows.filter((row) => row.app.installable && !row.appRow.hidden);
    const count = selectable.filter((row) => row.check.checked).length;
    countLabel.textContent = count > 0 ? ` (${count})` : "";
    installSelected.disabled = running || count === 0;
    selectAll.checked = count > 0 && count === selectable.length;
    selectAll.indeterminate = count > 0 && count < selectable.length;
  };

  /** Show only the rows of the chosen source; a hidden row is deselected. */
  const applyFilter = (): void => {
    const wanted = sourceFilter.value;
    for (const row of rows) {
      const hide = wanted !== "" && row.app.source !== wanted;
      row.appRow.hidden = hide;
      row.detailRow.hidden = hide || !row.appRow.classList.contains("expanded");
      if (hide) row.check.checked = false;
    }
    lock(running);
  };
  sourceFilter.addEventListener("change", applyFilter);

  /** Block every install while one is running, in either card. */
  const lock = (on: boolean): void => {
    running = on;
    install.disabled = on;
    loadCommunity.disabled = on;
    search.button.disabled = on;
    sourceFilter.disabled = on;
    sync.disabled = on || rows.length === 0;
    selectAll.disabled = on || !rows.some((row) => row.app.installable && !row.appRow.hidden);
    for (const row of rows) row.check.disabled = on || !row.app.installable;
    updateSelection();
  };

  selectAll.addEventListener("change", () => {
    for (const row of rows) {
      if (row.app.installable && !row.appRow.hidden) row.check.checked = selectAll.checked;
    }
    updateSelection();
  });

  loadCommunity.addEventListener("click", async () => {
    lock(true);
    storeDetails.replaceChildren();
    retry.hidden = true;
    showBusy(storeStatus, tr("store-loading"));
    tbody.replaceChildren();
    rows = [];
    table.hidden = true;
    filterBlock.hidden = true;
    try {
      const state = await invoke<SourcesState>("sources_list");
      if (state.sources.length === 0) {
        // Nothing to list yet: offer the suggestions one by one, this program's own list first.
        const accepted: string[] = [];
        for (const suggestion of state.suggestions) {
          const yes = await chosen(
            trFill(suggestion.promptKey, { source: suggestion.source.id }),
            tr("btn-activate-list"),
            tr("btn-cancel"),
          );
          if (yes) accepted.push(suggestion.source.id);
        }
        if (accepted.length === 0) {
          showStatus(storeStatus, tr("store-no-sources"));
          return;
        }
        showBusy(storeStatus, tr("sources-adding"));
        for (const id of accepted) await invoke("sources_add", { spec: id });
        showBusy(storeStatus, tr("store-loading"));
      }
      const list = await invoke<CommunityList>("list_community_apps");
      const apps = list.entries;
      for (const failure of list.failed) {
        storeDetails.append(
          make("p", { className: "error-text", textContent: trFill("source-failed", { ...failure }) }),
        );
      }
      // An empty list is not a success worth ticking off — nothing was found.
      if (apps.length === 0) {
        showStatus(storeStatus, tr("msg-no-apps"));
      } else {
        showDone(storeStatus, trFill("msg-apps-found", { count: apps.length }));
      }
      apps.forEach((app, index) => {
        const id = `store-app-${index}`;
        const check = make("input", { type: "checkbox", id });
        check.addEventListener("change", updateSelection);
        const status = make("p", { className: "status row-status" });
        if (!app.installable) showStateKey(status, "idle", "store-not-installable");
        const version = make("td", { textContent: app.installable ? "…" : "–" });
        const camera = make("td");
        const detailRow = make(
          "tr",
          { className: "store-detail-row" },
          make("td", { colSpan: 7 }, entryDetails(app)),
        );
        const nameCell = make(
          "th",
          { scope: "row" },
          make("label", { htmlFor: id, textContent: app.name, title: app.description ?? "" }),
        );
        const appRow = make(
          "tr",
          { className: app.installable ? "" : "unavailable" },
          make("td", {}, check),
          nameCell,
          version,
          make("td", { textContent: app.author ?? "" }),
          make("td", {}, `${app.source} `, starsBadge(app.source)),
          camera,
          make("td", {}, status),
        );
        // The row's rule would otherwise cut the app off from its own details.
        nameCell.append(disclosure(detailRow, false, (open) => appRow.classList.toggle("expanded", open)));
        tbody.append(appRow, detailRow);
        const row: StoreRow = {
          app,
          check,
          version,
          camera,
          status,
          result: null,
          appRow,
          detailRow,
          name: nameCell,
        };
        setCameraCell(row);
        rows.push(row);
      });
      const sources = [...new Set(apps.map((app) => app.source))];
      sourceFilter.replaceChildren(
        make("option", { value: "", i18n: "store-filter-all" }),
        ...sources.map((source) => make("option", { value: source, textContent: source })),
      );
      filterBlock.hidden = sources.length < 2;
      table.hidden = apps.length === 0;
      void fillVersions(++listGeneration);
    } catch (e) {
      showError(storeStatus, e);
    } finally {
      lock(false);
    }
  });

  sync.addEventListener("click", async () => {
    lock(true);
    storeDetails.replaceChildren();
    showBusy(storeStatus, tr("store-batch-query"));
    try {
      const outcome = await withProgressTarget(storeStatus, () =>
        invoke<InstallOutcome>("install_app", { apk: null }),
      );
      const apps = cameraApps(outcome);
      if (apps) {
        showInstalled(apps);
        const count = rows.filter((row) => apps.has(row.app.package)).length;
        showDone(storeStatus, trFill("store-sync-done", { count }));
      } else {
        showDone(storeStatus, tr("install-done"));
      }
      if (outcome.device) storeDetails.append(deviceFields(outcome.device));
    } catch (e) {
      showError(storeStatus, e);
    } finally {
      lock(false);
    }
  });

  stop.addEventListener("click", () => {
    stopRequested = true;
    stop.disabled = true;
    stop.dataset.i18n = "btn-store-stopping";
    stop.textContent = tr("btn-store-stopping");
  });

  /**
   * Install the given apps one after the other.
   *
   * Every app is downloaded before the camera is touched, so a network failure
   * cannot leave it stranded in app-install mode. A failed download only costs
   * that app; a failure on the camera stops the run, because the next app
   * would meet the same camera in the same state.
   */
  const runBatch = async (selected: StoreRow[]): Promise<void> => {
    lock(true);
    stopRequested = false;
    stop.dataset.i18n = "btn-store-stop";
    stop.textContent = tr("btn-store-stop");
    stop.disabled = false;
    stop.hidden = false;
    retry.hidden = true;
    storeDetails.replaceChildren();
    for (const row of rows) {
      row.result = null;
      if (!row.app.installable) continue;
      if (selected.includes(row)) showStateKey(row.status, "queued", "store-state-queued");
      else showStatus(row.status, "");
    }

    const versions = new Map<StoreRow, string>();
    let current: StoreRow | null = null;
    let cameraError: unknown = null;
    let device: InstalledDevice | null = null;

    try {
      for (const [index, row] of selected.entries()) {
        if (stopRequested) break;
        showBusy(
          storeStatus,
          trFill("store-batch-downloading", { current: index + 1, total: selected.length }),
          Math.round((index / selected.length) * 100),
        );
        showBusy(row.status, tr("store-loading"));
        try {
          const download = await withProgressTarget(row.status, () =>
            invoke<DownloadedApp>("download_community_app", {
              source: row.app.source,
              package: row.app.package,
            }),
          );
          versions.set(row, download.version);
          // The APK's own versionName, which is what the camera will report.
          row.version.textContent = download.version;
          row.version.removeAttribute("title");
          showStateKey(row.status, "queued", "store-state-queued");
        } catch (e) {
          row.result = "failed";
          showError(row.status, e);
        }
      }

      const pending = selected.filter((row) => versions.has(row));
      if (pending.length > 0 && !stopRequested) {
        // The camera only lists its apps at the end of a task, so the
        // reinstall check below needs one empty task up front.
        showBusy(storeStatus, tr("store-batch-query"));
        const outcome = await withProgressTarget(storeStatus, () =>
          invoke<InstallOutcome>("install_app", { apk: null, series: true }),
        );
        device = outcome.device;
        const apps = cameraApps(outcome);
        if (apps) showInstalled(apps);
      }

      for (const [index, row] of pending.entries()) {
        if (stopRequested) break;
        showBusy(
          storeStatus,
          trFill("store-batch-installing", {
            current: index + 1,
            total: pending.length,
            name: row.app.name,
          }),
          Math.round((index / pending.length) * 100),
        );
        const installed = known?.get(row.app.package);
        if (installed !== undefined) {
          const proceed = await chosen(
            trFill("store-reinstall-confirm", {
              name: row.app.name,
              installed: installed ?? tr("store-version-unknown"),
              store: versions.get(row) ?? tr("store-version-unknown"),
            }),
            tr("btn-reinstall"),
            tr("btn-skip"),
          );
          if (!proceed) {
            row.result = "skipped";
            showStateKey(row.status, "skipped", "store-state-skipped");
            continue;
          }
        }
        current = row;
        showBusy(row.status, tr("install-starting"));
        const outcome = await withProgressTarget(row.status, () =>
          invoke<InstallOutcome>("install_downloaded_app", {
            source: row.app.source,
            package: row.app.package,
          }),
        );
        current = null;
        row.result = "done";
        device = outcome.device ?? device;
        const apps = cameraApps(outcome) ?? new Map(known ?? []);
        // Should the camera's report predate the install, the app is still on it.
        if (!apps.has(row.app.package)) apps.set(row.app.package, versions.get(row) ?? null);
        showInstalled(apps);
        showStateKey(row.status, "ok", "store-state-installed");
      }
    } catch (e) {
      cameraError = e;
      if (current) {
        current.result = "failed";
        showStateKey(current.status, "error", "store-state-failed");
        current.status.title = errorText(e);
      }
    }

    for (const row of selected) {
      if (row.result !== null) continue;
      row.result = "not-run";
      showStateKey(row.status, "idle", "store-state-not-run");
    }
    await invoke("discard_downloads").catch(() => undefined);
    // The tasks above leave the camera in app-install mode, which is where the
    // next one finds it. Only the end of the batch waits for it to leave, so
    // a camera that stays there is reported once, here, rather than per app.
    await withProgressTarget(storeStatus, () => invoke("finish_install_series")).catch(
      () => undefined,
    );

    const count = (result: BatchResult): number =>
      selected.filter((row) => row.result === result).length;
    const summary = trFill("store-batch-summary", {
      done: count("done"),
      failed: count("failed"),
      skipped: count("skipped"),
      notrun: count("not-run"),
    });
    if (cameraError !== null) {
      showError(storeStatus, cameraError);
      storeDetails.append(make("p", { className: "hint", textContent: summary }));
    } else if (count("done") === selected.length) {
      showDone(storeStatus, summary);
    } else {
      showStatus(storeStatus, summary);
    }
    if (device) storeDetails.append(deviceFields(device));

    stop.hidden = true;
    retry.hidden = count("failed") + count("not-run") === 0;
    lock(false);
  };

  installSelected.addEventListener("click", async () => {
    lock(true);
    storeDetails.replaceChildren();
    const go = await lowRiskCheck(storeStatus, "apps");
    lock(false);
    if (go) void runBatch(rows.filter((row) => row.app.installable && row.check.checked));
  });

  retry.addEventListener("click", () => {
    const again = rows.filter((row) => row.result === "failed" || row.result === "not-run");
    for (const row of rows) row.check.checked = again.includes(row);
    void runBatch(again);
  });

  host.append(
    card(
      officialNotice,
      make("label", { className: "field-label", i18n: "install-apk-path" }),
      apkFile.element,
      make("div", { className: "row" }, install, deviceToggle(appDevices)),
      status,
      details,
      appDevices,
    ),
    card(
      make("h2", { i18n: "store-heading" }),
      make("p", { className: "hint", i18n: "store-sources-hint" }),
      make("div", { className: "row" }, loadCommunity, sync, search.button),
      search.status,
      search.results,
      storeStatus,
      storeDetails,
      filterBlock,
      make("div", { className: "store-list" }, table),
      make("div", { className: "row" }, installSelected, stop, retry),
    ),
    cardAppBackup(),
  );
}

/** Installed apps by package, with the `versionName` the camera reports. */
type CameraApps = Map<string, string | null>;

/**
 * The apps a camera lists in its report at the end of a task.
 *
 * `null` when the report has no such list, which is not the same as a camera
 * without apps.
 */
function cameraApps(outcome: InstallOutcome): CameraApps | null {
  const raw = outcome.raw;
  if (typeof raw !== "object" || raw === null) return null;
  const list = (raw as Record<string, unknown>).applications;
  if (!Array.isArray(list)) return null;
  const apps: CameraApps = new Map();
  for (const entry of list as { name?: unknown; version?: unknown }[]) {
    if (typeof entry?.name !== "string") continue;
    apps.set(
      entry.name,
      typeof entry.version === "string" ? entry.version.replace(/^[vV](?=\d)/, "") : null,
    );
  }
  return apps;
}

/** The camera identity reported at the end of an installation. */
function deviceFields(device: InstalledDevice): HTMLElement {
  return fieldList([
    ["info-model", device.name],
    ["info-product-code", device.productCode],
    ["info-serial", device.deviceId],
    ["info-firmware", device.firmwareVersion],
  ]);
}
