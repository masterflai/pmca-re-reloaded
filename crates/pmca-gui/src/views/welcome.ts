/** Welcome page. */

import { invoke } from "@tauri-apps/api/core";
import { card, make, showBusy, showDone, showError, showStatus, statusLine } from "../dom";
import { keyCard, keyStatus } from "../keys";
import { tr } from "../i18n";
import { hostPlatform } from "../platform";
import { analysisOn } from "../preferences";

// ---------------------------------------------------------------------------
// Views
// ---------------------------------------------------------------------------

/**
 * The page the window opens on: what the toolkit is for, and where things are.
 *
 * Deliberately makes no backend call. This is the first thing a new user sees,
 * and it has to render before a camera is plugged in; probing for one here
 * would greet them with a failure that means nothing is wrong.
 */
export function viewWelcome(host: HTMLElement): void {
  const intro = card(
    make("h2", { i18n: "welcome-intro-heading" }),
    make("p", { i18n: "welcome-intro-body" }),
  );
  host.append(
    intro,
    card(
      make("h2", { i18n: "welcome-start-heading" }),
      make("p", { i18n: "welcome-start-body" }),
      make("p", { className: "hint", i18n: "usb-mode-advice" }),
      make("p", { className: "hint", i18n: welcomeDriverKey() }),
    ),
    card(
      make("h2", { i18n: "welcome-privacy-heading" }),
      make("p", { i18n: "about-privacy-body-1" }),
      make("p", { i18n: "about-privacy-body-2" }),
      make("p", { i18n: "about-privacy-body-3" }),
      make("p", { className: "hint", i18n: "welcome-privacy-where" }),
    ),
    card(
      make("h2", { i18n: "welcome-where-heading" }),
      welcomeMap(),
      make("p", { className: "hint", i18n: "welcome-where-hint" }),
    ),
  );

  // The key material is the user's to supply; show the import card right after
  // the introduction only while something is still missing. A local call, no
  // camera involved.
  void keyStatus()
    .then((status) => {
      if (status.complete) return;
      const section = keyCard((updated) => {
        if (updated.complete) section.remove();
      });
      intro.after(section);
    })
    .catch(() => undefined);
}

/**
 * Which of the three driver paragraphs applies to the machine this runs on.
 *
 * The question the reader has is the same everywhere — "will something ask me
 * for rights, and why?" — but the true answer differs by platform, and only on
 * Windows does this program ask for anything itself. Naming all three would
 * make the reader decide which one is about them, which is the one thing the
 * first page should never require.
 *
 * Deliberately not `driverAdvice`: that one falls back to the service-mode
 * notes, which describe a shell this reader has not opened yet.
 */
function welcomeDriverKey(): string {
  if (hostPlatform === "windows") return "welcome-start-driver";
  return hostPlatform === "macos" ? "welcome-start-driver-macos" : "welcome-start-driver-linux";
}

/**
 * The navigation rail restated, group by group, each page with what it is for.
 *
 * Read from the rail itself, so order, groups and the pages on offer (the
 * function analysis only once switched on) always match it. Each page needs
 * a `welcome-where-<view>` message; a test in pmca-i18n checks that.
 */
function welcomeMap(): HTMLElement {
  const map = make("div");
  let list: HTMLElement | null = null;
  let group: string | undefined;
  for (const entry of document.querySelectorAll<HTMLElement>(".nav-rail .nav-section, .nav-rail .nav-item")) {
    const label = entry.querySelector<HTMLElement>(".nav-label")?.dataset.i18n;
    if (!label) continue;
    if (entry.classList.contains("nav-section")) {
      map.append(make("h3", { className: "welcome-map-group", i18n: label }));
      list = null;
      group = entry.dataset.group;
      continue;
    }
    const view = entry.dataset.view;
    // `hidden` also marks pages of a folded group, so the lock is asked directly.
    if (!view || view === "welcome" || (view === "analysis" && !analysisOn)) continue;
    if (!list || entry.dataset.group !== group) {
      group = entry.dataset.group;
      list = make("dl", { className: group ? "fields welcome-map" : "fields welcome-map welcome-map-rest" });
      map.append(list);
    }
    list.append(make("dt", { i18n: label }), make("dd", { i18n: `welcome-where-${view}` }));
  }
  return map;
}

/** Where the project documents which cameras are supported. */
export const DOCS_DEVICES_URL = "https://openmemories.readthedocs.io/devices.html";

/** A running Windows service that can keep the camera from being reached. */
interface InterferingService {
  name: string;
  displayName: string;
}

/**
 * Checks for Windows services known to hold on to a camera.
 *
 * Only reads the service manager. Stopping a system service is left to the
 * user, so the card names the service and says how, nothing more.
 */
export function cardServiceDiagnosis(): HTMLElement {
  const status = statusLine();
  const result = make("div");
  const button = make("button", { i18n: "btn-diagnose-services" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    result.replaceChildren();
    showBusy(status, tr("msg-working"));
    try {
      const found = await invoke<InterferingService[]>("check_interfering_services");
      if (found.length === 0) {
        showDone(status, tr("diagnose-services-none"));
        return;
      }
      showStatus(status, "");
      result.append(
        make("p", { className: "warning", i18n: "diagnose-services-found" }),
        make(
          "ul",
          {},
          ...found.map((s) => make("li", { textContent: `${s.displayName} (${s.name})` })),
        ),
        make("p", { i18n: "diagnose-services-advice" }),
        ...found.map((s) =>
          make("p", {}, make("code", { textContent: `net stop ${s.name}` })),
        ),
      );
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });
  return card(
    make("h2", { i18n: "diagnose-services-heading" }),
    make("p", { i18n: "diagnose-services-hint" }),
    button,
    status,
    result,
  );
}
