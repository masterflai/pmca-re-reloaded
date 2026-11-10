/**
 * Notices about a newer release: at start when the user switched that on, and
 * on demand from the version number below the navigation.
 */

import { invoke } from "@tauri-apps/api/core";
import { fieldList, make, showBusy, showDone, showError, showStatus, statusLine } from "./dom";
import type { AvailableUpdate } from "./dto";
import { tr, trFill } from "./i18n";
import { setUpdateCheck, updateCheckOn } from "./preferences";

const TOGGLE_NAME = "update-check";

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
