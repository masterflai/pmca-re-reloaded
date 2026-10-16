/** Service shell and updater shell pages. */

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  card,
  confirmed,
  connectingText,
  driverAdvice,
  make,
  showBusy,
  showDone,
  showError,
  showStatus,
  statusLine,
} from "../dom";
import { appendPickedPath, browseButton } from "../drop";
import { PROGRESS_EVENT, type ProgressPayload, type ShellOutcome } from "../dto";
import { errorText, tr } from "../i18n";
import { advancedMode } from "../preferences";

/**
 * The service page.
 *
 * Two pages, really. Without advanced mode it explains how to reach the shell
 * from the console and what has to be set up first; with it, the shell is here.
 * The gate is deliberate — service mode can leave a camera unusable, and until
 * now the console was the thing standing between a curious click and that.
 */
export function viewService(host: HTMLElement): void {
  host.append(
    card(
      make("p", { className: "warning", i18n: "service-warning" }),
      make("p", { className: "hint", i18n: "usb-mode-advice" }),
    ),
  );
  host.append(advancedMode() ? cardServiceShell() : cardServiceConsole());
  host.append(cardShellPrimer("service"));
}

/**
 * The commands each shell accepts, paired with what they do.
 *
 * The syntax is not translated and never will be: these are words that have to
 * be typed exactly as they appear, and a translated `pull` would not work. Only
 * the explanation beside each one comes from the catalogue.
 *
 * Several entries are shared between the two shells because the command really
 * is the same command. The ones that are not — `bk s` takes a subsystem on the
 * service shell and nothing on the updater shell — carry a key of their own
 * rather than a footnote, because a reference that is almost right is worse
 * than one that is longer.
 */
const SHELL_COMMANDS: Record<string, [syntax: string, key: string][]> = {
  service: [
    ["hasp", "shell-cmd-hasp"],
    ["pull <remote> [local]", "shell-cmd-pull"],
    ["push <local> <remote>", "shell-cmd-push"],
    ["rm <remote>", "shell-cmd-rm"],
    ["rmem <hexoff> <hexlen> <outfile>", "shell-cmd-rmem"],
    ["bk r <hexid>", "shell-cmd-bk-r"],
    ["bk d <outfile>", "shell-cmd-bk-d"],
    ["bk dump-txt <file>", "shell-cmd-bk-dump-txt"],
    ["bk stat", "shell-cmd-bk-stat"],
    ["bk w <hexid> <hex...>", "shell-cmd-bk-w"],
    ["bk s <hexsubsystem>", "shell-cmd-bk-s-sub"],
    ["bk lock | bk unlock", "shell-cmd-bk-lock"],
    ["term on | term off", "shell-cmd-term"],
    ["help", "shell-cmd-help"],
    ["exit", "shell-cmd-exit"],
  ],
  updater: [
    ["info", "shell-cmd-info"],
    ["bk r <hexid>", "shell-cmd-bk-r"],
    ["bk w <hexid> <hex...>", "shell-cmd-bk-w"],
    ["bk s", "shell-cmd-bk-s"],
    ["bk lock | bk unlock", "shell-cmd-bk-lock"],
    ["push <local> <remote>", "shell-cmd-push"],
    ["pull <remote> [local]", "shell-cmd-pull"],
    ["rmem <hexoff> <hexlen> <outfile>", "shell-cmd-rmem"],
    ["shell [tcp <port>]", "shell-cmd-shell"],
    ["help", "shell-cmd-help"],
    ["exit", "shell-cmd-exit"],
  ],
};

/** Service-shell commands for investigating and repairing a camera, listed apart. */
const SERVICE_DIAGNOSTICS: [syntax: string, key: string][] = [
  ["bk trace <outfile>", "shell-cmd-bk-trace"],
  ["bk rewrite", "shell-cmd-bk-rewrite"],
  ["bk restore-attrs <reference>", "shell-cmd-bk-restore-attrs"],
];

/**
 * What the two shells are, when to reach for which, and what each command does.
 *
 * Folded away behind a button rather than printed on the page: someone who has
 * come here to type three commands should not have to scroll past an essay,
 * and someone who has *not* done this before needs considerably more than a
 * tooltip. Both pages carry the same explanation, because the useful thing to
 * know about either shell is largely how it differs from the other one.
 *
 * The box scrolls instead of growing, so opening it never pushes the shell
 * itself off the screen.
 */
