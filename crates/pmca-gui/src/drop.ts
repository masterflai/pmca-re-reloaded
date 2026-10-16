/** Dropping a file onto the window. */

import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { make, showError, showErrorKey } from "./dom";

// ---------------------------------------------------------------------------
// Dropping a file onto the window
// ---------------------------------------------------------------------------

/**
 * Where a dropped file should go, set by whichever view can accept one.
 *
 * Views are rebuilt from scratch on every navigation, so this is cleared in
 * `renderView` to keep a stale closure from capturing detached DOM nodes. The
 * suffixes travel with the handler because each view accepts a different kind
 * of file — an `.apk` to install, a `.dat` firmware image to flash, a photo.
 */
let fileDrop: { suffixes: string[]; accept: (path: string) => void } | null = null;

/** Claim dropped files for the current view, or release them with `null`. */
export function setFileDrop(target: typeof fileDrop): void {
  fileDrop = target;
}

/** Guards against registering the webview listener once per view rebuild. */
let dragDropRegistered = false;

/**
 * Forward files dropped onto the window to the active view.
 *
 * Tauri reports real filesystem paths for dropped files, which is why the app
 * can use a dropped file without bundling a file-picker plugin and without
 * pushing megabytes through the IPC channel.
 */
export async function registerDragDrop(zone: HTMLElement): Promise<void> {
  zone.classList.remove("active");
  if (dragDropRegistered) return;
  dragDropRegistered = true;
  await getCurrentWebview().onDragDropEvent((event) => {
    const active = document.querySelector<HTMLElement>(".dropzone");
    if (event.payload.type === "over") {
      active?.classList.add("active");
      return;
    }
    active?.classList.remove("active");
    if (event.payload.type !== "drop") return;
    const target = fileDrop;
    if (!target) return;
    const match = event.payload.paths.find((p) =>
      target.suffixes.some((suffix) => p.toLowerCase().endsWith(suffix)),
    );
    if (match) target.accept(match);
  });
}

/**
 * A "Browse…" button that fills a path field from a native file dialog.
 *
 * Dropping a file only works for a pointer user who already has the file
 * visible somewhere, and typing a path exactly is easy to get wrong. The dialog
 * is the way the platform expects a file to be chosen and is reachable from the
 * keyboard, so it is offered next to every path field rather than instead of
 * the dropzone.
 *
 * The backend returns `null` when the dialog is dismissed. Cancelling is a
 * normal outcome, so the field and the status line are both left untouched.
 */
export function browseButton(
  status: HTMLElement,
  pick: () => Promise<string | null>,
  accept: (path: string) => void,
): HTMLButtonElement {
  const button = make("button", { i18n: "btn-browse" });
  button.addEventListener("click", async () => {
    button.disabled = true;
    try {
      const chosen = await pick();
      if (chosen !== null) accept(chosen);
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });
  return button;
}

/** A file chosen with {@link filePicker}; `path()` is empty while none is. */
interface FilePicker {
  element: HTMLElement;
  path: () => string;
}

/**
 * One box to choose a file with: drop it onto the window, or pick it with the
 * dialog.
 *
 * This replaces a text field, a separate drop target and a "Browse…" button,
 * which were three controls for one decision. Typing a path is gone on
 * purpose: it was the one of the three that could name a file that is not
 * there. Whichever way the file arrived, the box then shows its full path —
 * the name alone would not tell two files of the same name apart.
 *
 * "Remove" empties the box again, which matters where no file is a valid
 * answer, such as an install that only serves the camera an empty task.
 */
export function filePicker(
  status: HTMLElement,
  kind: string,
  suffixes: string[],
  hintKey: string,
): FilePicker {
  let chosen = "";
  const zone = make("div", { className: "dropzone file-picker" });
  const hint = make("span", { className: "file-picker-hint", i18n: hintKey });
  const shown = make("span", { className: "file-picker-path" });
  const remove = make("button", { i18n: "btn-remove" });

  const choose = (path: string): void => {
    chosen = path;
    shown.textContent = path;
    shown.title = path;
    hint.hidden = path !== "";
    shown.hidden = path === "";
    remove.hidden = path === "";
    zone.classList.remove("active");
  };
  const browse = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind }),
    choose,
  );
  remove.addEventListener("click", () => choose(""));

  zone.append(hint, shown, make("div", { className: "file-picker-actions" }, browse, remove));
  choose("");
  fileDrop = { suffixes, accept: choose };
  void registerDragDrop(zone);
  return { element: zone, path: () => chosen };
}

/**
 * Append a picked path to a shell command line, quoted.
 *
 * The quotes are what let a path with a space in it arrive as one argument.
 * They also mean the path cannot itself contain a double quote: the parser
 * would read the first one as the end of the argument and act on a shorter
 * path than the user chose, without saying so. Windows forbids `"` in file
 * names, so this can only arise on Linux and macOS — rarely, but silently,
 * which is why it is refused here rather than left to the parser.
 */
export function appendPickedPath(input: HTMLInputElement, status: HTMLElement, path: string): void {
  if (path.includes('"')) {
    showErrorKey(status, "shell-path-quote");
    return;
  }
  input.value = `${input.value}${input.value.endsWith(" ") ? "" : " "}"${path}"`.trimStart();
  input.focus();
}
