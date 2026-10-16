/** Small DOM helpers shared by all views: element factory, status lines, dialogs, error display. */

import { invoke } from "@tauri-apps/api/core";
import type { Access } from "./bindings/Access";
import { refreshElevation } from "./elevation";
import { errorText, isCommandError, tr } from "./i18n";
import { hostPlatform } from "./platform";

// ---------------------------------------------------------------------------
// Small DOM helpers
// ---------------------------------------------------------------------------

export function make<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  props: Partial<HTMLElementTagNameMap[K]> & { i18n?: string } = {},
  ...children: (Node | string)[]
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  const { i18n, ...rest } = props;
  Object.assign(node, rest);
  if (i18n) {
    node.dataset.i18n = i18n;
    node.textContent = tr(i18n);
  }
  node.append(...children);
  return node;
}

export function card(...children: (Node | string)[]): HTMLElement {
  return make("div", { className: "card" }, ...children);
}

/** A badge saying what a step does to the camera: read, write, user, local. */
export function accessBadge(access: Access): HTMLElement {
  return make("span", {
    className: `access-badge access-${access}`,
    i18n: `analysis-access-${access}`,
  });
}

/**
 * A button that changes the camera, preceded by the write badge.
 *
 * The badge sits beside the button rather than inside it, because translating
 * the button replaces its whole text. It disappears with a hidden button.
 */
export function writing(button: HTMLElement): HTMLElement {
  return make("span", { className: "write-action" }, accessBadge("write"), button);
}

/**
 * The driver advice that applies to the machine this window runs on.
 *
 * The three platforms do not differ in tone, they differ in fact. Windows hands
 * the camera to a kernel driver that raw USB cannot work through, so one has to
 * be displaced and put back. Linux has no such driver in the way; what it has
 * is a permissions question and a desktop that grabs the device. macOS needs
 * neither, except for Sony's own driver in mass-storage mode.
 *
 * Printing all three and letting the reader pick would be the same as printing
 * none: the paragraph that matters is the one you cannot tell apart from the
 * two that do not.
 */
export function driverAdvice(windowsHeadingKey: string, windowsBodyKey: string): Node[] {
  if (hostPlatform === "windows") {
    return [make("h3", { i18n: windowsHeadingKey }), make("p", { i18n: windowsBodyKey })];
  }
  return [
    make("h3", { i18n: "service-driver-heading" }),
    make("p", { i18n: hostPlatform === "macos" ? "driver-note-macos" : "driver-note-linux" }),
  ];
}

/** A value shown as is, or `{ key }` for one that is itself translated. */
type FieldValue = string | { key: string } | null | undefined;

/** Render a `label: value` grid from the given pairs, skipping empty values. */
export function fieldList(pairs: [string, FieldValue][]): HTMLElement {
  const dl = make("dl", { className: "fields" });
  for (const [key, value] of pairs) {
    if (value === null || value === undefined || value === "") continue;
    const dd =
      typeof value === "string"
        ? make("dd", { textContent: value })
        : make("dd", { i18n: value.key });
    dl.append(make("dt", { i18n: key }), dd);
  }
  return dl;
}

/**
 * Asks for consent before a risky step, in a warning dialog.
 *
 * The confirming button names the step (`ok`), the other one cancels: a bare
 * "Yes"/"No" leaves open what is being agreed to when the message is a warning
 * rather than a question. Never use `window.confirm` — the dialog plugin
 * overrides it with a call to a command it no longer has.
 */
export async function confirmed(message: string, ok: string): Promise<boolean> {
  return chosen(message, ok, tr("btn-cancel"), true);
}

/**
 * Offers a choice between two named actions in a native dialog.
 *
 * Only the `ok` action returns `true`; a failure counts as `cancel`. With
 * `warning` set the dialog carries the warning icon.
 */
export async function chosen(
  message: string,
  ok: string,
  cancel: string,
  warning = false,
): Promise<boolean> {
  try {
    return (await invoke<boolean>("ask_choice", { message, ok, cancel, warning })) === true;
  } catch {
    return false;
  }
}