function cardShellPrimer(shell: "service" | "updater"): HTMLElement {
  const commandList = (entries: [string, string][]): HTMLElement => {
    const list = make("dl", { className: "primer-commands" });
    for (const [syntax, key] of entries) {
      list.append(
        make("dt", {}, make("code", { textContent: syntax })),
        make("dd", { i18n: key }),
      );
    }
    return list;
  };

  const body = make(
    "div",
    { className: "primer" },
    make("h3", { i18n: "shell-primer-compare-heading" }),
    make("p", { i18n: "shell-primer-compare" }),
    make("p", { i18n: "shell-primer-payload" }),
    make("p", { i18n: "shell-primer-when-service" }),
    make("p", { i18n: "shell-primer-when-updater" }),
    ...driverAdvice("shell-primer-driver-heading", "shell-primer-driver"),
    make("h3", { i18n: "shell-primer-risk-heading" }),
    make("p", { i18n: "shell-primer-risk" }),
    make("h3", { i18n: "shell-primer-commands-heading" }),
    commandList(SHELL_COMMANDS[shell]),
    make("p", { className: "hint", i18n: "shell-primer-writes" }),
    ...(shell === "service"
      ? [
          make("h3", { i18n: "shell-primer-diagnostics-heading" }),
          commandList(SERVICE_DIAGNOSTICS),
          make("p", { i18n: "shell-primer-diagnostics-env" }),
        ]
      : []),
  );
  body.hidden = true;

  const toggle = make("button", { i18n: "btn-shell-primer" });
  toggle.setAttribute("aria-expanded", "false");
  toggle.addEventListener("click", () => {
    body.hidden = !body.hidden;
    toggle.setAttribute("aria-expanded", String(!body.hidden));
    // The label has to name what the next click does, or the button becomes a
    // switch with no visible position.
    toggle.dataset.i18n = body.hidden ? "btn-shell-primer" : "btn-shell-primer-hide";
    toggle.textContent = tr(toggle.dataset.i18n);
  });

  return card(toggle, body);
}

/** How to run the shell in the console, and what it needs to work. */
function cardServiceConsole(): HTMLElement {
  const status = statusLine();
  const command = "pmca-console serviceshell";

  const copy = make("button", { i18n: "btn-copy-command" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(command);
      showDone(status, tr("msg-command-copied"));
    } catch (e) {
      showError(status, e);
    }
  });

  return card(
    make("h2", { i18n: "service-console-heading" }),
    make("p", { i18n: "service-console-hint" }),
    make(
      "div",
      { className: "link-row" },
      make("code", { textContent: command }),
      copy,
    ),
    status,
    ...driverAdvice("service-driver-heading", "service-driver-hint"),
    make("p", { className: "hint", i18n: "service-advanced-hint" }),
  );
}

/**
 * The service shell, run inside the window.
 *
 * It reads as a command line rather than a form because that is what it is:
 * the arguments are hexadecimal offsets and device paths, the audience is
 * someone who would otherwise be at a console, and a second dialect of the
 * same commands would be one more thing that can disagree with the first. The
 * backend parses every line, so a line typed here means exactly what it means
 * in `pmca-console`.
 *
 * What the window adds is a file picker: `pull`, `push`, `rmem` and `bk d` all
 * name a local file, and a path chosen from a dialog cannot be mistyped.
 */
