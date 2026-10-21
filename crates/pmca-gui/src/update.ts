/** The start-up notice about a newer release, shown only when the user switched it on. */

import { invoke } from "@tauri-apps/api/core";
import { fieldList, make, showDone, showError, statusLine } from "./dom";
import type { AvailableUpdate } from "./dto";
import { tr } from "./i18n";
import { updateCheckOn } from "./preferences";

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
  const view = document.querySelector<HTMLElement>("#view");
  if (!update || !view) return;
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
  const hide = make("button", { i18n: "btn-update-dismiss" });

  // Text plus a copy button rather than a link: no opener plugin is bundled.
  const banner = make(
    "div",
    { className: "card update-banner" },
    make("h2", { i18n: "update-available" }),
    fieldList([
      ["update-current", update.current],
      ["update-latest", update.latest],
    ]),
    make("div", { className: "row" }, make("code", { textContent: url }), copy, hide),
    status,
  );
  banner.setAttribute("role", "status");
  hide.addEventListener("click", () => banner.remove());
  view.before(banner);
}