/** Shows a message in a native dialog with a single button labelled `ok`. */
export async function told(message: string, ok: string, warning = false): Promise<void> {
  try {
    await invoke("show_message", { message, ok, warning });
  } catch {
    // A dialog that cannot open has nobody to tell.
  }
}

/** A status line used for progress, results and errors. */
export function statusLine(): HTMLElement {
  return make("p", { className: "status" });
}

/** "Connecting…" plus the note that prompts on the camera's screen do no harm. */
export function connectingText(): string {
  return `${tr("msg-connecting")} ${tr("camera-confirm-hint")}`;
}

/**
 * Drop whatever made the current text translatable, along with any state.
 *
 * A status line is reused for progress, results and failures, so the marker
 * left behind by the previous message has to go before the next one is
 * rendered — otherwise a language change would resurrect it, or a finished
 * operation would keep the spinner of the one before it.
 */
export function clearStatusState(target: HTMLElement): void {
  delete target.dataset.i18n;
  delete target.dataset.errCode;
  delete target.dataset.errMessage;
  delete target.dataset.errDetail;
  delete target.dataset.state;
  delete target.dataset.progress;
  target.style.removeProperty("--progress");
  target.removeAttribute("title");
  const recovery = target.nextElementSibling;
  if (recovery?.classList.contains("recovery")) recovery.remove();
}

/** The only camera mode change that can be triggered over USB. */
const RECOVERY_SWITCH_TO_MASS_STORAGE = "switch-to-mass-storage";

/** Offered after the rights prompt for the driver helper was dismissed. */
const RECOVERY_START_DRIVER_HELPER = "start-driver-helper";

/** Offered when the camera refused a single value but may take the whole image. */
export const RECOVERY_WHOLE_IMAGE = "whole-image";

/** Offered when an operation needs key material the user has not imported yet. */
const RECOVERY_IMPORT_KEYS = "import-keys";

/** One tweak change as the write commands take it. */
export interface TweakWrite {
  id: string;
  enabled: boolean;
  acceptCaution: boolean;
}

/**
 * Offer an action below a failure the user can actually resolve.
 *
 * Two exist. An MTP camera can be told to reconnect as a mass-storage device,
 * which is the only mode transition that exists as a USB command — the way
 * back is a power cycle, so the accompanying text explains that rather than
 * offering a second button. And a dismissed rights prompt can be answered by
 * starting the driver helper again, after which the operation is retried.
 * Every other failure needs the user to act on the camera, so the backend
 * offers no action and nothing is rendered here.
 */
function renderRecovery(target: HTMLElement, error: unknown, retry?: () => void): void {
  if (!isCommandError(error)) return;

  if (error.recovery === RECOVERY_IMPORT_KEYS) {
    const button = make("button", { i18n: "btn-import-keys" });
    // The import lives on the welcome page; send the user there.
    button.addEventListener("click", () => {
      document.querySelector<HTMLElement>('[data-view="welcome"]')?.click();
    });
    target.after(
      make(
        "div",
        { className: "recovery" },
        button,
        make("p", { className: "hint", i18n: "keys-not-set-up" }),
      ),
    );
    return;
  }

  if (error.recovery === RECOVERY_START_DRIVER_HELPER) {
    const button = make("button", { i18n: "btn-start-driver-helper" });
    button.addEventListener("click", async () => {
      button.disabled = true;
      try {
        await invoke("start_driver_helper");
        await refreshElevation();
        button.closest(".recovery")?.remove();
        retry?.();
      } catch (e) {
        showError(target, e, retry);
      }
    });
    target.after(make("div", { className: "recovery" }, button));
    return;
  }

  if (error.recovery !== RECOVERY_SWITCH_TO_MASS_STORAGE) return;

  const button = make("button", { i18n: "btn-switch-msc" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    // Clears this very box, which is intended: the switch replaces the failure
    // it was offered for, and its outcome decides what is shown next.
    showBusy(target, tr("mode-switching"));
    try {
      await invoke("switch_camera_mode");
      showDone(target, tr("mode-switch-done"));
      retry?.();
    } catch (e) {
      showError(target, e, retry);
    }
  });

  target.after(
    make(
      "div",
      { className: "recovery" },
      button,
      make("p", { className: "hint", i18n: "mode-switch-hint" }),
    ),
  );
}