function cardServiceShell(): HTMLElement {
  const status = statusLine();
  const output = make("pre", { className: "shell-output" });
  const input = make("input", { type: "text", className: "shell-input", spellcheck: false });
  input.disabled = true;

  const connect = make("button", { className: "primary", i18n: "btn-service-open" });
  const disconnect = make("button", { i18n: "btn-service-close" });
  const run = make("button", { i18n: "btn-service-run" });
  disconnect.disabled = true;
  run.disabled = true;

  /** Append a line to the transcript and keep the newest one in view. */
  const echo = (text: string, className?: string): void => {
    output.append(make("span", { className: className ?? "", textContent: `${text}\n` }));
    output.scrollTop = output.scrollHeight;
  };

  const setConnected = (connected: boolean): void => {
    connect.disabled = connected;
    disconnect.disabled = !connected;
    run.disabled = !connected;
    input.disabled = !connected;
    if (connected) input.focus();
  };

  connect.addEventListener("click", async () => {
    connect.disabled = true;
    showBusy(status, connectingText());
    try {
      await invoke("service_open");
      setConnected(true);
      showDone(status, tr("service-shell-ready"));
      echo(await invoke<string>("service_help"));
    } catch (e) {
      setConnected(false);
      showError(status, e);
    }
  });

  disconnect.addEventListener("click", async () => {
    try {
      await invoke("service_close");
      showDone(status, tr("service-shell-done"));
    } catch (e) {
      showError(status, e);
    } finally {
      setConnected(false);
    }
  });

  /** Send the current line, echoing it so the transcript reads as a session. */
  const submit = async (): Promise<void> => {
    const line = input.value;
    if (line.trim() === "") return;
    input.value = "";
    echo(`senser> ${line}`, "shell-echo");
    run.disabled = true;
    showBusy(status, tr("msg-working"));
    try {
      const outcome = await invoke<ShellOutcome | null>("service_run", { line });
      if (outcome === null) return;
      echo(outcome.message);
      if (outcome.dump !== null) echo(outcome.dump.trimEnd());
      if (outcome.finished) {
        setConnected(false);
        showDone(status, outcome.message);
        return;
      }
      showDone(status, outcome.message);
    } catch (e) {
      echo(errorText(e), "shell-error");
      showError(status, e);
    } finally {
      // A failed command does not end the session, so the prompt comes back
      // either way — except when `exit` closed it, which cleared the flag.
      if (!disconnect.disabled) {
        run.disabled = false;
        input.focus();
      }
    }
  };

  run.addEventListener("click", () => void submit());
  input.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      void submit();
    }
  });

  const browse = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind: "any" }),
    (path) => appendPickedPath(input, status, path),
  );
  browse.dataset.i18n = "btn-service-browse";
  browse.textContent = tr("btn-service-browse");

  return card(
    make("h2", { i18n: "nav-service" }),
    make("div", { className: "row" }, connect, disconnect),
    output,
    make(
      "div",
      { className: "row" },
      make("label", { i18n: "service-command-label" }),
      input,
      browse,
      run,
    ),
    status,
  );
}

/**
 * The updater-shell page.
 *
 * Built like the service page, and gated behind the same switch, because the
 * risk is of the same order: this one reboots the camera into Sony's factory
 * updater mode and writes our own payload into it. What the payload offers —
 * reading and writing the camera's persistent settings memory — is exactly what
 * can leave a camera unusable if it is interrupted.
 *
 * It is a page of its own rather than a second tab on the service page because
 * the two are different shells: different protocols, different command sets,
 * different driver requirements, and only one of them can be reached at a time.
 */
export function viewUpdater(host: HTMLElement): void {
  host.append(card(make("p", { className: "warning", i18n: "updater-warning" })));
  host.append(advancedMode() ? cardUpdaterShell() : cardUpdaterConsole());
  host.append(cardShellPrimer("updater"));
}

/** How to run the updater shell in the console, and what it needs to work. */
function cardUpdaterConsole(): HTMLElement {
  const status = statusLine();
  const command = "pmca-console updatershell --flash";

  const copy = make("button", { i18n: "btn-copy-command" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(command);
      showDone(status, tr("msg-command-copied"));
    } catch (e) {
      showError(status, e);
    }
  });

  return card(
    make("h2", { i18n: "updater-console-heading" }),
    make("p", { i18n: "service-console-hint" }),
    make("div", { className: "link-row" }, make("code", { textContent: command }), copy),
    status,
    ...driverAdvice("service-driver-heading", "updater-driver-hint"),
    make("p", { className: "hint", i18n: "service-advanced-hint" }),
  );
}

/**
 * The updater shell, run inside the window.
 *
 * The transcript half is the service shell's, deliberately: the same prompt,
 * the same file picker, the same rule that the backend parses every line. What
 * this one adds is the way in, which is a flash — so a payload of one's own
 * can be named, and the button asks before it writes, exactly as the firmware
 * page does.
 *
 * There is no field for the model. The camera reports it while it is still in
 * normal mode, and once it sits in updater mode it cannot report anything, so
 * a typed model could only ever disagree with the camera. What was read is
 * shown in the transcript instead.
 *
 * The one command missing here is `shell`. It needs a terminal to tunnel to,
 * and answering with the console invocation is more honest than a button that
 * would open nothing.
 */
