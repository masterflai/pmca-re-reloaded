/**
 * PMCA GUI front-end entry point.
 *
 * Responsibilities:
 *  - Load the shared Fluent (.ftl) language files and apply translations to
 *    every element carrying a `data-i18n` key (same catalog the CLI uses).
 *  - Wire up the left navigation rail and the language / theme pickers.
 *  - Invoke the Rust backend (Tauri commands in `src-tauri`) for the actual
 *    camera work; all orchestration lives in the `pmca-app` crate.
 *
 * The translation catalog is intentionally the single source of truth living
 * in /locales, so adding a language never requires touching this file.
 *
 * Layout: shared helpers sit next to this file (`i18n`, `dom`, `dto`,
 * `preferences`, `elevation`, `platform`, `drop`); every page has a module of
 * its own under `views/`. This file only maps navigation entries to views and
 * starts the window.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { showBusy, showDone, showNotice } from "./dom";
import { setFileDrop } from "./drop";
import { PROGRESS_EVENT, type AboutInfo, type ProgressPayload } from "./dto";
import {
  driverRights,
  offerForeignDriverCleanup,
  refreshElevation,
  setupElevationBadge,
} from "./elevation";
import {
  LOCALE_STORAGE_KEY,
  applyLocale,
  availableLocales,
  detectLocale,
  loadLocales,
  localeLabel,
  translateTree,
  type Locale,
} from "./i18n";
import { detectHostPlatform } from "./platform";
import { applyAnalysisNav, applyTheme, setupThemeSwitch, storedTheme } from "./preferences";
import { announceUpdate, checkForUpdateNow } from "./update";
import { viewAnalysis } from "./views/analysis";
import { viewCameraInfo } from "./views/camera";
import { viewFirmware } from "./views/firmware";
import { viewInstall } from "./views/install";
import { viewLicense } from "./views/license";
import { viewNetwork } from "./views/network";
import { viewSettings } from "./views/settings";
import { viewService, viewUpdater } from "./views/shells";
import { viewTools } from "./views/tools";
import { viewTweaks } from "./views/tweaks";
import { viewWelcome } from "./views/welcome";

const VIEWS: Record<string, (host: HTMLElement) => void> = {
  welcome: viewWelcome,
  info: viewCameraInfo,
  install: viewInstall,
  tweaks: viewTweaks,
  analysis: viewAnalysis,
  network: viewNetwork,
  firmware: viewFirmware,
  service: viewService,
  updater: viewUpdater,
  tools: viewTools,
  settings: viewSettings,
  license: viewLicense,
};

function renderView(name: string, host: HTMLElement): void {
  host.replaceChildren();
  // The outgoing view's nodes are gone; drop its claim on dropped files too.
  setFileDrop(null);
  // The service view's controls go with it, and a camera left in service mode
  // stalls the next session's authentication — so navigating away ends the
  // session rather than stranding it behind a page the user has left.
  if (name !== "service") void invoke("service_close").catch(() => undefined);
  // The same for the updater session, which additionally holds a firmware
  // write open: leaving it running behind a hidden page would keep the camera
  // waiting for a chunk nothing is going to send.
  if (name !== "updater") void invoke("updater_close").catch(() => undefined);
  // The tweak view keeps its session between listing and applying so the
  // second operation does not pay for the mode switch again. That saving is
  // only worth having while the view is on screen; once it is not, the camera
  // has no business staying in service mode.
  if (name !== "tweaks") void invoke("tweaks_close").catch(() => undefined);
  (VIEWS[name] ?? viewWelcome)(host);
  translateTree(host);
  // The scrolling container is the content pane around the view, and it keeps
  // its offset across views; a new page has to open at its top.
  host.closest<HTMLElement>(".content")?.scrollTo({ top: 0 });
}

function setupLanguagePicker(current: Locale): void {
  const select = document.querySelector<HTMLSelectElement>("#lang-select")!;
  for (const l of availableLocales) {
    const opt = document.createElement("option");
    opt.value = l;
    opt.textContent = localeLabel(l);
    if (l === current) opt.selected = true;
    select.append(opt);
  }
  select.addEventListener("change", () => {
    void applyLocale(select.value as Locale);
    try {
      localStorage.setItem(LOCALE_STORAGE_KEY, select.value);
    } catch {
      /* the choice still applies to this session */
    }
  });
}

function setupNavigation(): void {
  const items = document.querySelectorAll<HTMLButtonElement>(".nav-item");
  const view = document.querySelector<HTMLElement>("#view")!;
  items.forEach((item) => {
    item.addEventListener("click", () => {
      items.forEach((i) => i.classList.remove("active"));
      item.classList.add("active");
      renderView(item.dataset.view ?? "welcome", view);
    });
  });
  renderView("welcome", view);
}

/** Mirror backend progress events into the currently visible status line. */
async function setupProgress(): Promise<void> {
  await listen<ProgressPayload>(PROGRESS_EVENT, (event) => {
    // A task may just have started the driver helper.
    if (driverRights === "standard") void refreshElevation();
    // Camera work runs in the background, so the user may have moved to
    // another page meanwhile; only a line that is waiting on work takes events.
    const status =
      document.querySelector<HTMLElement>("#view .status[data-progress-target]") ??
      document.querySelector<HTMLElement>('#view .status[data-state="busy"]');
    if (!status) return;
    const { kind, message, percent } = event.payload;
    // `done` is the backend's own closing event. Honouring it stops the
    // spinner the moment the camera is finished rather than when the command
    // call finally returns, which can be a noticeable moment later.
    if (kind === "done") showDone(status, message);
    else if (kind === "notice") showNotice(status, message);
    else showBusy(status, message, percent);
  });
}

/**
 * Show the running build's version below the navigation.
 *
 * Asked of the backend rather than written into the page, so an updated
 * program shows its new number without anyone remembering to change it here;
 * the backend's build refuses to proceed when the version files disagree.
 */
function showAppVersion(): void {
  const target = document.querySelector<HTMLElement>("#app-version")!;
  invoke<AboutInfo>("about_info")
    .then((about) => {
      target.textContent = about.version;
    })
    .catch(() => target.parentElement?.remove());
}

async function main(): Promise<void> {
  // The theme is applied before anything is awaited: a later call would let the
  // default light shell paint first and then flip, which reads as a flicker.
  const theme = storedTheme();
  applyTheme(theme);
  // Plugin languages have to be known before the saved choice is honoured.
  await loadLocales();
  const locale = detectLocale();
  setupLanguagePicker(locale);
  setupThemeSwitch();
  await applyLocale(locale);
  // Before any view is built: the driver paragraphs are chosen at render time,
  // and a page rendered before the answer arrives would show the wrong one.
  await detectHostPlatform();
  await setupElevationBadge();
  applyAnalysisNav();
  setupNavigation();
  showAppVersion();
  const versionCheck = document.querySelector<HTMLButtonElement>("#version-check");
  versionCheck?.addEventListener("click", () => void checkForUpdateNow(versionCheck));
  void announceUpdate();
  await setupProgress();
  // Last, so the window is complete behind the question.
  void offerForeignDriverCleanup();
}

void main();
