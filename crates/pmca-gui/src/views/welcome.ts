/** Welcome page. */

import { invoke } from "@tauri-apps/api/core";
import { card, make, showBusy, showDone, showError, showStatus, statusLine } from "../dom";
import { keyCard, keyStatus } from "../keys";
import { tr } from "../i18n";
import { hostPlatform } from "../platform";

// ---------------------------------------------------------------------------
// Views
// ---------------------------------------------------------------------------

/**
 * The page the window opens on: what the toolkit is for, and where things are.
 *
 * Deliberately makes no backend call. This is the first thing a new user sees,
 * and it has to render before a camera is plugged in; probing for one here
 * would greet them with a failure that means nothing is wrong.
 *
 * The two meanings of "language" get a heading each rather than a clause each.
 * One of them changes this window, the other rewrites the camera's backup
 * memory, and they are the likeliest thing on this page to be confused.
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
      make("h2", { i18n: "welcome-tweaks-heading" }),
      make("p", { i18n: "welcome-tweaks-body" }),
      make("h3", { i18n: "welcome-tweaks-languages-heading" }),
      make("p", { i18n: "welcome-tweaks-languages-body" }),
      make("h3", { i18n: "welcome-tweaks-ui-heading" }),
      make("p", { i18n: "welcome-tweaks-ui-body" }),
      make("div", { className: "notice-block" }, make("p", { i18n: "welcome-tweaks-safety" })),
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
 * The navigation rail restated as a list, each page with what it is for.
 *
 * The page names are the rail's own keys rather than copies, so renaming a
 * page renames it here too and the two can never drift apart.
 */
function welcomeMap(): HTMLElement {
  const rows: [string[], string][] = [
    [["nav-camera-info"], "welcome-where-info"],
    [["docs-compatibility"], "welcome-where-compat"],
    [["nav-install-app"], "welcome-where-install"],
    [["nav-tweaks"], "welcome-where-tweaks"],
    [["nav-network"], "welcome-where-network"],
    [["nav-firmware"], "welcome-where-firmware"],
    [["nav-tools"], "welcome-where-tools"],
    [["nav-service", "nav-updater"], "welcome-where-shells"],
    [["nav-settings"], "welcome-where-settings"],
    [["nav-license"], "welcome-where-license"],
  ];
  const dl = make("dl", { className: "fields welcome-map" });
  for (const [labels, body] of rows) {
    // The two shells share one line, so that term holds two keys. It therefore
    // carries none itself: `translateTree` replaces the text content of any
    // element with a key, which would leave one of the two names standing.
    const dt = make("dt");
    labels.forEach((key, index) => {
      if (index > 0) dt.append(" / ");
      dt.append(make("span", { i18n: key }));
    });
    dl.append(dt, make("dd", { i18n: body }));
  }
  return dl;
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