function cardUpdaterShell(): HTMLElement {
  const status = statusLine();
  const output = make("pre", { className: "shell-output" });
  const input = make("input", { type: "text", className: "shell-input", spellcheck: false });
  input.disabled = true;

  const fdatInput = make("input", { type: "text", className: "path-input", placeholder: ".dat" });

  const connect = make("button", { className: "primary", i18n: "btn-service-open" });
  const disconnect = make("button", { i18n: "btn-service-close" });
  const run = make("button", { i18n: "btn-service-run" });
  disconnect.disabled = true;
  run.disabled = true;

  /** Append a line to the transcript and keep the newest one in view. */
  const echo = (text: string, className?: string): void => {
    output.append(make("span", { className: className ?? "", textContent: `${text}\n` }));
    output.scrollTop = output.scrollHeight;
  };

  const setConnected = (connected: boolean): void => {
    connect.disabled = connected;
    disconnect.disabled = !connected;
    run.disabled = !connected;
    input.disabled = !connected;
    fdatInput.disabled = connected;
    if (connected) input.focus();
  };

  connect.addEventListener("click", async () => {
    // Getting here costs a reboot and a write, so it is asked for once, in
    // plain words, before anything touches the camera.
    if (!(await confirmed(tr("updater-confirm"), tr("btn-confirm-updater")))) {
      showStatus(status, tr("updater-cancelled"));
      return;
    }
    connect.disabled = true;
    showBusy(status, connectingText());
    // The milestones worth keeping — the model read from the camera, the
    // guard's verdict, the firmware it runs — arrive as progress events and
    // would otherwise only flicker past in the status line.
    const unlisten = await listen<ProgressPayload>(PROGRESS_EVENT, (event) => {
      if (event.payload.kind === "info") echo(event.payload.message);
    });
    try {
      await invoke("updater_open", {
        fdatFile: fdatInput.value.trim(),
        confirm: true,
      });
      setConnected(true);
      showDone(status, tr("updater-shell-ready"));
      echo(await invoke<string>("updater_help"));
    } catch (e) {
      setConnected(false);
      showError(status, e);
    } finally {
      unlisten();
    }
  });

  disconnect.addEventListener("click", async () => {
    try {
      await invoke("updater_close");
      showDone(status, tr("updater-shell-done"));
    } catch (e) {
      showError(status, e);
    } finally {
      setConnected(false);
    }
  });

  /** Send the current line, echoing it so the transcript reads as a session. */
  const submit = async (): Promise<void> => {
    const line = input.value;
    if (line.trim() === "") return;
    input.value = "";
    echo(`pmca> ${line}`, "shell-echo");
    run.disabled = true;
    showBusy(status, tr("msg-working"));
    try {
      const outcome = await invoke<ShellOutcome | null>("updater_run", { line });
      if (outcome === null) return;
      echo(outcome.message);
      if (outcome.dump !== null) echo(outcome.dump.trimEnd());
      if (outcome.finished) {
        setConnected(false);
        showDone(status, outcome.message);
        return;
      }
      showDone(status, outcome.message);
    } catch (e) {
      echo(errorText(e), "shell-error");
      showError(status, e);
    } finally {
      // A failed command does not end the session, so the prompt comes back
      // either way — except when `exit` closed it, which cleared the flag.
      if (!disconnect.disabled) {
        run.disabled = false;
        input.focus();
      }
    }
  };

  run.addEventListener("click", () => void submit());
  input.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      void submit();
    }
  });

  const browseFdat = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind: "firmware" }),
    (path) => {
      fdatInput.value = path;
      showStatus(status, path);
    },
  );

  const browse = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind: "any" }),
    (path) => appendPickedPath(input, status, path),
  );
  browse.dataset.i18n = "btn-service-browse";
  browse.textContent = tr("btn-service-browse");

  return card(
    make("h2", { i18n: "nav-updater" }),
    make("label", { i18n: "updater-fdat-label" }),
    fdatInput,
    make("div", { className: "row" }, browseFdat, connect, disconnect),
    output,
    make(
      "div",
      { className: "row" },
      make("label", { i18n: "service-command-label" }),
      input,
      browse,
      run,
    ),
    status,
  );
}
