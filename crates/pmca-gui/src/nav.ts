/** The navigation rail around the views: pinning, the overlay, Ctrl+B, the page title and the camera status. */

import { invoke } from "@tauri-apps/api/core";
import type { CameraPresence } from "./dto";
import { currentLocale, tr, trFill } from "./i18n";
import { hostPlatform } from "./platform";
import { expandNavGroup } from "./preferences";

// ---------------------------------------------------------------------------
// Pinning and the overlay
// ---------------------------------------------------------------------------

const PINNED_STORAGE_KEY = "pmca.nav.pinned";
/** Below this width the rail unpins itself; the stored choice is left alone. */
const narrowQuery = window.matchMedia("(max-width: 999px)");
/** Long enough that passing the pointer across the rail does not open it. */
const OPEN_DELAY_MS = 200;
const CLOSE_DELAY_MS = 300;

let pinned = readPinned();
/** Pinned by hand in a narrow window; dropped whenever the window crosses the width. */
let pinnedWhileNarrow = false;
let open = false;
/** Opened by the expand button: stays until dismissed instead of following the pointer. */
let sticky = false;
let timer: number | undefined;

function readPinned(): boolean {
  try {
    return localStorage.getItem(PINNED_STORAGE_KEY) !== "off";
  } catch {
    return true;
  }
}

function isPinned(): boolean {
  return narrowQuery.matches ? pinnedWhileNarrow : pinned;
}

function shell(): HTMLElement {
  return document.querySelector<HTMLElement>(".app")!;
}

function togglePinned(): void {
  if (narrowQuery.matches) {
    pinnedWhileNarrow = !pinnedWhileNarrow;
  } else {
    pinned = !pinned;
    try {
      localStorage.setItem(PINNED_STORAGE_KEY, pinned ? "on" : "off");
    } catch {
      /* the choice still applies to this session */
    }
  }
  setOpen(false);
  applyPinned();
}

function applyPinned(): void {
  shell().classList.toggle("nav-unpinned", !isPinned());
  if (isPinned()) setOpen(false);
  document.querySelector("#nav-pin use")?.setAttribute("href", isPinned() ? "#i-pin" : "#i-unpin");
  translatePin();
}

function setOpen(value: boolean, keep = false): void {
  window.clearTimeout(timer);
  open = value && !isPinned();
  sticky = open && keep;
  shell().classList.toggle("nav-open", open);
  document.querySelector("#nav-expand")?.setAttribute("aria-expanded", String(open));
}

function after(ms: number, action: () => void): void {
  window.clearTimeout(timer);
  timer = window.setTimeout(action, ms);
}

function translatePin(): void {
  const button = document.querySelector<HTMLButtonElement>("#nav-pin");
  const expand = document.querySelector<HTMLButtonElement>("#nav-expand");
  if (!button || !expand) return;
  const shortcut = hostPlatform === "macos" ? "⌘B" : "Ctrl+B";
  const label = `${tr(isPinned() ? "nav-unpin" : "nav-pin")} (${shortcut})`;
  button.title = label;
  button.setAttribute("aria-label", label);
  // Pinned, the program icon is only a picture.
  expand.disabled = isPinned();
  expand.title = isPinned() ? "" : tr("nav-expand");
  expand.setAttribute("aria-label", tr("nav-expand"));
}

/** The program name stands on its own line; below it what the translated title adds. */
function translateBrand(): void {
  const sub = document.querySelector<HTMLElement>("#nav-brand-sub");
  if (!sub) return;
  const title = tr("app-title");
  const rest = title.replace(/^PMCA-RE RELOADED\s*[–—-]\s*/, "");
  sub.textContent = rest === title ? "" : `${rest.charAt(0).toLocaleUpperCase(currentLocale)}${rest.slice(1)}`;
}