export function showError(target: HTMLElement, error: unknown, retry?: () => void): void {
  clearStatusState(target);
  target.dataset.state = "error";
  target.textContent = errorText(error);
  // Remember the failure so a later language change can re-render it.
  if (isCommandError(error)) {
    target.dataset.errCode = error.code;
    target.dataset.errMessage = error.message;
    if (error.detail) target.dataset.errDetail = error.detail;
  } else {
    target.dataset.errMessage = typeof error === "string" ? error : String(error);
  }
  renderRecovery(target, error, retry);
}

/** Show a front-end validation failure, addressed by its catalogue key. */
export function showErrorKey(target: HTMLElement, key: string): void {
  clearStatusState(target);
  target.dataset.state = "error";
  target.dataset.i18n = key;
  target.textContent = tr(key);
}

export function showStatus(target: HTMLElement, message: string): void {
  clearStatusState(target);
  target.textContent = message;
}

/**
 * Show something the user has to act on, outliving the events that follow.
 *
 * The status line holds one message at a time, and the backend emits these as
 * an operation winds down — a millisecond before its closing event, which would
 * overwrite them. They therefore get a block of their own, which stays until
 * the next operation begins.
 */
export function showNotice(target: HTMLElement, message: string): void {
  const existing = target.nextElementSibling;
  if (existing?.classList.contains("status-notice")) {
    existing.textContent = message;
    return;
  }
  target.after(make("div", { className: "notice-block status-notice", textContent: message }));
}

/** Drop the notice left behind by a previous operation, if any. */
function clearNotice(target: HTMLElement): void {
  const notice = target.nextElementSibling;
  if (notice?.classList.contains("status-notice")) notice.remove();
}

/**
 * Report that a long-running operation is under way.
 *
 * `percent` fills the progress bar instead of sweeping it. The backend reports
 * a percentage only for transfers whose total size it knows, so an absent value
 * means "still working", not "nothing done yet" — rendering the two the same
 * way would make a slow start look like a stall.
 */
export function showBusy(target: HTMLElement, message: string, percent: number | null = null): void {
  // A new operation always starts from a finished or untouched status line,
  // whereas its own later events arrive while the line is already busy. That
  // tells the two apart, so a notice survives the operation that raised it
  // without lingering into the next one.
  if (target.dataset.state !== "busy") clearNotice(target);
  clearStatusState(target);
  target.dataset.state = "busy";
  // The bar is only four pixels tall, so the exact figure stays in the text.
  target.textContent = percent === null ? message : `${message} ${percent}%`;
  if (percent !== null) {
    target.dataset.progress = "";
    target.style.setProperty("--progress", `${Math.min(100, Math.max(0, percent))}%`);
  }
}

/** Report that an operation finished successfully. */
export function showDone(target: HTMLElement, message: string): void {
  clearStatusState(target);
  target.dataset.state = "ok";
  target.textContent = message;
}

/** Show a fixed state, addressed by its catalogue key, e.g. in a table row. */
export function showStateKey(target: HTMLElement, state: string, key: string): void {
  clearStatusState(target);
  target.dataset.state = state;
  target.dataset.i18n = key;
  target.textContent = tr(key);
}

/**
 * Run `work` with backend progress routed to `target`.
 *
 * Progress events go to the first status line of the view unless another one
 * claims them; a view with one line per running item has to say which.
 */
export async function withProgressTarget<T>(target: HTMLElement, work: () => Promise<T>): Promise<T> {
  target.dataset.progressTarget = "";
  try {
    return await work();
  } finally {
    delete target.dataset.progressTarget;
  }
}
