/** Diagnosis page: reads that help find out why something goes wrong; nothing here writes to the camera. */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  clearStatusState,
  connectingText,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  showStatus,
  statusLine,
} from "../dom";
import type { UsageLogEntry, UsageLogOutcome, UsageLogView } from "../dto";
import { isCommandError, tr, trFill } from "../i18n";
import { hostPlatform } from "../platform";

export function viewDiagnose(host: HTMLElement): void {
  host.append(
    card(make("p", { i18n: "diagnose-hint" })),
    ...(hostPlatform === "windows" ? [cardServiceDiagnosis()] : []),
    cardUsageLog(),
  );
}

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
function cardServiceDiagnosis(): HTMLElement {
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

/** Entries shown at first and added by each "show more". */
const PAGE = 200;

/** A camera that refuses the log request says so, rather than "function not supported". */
function showUsageLogError(status: HTMLElement, error: unknown): void {
  if (isCommandError(error) && error.code === "usb-unsupported") {
    showErrorKey(status, "usage-log-unsupported");
  } else {
    showError(status, error);
  }
}

/** Entry count of the previous read in this run, to report what is new. */
let previousCount: number | null = null;

const hex = (value: number, digits: number): string =>
  `0x${value.toString(16).padStart(digits, "0")}`;

/**
 * Read the camera's usage log and show it as numbers; saving is optional.
 *
 * Nothing is named or interpreted: the format is undocumented, so the page
 * shows what the parser can vouch for and leaves the meaning to the reader.
 */
function cardUsageLog(): HTMLElement {
  const status = statusLine();
  const result = make("div");
  const read = make("button", { className: "primary", i18n: "btn-usage-log-read" });
  const save = make("button", { i18n: "btn-usage-log" });

  read.addEventListener("click", async () => {
    read.disabled = true;
    save.disabled = true;
    showBusy(status, connectingText());
    try {
      const view = await invoke<UsageLogView>("read_usage_log");
      showStatus(status, "");
      const fresh = previousCount === null ? null : view.entries.length - previousCount;
      previousCount = view.entries.length;
      result.replaceChildren(...renderUsageLog(view, fresh));
    } catch (e) {
      showUsageLogError(status, e);
    } finally {
      read.disabled = false;
      save.disabled = false;
    }
  });

  save.addEventListener("click", async () => {
    read.disabled = true;
    save.disabled = true;
    showBusy(status, connectingText());
    try {
      // The backend asks for the destination before it touches the camera.
      const written = await invoke<UsageLogOutcome | null>("save_usage_log");
      if (written === null) {
        clearStatusState(status);
        status.textContent = "";
        return;
      }
      showDone(status, `${tr("usage-log-written")} ${written.path} (${written.bytes})`);
    } catch (e) {
      showUsageLogError(status, e);
    } finally {
      read.disabled = false;
      save.disabled = false;
    }
  });

  const info = make(
    "div",
    { className: "primer" },
    make("h3", { i18n: "usage-log-info-what-heading" }),
    make("p", { i18n: "usage-log-info-what" }),
    make("p", { i18n: "usage-log-info-read" }),
    make("h3", { i18n: "usage-log-info-flush-heading" }),
    make("p", { i18n: "usage-log-info-flush" }),
    make("h3", { i18n: "usage-log-info-content-heading" }),
    make("p", { i18n: "usage-log-info-shutter" }),
    make("p", { i18n: "usage-log-info-clock" }),
    make("p", { i18n: "usage-log-info-unknown" }),
    make("h3", { i18n: "usage-log-info-privacy-heading" }),
    make("p", { i18n: "usage-log-info-privacy" }),
  );
  info.hidden = true;

  const toggle = make("button", { i18n: "btn-usage-log-info" });
  toggle.setAttribute("aria-expanded", "false");
  toggle.addEventListener("click", () => {
    info.hidden = !info.hidden;
    toggle.setAttribute("aria-expanded", String(!info.hidden));
    toggle.dataset.i18n = info.hidden ? "btn-usage-log-info" : "btn-usage-log-info-hide";
    toggle.textContent = tr(toggle.dataset.i18n);
  });

  return card(
    make("h2", { i18n: "usage-log-heading" }),
    make("p", { i18n: "usage-log-hint" }),
    make("div", { className: "row" }, read, save),
    status,
    make("div", { className: "row" }, toggle),
    info,
    result,
  );
}

/** Header figures, the tally as filter chips, and the entries newest first. */
function renderUsageLog(view: UsageLogView, fresh: number | null): Node[] {
  const newestFirst = [...view.entries].reverse();
  let filter: number | null = null;
  let limit = PAGE;

  const figures = make(
    "table",
    { className: "analysis-table" },
    make(
      "tbody",
      {},
      ...(
        [
          ["usage-log-bytes", view.bytes],
          ["usagelog-size", view.logSize],
          ["usagelog-cursor", view.cursor],
          ["usagelog-spare", view.spareSize],
          ["usagelog-count", view.entries.length],
        ] as [string, number][]
      ).map(([key, value]) =>
        make(
          "tr",
          {},
          make("th", { i18n: key }),
          make("td", { className: "mono", textContent: String(value) }),
        ),
      ),
    ),
  );

  const chips = make("div", { className: "row cmp-chips" });
  const table = make("div");
  const more = make("button", { i18n: "btn-usage-log-more" });
  const showing = make("p", { className: "hint" });

  const drawChips = (): void => {
    chips.replaceChildren(
      ...view.events.map((e) => {
        const chip = make("button", {
          className: e.event === filter ? "cmp-chip cmp-mono on" : "cmp-chip cmp-mono",
          textContent: `${hex(e.event, 4)} · ${e.count}`,
        });
        chip.setAttribute("aria-pressed", String(e.event === filter));
        chip.addEventListener("click", () => {
          filter = e.event === filter ? null : e.event;
          limit = PAGE;
          drawChips();
          drawEntries();
        });
        return chip;
      }),
    );
  };

  const drawEntries = (): void => {
    const rows = filter === null ? newestFirst : newestFirst.filter((e) => e.event === filter);
    const shown = rows.slice(0, limit);
    table.replaceChildren(entriesTable(shown));
    showing.textContent = trFill("usage-log-showing", { shown: shown.length, total: rows.length });
    more.hidden = shown.length >= rows.length;
  };

  more.addEventListener("click", () => {
    limit += PAGE;
    drawEntries();
  });
  drawChips();
  drawEntries();

  return [
    figures,
    ...(fresh === null ? [] : [make("p", { textContent: trFill("usage-log-new", { count: fresh }) })]),
    make("h3", { i18n: "usagelog-events" }),
    make("p", { className: "hint", i18n: "usage-log-events-hint" }),
    chips,
    make("h3", { i18n: "usagelog-count" }),
    showing,
    table,
    make("div", { className: "row" }, more),
  ];
}

function entriesTable(entries: UsageLogEntry[]): HTMLElement {
  const head = [
    "usage-log-col-offset",
    "usage-log-col-event",
    "usage-log-col-source",
    "usage-log-col-value",
    "usage-log-col-time",
  ];
  return make(
    "table",
    { className: "analysis-table" },
    make("thead", {}, make("tr", {}, ...head.map((key) => make("th", { i18n: key })))),
    make(
      "tbody",
      {},
      ...entries.map((e) =>
        make(
          "tr",
          {},
          make("td", { className: "mono", textContent: hex(e.offset, 8) }),
          make("td", { className: "mono", textContent: hex(e.event, 4) }),
          make("td", { className: "mono", textContent: hex(e.source, 4) }),
          make("td", { className: "mono", textContent: hex(e.value, 8) }),
          make("td", { textContent: e.time ?? "" }),
        ),
      ),
    ),
  );
}