export function setupNav(): void {
  const rail = document.querySelector<HTMLElement>(".nav-rail")!;
  document.querySelector("#nav-pin")!.addEventListener("click", togglePinned);
  document.querySelector("#nav-expand")!.addEventListener("click", () => {
    if (open && sticky) setOpen(false);
    else setOpen(true, true);
  });

  // Hover is for the mouse only; touch and pen use the expand button.
  rail.addEventListener("pointerenter", (event) => {
    if (event.pointerType !== "mouse" || isPinned()) return;
    if (open) window.clearTimeout(timer);
    else after(OPEN_DELAY_MS, () => setOpen(true));
  });
  rail.addEventListener("pointerleave", (event) => {
    if (event.pointerType !== "mouse" || isPinned()) return;
    if (sticky) window.clearTimeout(timer);
    else after(CLOSE_DELAY_MS, () => setOpen(false));
  });

  // Keyboard focus opens the rail; a mouse click focuses too, but hover has that covered.
  rail.addEventListener("focusin", (event) => {
    if (!open && !isPinned() && (event.target as Element).matches(":focus-visible")) setOpen(true);
  });
  rail.addEventListener("focusout", (event) => {
    const next = event.relatedTarget as Node | null;
    if (open && !sticky && !rail.contains(next) && !rail.matches(":hover")) setOpen(false);
  });

  // A group icon clicked while closed opens the rail on that group rather than folding it.
  rail.addEventListener(
    "click",
    (event) => {
      const section = (event.target as Element).closest<HTMLElement>(".nav-section");
      if (!section || open || isPinned()) return;
      event.stopPropagation();
      setOpen(true, true);
      expandNavGroup(section.dataset.group ?? "");
    },
    true,
  );
  rail.addEventListener("click", (event) => {
    if ((event.target as Element).closest(".nav-item")) setOpen(false);
  });

  document.addEventListener("pointerdown", (event) => {
    if (open && !rail.contains(event.target as Node)) setOpen(false);
  });
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && open) {
      setOpen(false);
      return;
    }
    if (
      (event.ctrlKey || event.metaKey) &&
      !event.altKey &&
      !event.shiftKey &&
      event.key.toLowerCase() === "b"
    ) {
      event.preventDefault();
      togglePinned();
    }
  });
  narrowQuery.addEventListener("change", () => {
    pinnedWhileNarrow = false;
    applyPinned();
  });

  applyPinned();
  void pollCameraPresence();
}

// ---------------------------------------------------------------------------
// Where the user is
// ---------------------------------------------------------------------------

/** Name the open page in the header and mark its group in the rail. */
export function showLocation(view: string): void {
  const item = document.querySelector<HTMLElement>(`.nav-item[data-view="${view}"]`);
  const group = item?.dataset.group;
  let groupKey: string | undefined;
  for (const section of document.querySelectorAll<HTMLElement>(".nav-section")) {
    const mine = section.dataset.group === group;
    section.classList.toggle("has-active", mine);
    if (mine) groupKey = section.querySelector<HTMLElement>(".nav-label")?.dataset.i18n;
  }
  setCrumb("#crumb-group", groupKey);
  setCrumb("#crumb-page", item?.querySelector<HTMLElement>(".nav-label")?.dataset.i18n ?? "app-title");
}

/** Fill one part of the header; the key stays on it so a language change follows. */
function setCrumb(selector: string, key: string | undefined): void {
  const part = document.querySelector<HTMLElement>(selector);
  if (!part) return;
  part.hidden = key === undefined;
  if (key === undefined) {
    delete part.dataset.i18n;
    return;
  }
  part.dataset.i18n = key;
  part.textContent = tr(key);
}

// ---------------------------------------------------------------------------
// Camera status
// ---------------------------------------------------------------------------

/** Often enough to follow a plug or unplug; the check opens no device. */
const PRESENCE_POLL_MS = 3000;

let presence: CameraPresence = "none";
/** The model last read in this session, until the camera leaves the bus. */
let cameraModel: string | null = null;

/** Note a camera that has just answered, so the status can name it. */
export function rememberCameraModel(model: string | null): void {
  cameraModel = model;
  presence = "connected";
  renderCameraStatus();
}

async function pollCameraPresence(): Promise<void> {
  try {
    const next = await invoke<CameraPresence>("camera_presence");
    // Another camera may be plugged in next; only a fresh read may name it.
    if (next === "none") cameraModel = null;
    if (next !== presence) {
      presence = next;
      renderCameraStatus();
    }
  } catch {
    /* keep the last answer */
  }
  window.setTimeout(() => void pollCameraPresence(), PRESENCE_POLL_MS);
}

function renderCameraStatus(): void {
  const box = document.querySelector<HTMLElement>("#camera-status");
  const text = document.querySelector<HTMLElement>("#camera-status-text");
  if (!box || !text) return;
  box.dataset.state = presence;
  text.textContent =
    presence === "connected"
      ? cameraModel
        ? trFill("nav-camera-model", { model: cameraModel })
        : tr("nav-camera-connected")
      : tr(presence === "waiting" ? "nav-camera-waiting" : "nav-camera-none");
  // Closed to icons, the dot is all that shows; the tooltip says what it means.
  box.title = text.textContent;
}

/** Re-render the texts built here rather than from a `data-i18n` key. */
export function translateNav(): void {
  translatePin();
  translateBrand();
  renderCameraStatus();
}
