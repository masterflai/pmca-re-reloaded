/**
 * Notices about a newer release: at start when the user switched that on, and
 * on demand from the version number below the navigation.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { fieldList, make, showBusy, showDone, showError, showStatus, statusLine } from "./dom";
import type { AvailableUpdate } from "./dto";
import { tr, trFill } from "./i18n";
import { setUpdateCheck, updateCheckOn } from "./preferences";

const TOGGLE_NAME = "update-check";

/** Download progress of an in-app update, emitted by the backend. */
const UPDATE_PROGRESS_EVENT = "pmca://update-progress";

interface UpdateProgress {
  downloaded: number;
  total: number | null;
}

export async function announceUpdate(): Promise<void> {
  if (!updateCheckOn) return;
  let update: AvailableUpdate | null;
  try {
    update = await invoke<AvailableUpdate | null>("check_for_update");
  } catch (e) {
    // Offline or rate-limited: not worth interrupting the start for.
    console.warn("update check failed", e);
    return;
  }
  if (update) showUpdate(update);
}

/** Check now, whatever the start-up setting says, and always report how it ended. */
export async function checkForUpdateNow(trigger: HTMLButtonElement): Promise<void> {
  trigger.disabled = true;
  const status = statusLine();
  showBanner("update-check-heading", status);
  focusBanner();
  showBusy(status, tr("update-checking"));
  try {
    const update = await invoke<AvailableUpdate | null>("check_for_update");
    if (update) {
      showUpdate(update);
      focusBanner();
    } else {
      const version = document.querySelector("#app-version")?.textContent ?? "";
      showDone(status, trFill("update-none", { version }));
      status.after(startupChoice());
    }
  } catch (e) {
    showError(status, e);
  } finally {
    trigger.disabled = false;
  }
}

function showUpdate(update: AvailableUpdate): void {
  const url = update.url;
  const status = statusLine();
  const copy = make("button", { i18n: "btn-copy-url" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(url);
      showDone(status, tr("msg-url-copied"));
    } catch (e) {
      showError(status, e);
    }
  });
  // Text plus a copy button rather than a link: no opener plugin is bundled.
  showBanner(
    "update-available",
    fieldList([
      ["update-current", update.current],
      ["update-latest", update.latest],
    ]),
    make("div", { className: "row" }, make("code", { textContent: url }), copy),
    status,
    startupChoice(),
  );
  // Builds that can replace themselves gain a download-and-install button; the
  // copy-URL notice above still serves the portable archive and the .deb.
  void addInstallSection();
}

/** The reason the update is held, translated, or `null` when it is free to go. */
async function blockedReason(): Promise<string | null> {
  const blockers = await invoke<string[]>("update_blockers").catch(() => []);
  if (blockers.length === 0) return null;
  return `${tr("update-blocked-intro")} ${blockers.map((code) => tr(code)).join("; ")}`;
}

/**
 * Add the in-app updater to the open notice, on a build that can replace itself.
 *
 * The backend verified the download against the key built into the application
 * before handing it over, so the install step runs a signed artifact. The
 * blockers are checked before the download, so a finished download is never
 * left unable to install; `update_install` re-checks them and then restarts
 * into the new version, which is why its call is not expected to return.
 */
async function addInstallSection(): Promise<void> {
  let supported: boolean;
  try {
    supported = await invoke<boolean>("update_supported");
  } catch {
    return;
  }
  if (!supported) return;
  const banner = document.querySelector<HTMLElement>(".update-banner");
  if (!banner) return;

  const status = statusLine();
  const install = make("button", { className: "primary", i18n: "btn-update-install" });
  const restart = make("button", { className: "primary", i18n: "btn-update-restart", hidden: true });

  install.addEventListener("click", async () => {
    const blocked = await blockedReason();
    if (blocked) {
      showStatus(status, blocked);
      return;
    }
    install.disabled = true;
    const unlisten = await listen<UpdateProgress>(UPDATE_PROGRESS_EVENT, (event) => {
      const { downloaded, total } = event.payload;
      const percent = total ? Math.round((downloaded / total) * 100) : null;
      showBusy(status, tr("update-downloading"), percent);
    });
    try {
      await invoke("update_download");
      showDone(status, tr("update-downloaded"));
      install.hidden = true;
      restart.hidden = false;
      restart.focus();
    } catch (e) {
      showError(status, e);
      install.disabled = false;
    } finally {
      unlisten();
    }
  });

  restart.addEventListener("click", async () => {
    const blocked = await blockedReason();
    if (blocked) {
      showStatus(status, blocked);
      return;
    }
    restart.disabled = true;
    try {
      await invoke("update_install");
    } catch (e) {
      showError(status, e);
      restart.disabled = false;
    }
  });

  const section = make(
    "div",
    { className: "update-install" },
    make("div", { className: "row" }, install, restart),
    status,
  );
  // Above the copy-URL row, which is the first `.row` in the banner.
  banner.querySelector(".row")?.before(section);
}

/** Every toggle for the start-up check on screen shows the stored choice. */
export function syncUpdateToggles(): void {
  for (const box of document.querySelectorAll<HTMLInputElement>(`input[name="${TOGGLE_NAME}"]`)) {
    box.checked = updateCheckOn;
  }
}

/** Offer the start-up check right where the user has just seen what it does. */
function startupChoice(): HTMLElement {
  const status = statusLine();
  const toggle = make("input", { type: "checkbox", name: TOGGLE_NAME, checked: updateCheckOn });
  toggle.setAttribute("aria-describedby", "update-optin-note");
  toggle.addEventListener("change", () => {
    setUpdateCheck(toggle.checked);
    syncUpdateToggles();
    const key = updateCheckOn ? "settings-update-on" : "settings-update-off";
    showStatus(status, tr(key));
    status.dataset.i18n = key;
  });

  const settings = make("button", { type: "button", className: "link-button", i18n: "btn-open-settings" });
  settings.addEventListener("click", () => {
    document.querySelector<HTMLElement>('.nav-item[data-view="settings"]')?.click();
  });

  return make(
    "div",
    { className: "update-optin" },
    make("label", { className: "row" }, toggle, make("span", { i18n: "settings-update-toggle" })),
    make(
      "p",
      { className: "hint", id: "update-optin-note" },
      make("span", { i18n: "update-optin-note" }),
      " ",
      settings,
    ),
    status,
  );
}

function focusBanner(): void {
  document.querySelector<HTMLElement>(".update-banner summary")?.focus();
}

/** Put a foldable notice above the view, replacing an earlier one. */
function showBanner(headingKey: string, ...content: Node[]): void {
  const view = document.querySelector<HTMLElement>("#view");
  if (!view) return;
  document.querySelector(".update-banner")?.remove();
  const hide = make("button", { i18n: "btn-update-dismiss" });
  const banner = make(
    "details",
    { className: "card update-banner", open: true },
    make("summary", {}, make("h2", { i18n: headingKey })),
    ...content,
    make("div", { className: "row" }, hide),
  );
  banner.setAttribute("role", "status");
  hide.addEventListener("click", () => banner.remove());
  view.before(banner);
}
