/** Administrator rights and the driver helper badge. */

import { invoke } from "@tauri-apps/api/core";
import type { Rights as DriverRights } from "./bindings/Rights";
import { confirmed, make } from "./dom";
import { errorText, tr } from "./i18n";
import { hostPlatform } from "./platform";

// ---------------------------------------------------------------------------
// Administrator rights
// ---------------------------------------------------------------------------

/** Which rights driver tasks run with, as the backend reports them. */
export type { DriverRights };

/** The current rights; `null` where the question does not arise. */
export let driverRights: DriverRights | null = null;

const RIGHTS_KEYS: Record<DriverRights, string> = {
  elevated: "elevation-admin",
  helper: "elevation-helper",
  standard: "elevation-standard",
};

/** Shield outline on a 16x16 grid, plus a tick once the rights are there. */
const SHIELD_ICON = '<path d="M8 1.5 2.5 3.5v4c0 3.3 2.3 5.9 5.5 7 3.2-1.1 5.5-3.7 5.5-7v-4z"/>';
const SHIELD_TICK = '<path d="m5.5 8 1.8 1.8L10.8 6.3"/>';

/**
 * Show in the header whether driver tasks have administrator rights: the
 * window itself elevated, the elevated helper running, or neither yet.
 *
 * Windows only: elsewhere no driver is ever assigned, so the rights make no
 * difference and a badge would claim one that is not there.
 */
export async function setupElevationBadge(): Promise<void> {
  const badge = document.querySelector<HTMLButtonElement>("#elevation-badge");
  if (!badge || hostPlatform !== "windows") return;
  badge.addEventListener("click", async () => {
    if (driverRights !== "standard") return;
    if (!(await confirmed(tr("elevation-helper-confirm"), tr("btn-start-driver-helper")))) return;
    try {
      await invoke("start_driver_helper");
    } catch (e) {
      badge.title = errorText(e);
    }
    await refreshElevation();
  });
  await refreshElevation();
}

/** Ask the backend which rights driver tasks have now, and show it. */
export async function refreshElevation(): Promise<void> {
  const badge = document.querySelector<HTMLButtonElement>("#elevation-badge");
  if (!badge || hostPlatform !== "windows") return;
  try {
    driverRights = await invoke<DriverRights>("driver_rights");
  } catch {
    return;
  }
  const granted = driverRights !== "standard";
  badge.innerHTML = `<svg viewBox="0 0 16 16" aria-hidden="true">${SHIELD_ICON}${
    granted ? SHIELD_TICK : ""
  }</svg>`;
  badge.append(make("span", { i18n: RIGHTS_KEYS[driverRights] }));
  badge.classList.toggle("admin", granted);
  badge.hidden = false;
  translateElevation();
}

/** The tooltip and the title-bar suffix, in the current language. */
export function translateElevation(): void {
  if (driverRights === null) return;
  const badge = document.querySelector<HTMLElement>("#elevation-badge");
  if (badge) badge.title = tr(`${RIGHTS_KEYS[driverRights]}-hint`);
  const title =
    driverRights === "elevated" ? `${tr("app-title")} (${tr("elevation-admin")})` : tr("app-title");
  void invoke("set_window_title", { title }).catch(() => undefined);
}
