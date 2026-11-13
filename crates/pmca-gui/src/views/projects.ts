/**
 * Projects: independent GitHub repositories the user follows here.
 *
 * Nothing of them is downloaded or installed; the page only shows what GitHub
 * and the project's own `pmca-project.yaml` say. GitHub is asked only when the
 * user clicks, and the last answers are kept for the rest of the session.
 */

import { invoke } from "@tauri-apps/api/core";
import { card, fieldList, make, showBusy, showDone, showError, showStatus, statusLine } from "../dom";
import type { PinnedProject, ProjectDetails, ProjectFailure, ProjectInfo, ProjectList, ProjectRepo } from "../dto";
import { currentLocale, errorText, tr, trFill } from "../i18n";
import { detailsLine, detailsPanel, disclosure, starsText } from "./sources";

/** The last refresh of the pinned projects, by lowercase id. */
const known = new Map<string, ProjectInfo>();

/** Names of platforms that are not translated. */
const PLATFORM_NAMES: Record<string, string> = {
  windows: "Windows",
  macos: "macOS",
  linux: "Linux",
  android: "Android",
  web: "Web",
};

export function viewProjects(host: HTMLElement): void {
  const pinned = cardPinned();
  host.append(
    make("div", { className: "notice-block" }, make("p", { i18n: "projects-notice" })),
    pinned.card,
    cardSearch(pinned.reload),
  );
}

function dateText(iso: string | null | undefined): string | null {
  return iso ? new Date(iso).toLocaleDateString(currentLocale) : null;
}

function platformName(id: string): string {
  return PLATFORM_NAMES[id] ?? tr(`projects-platform-${id}`);
}

function badge(key: string, className = "archived-badge"): HTMLElement {
  return make("span", { className, i18n: key });
}

/** Everything GitHub and the project file say, with the addresses to copy. */
function infoPanel(info: ProjectInfo): HTMLElement {
  const { repo, details, release } = info;
  const description =
    repo.description && repo.description !== details?.summary ? make("p", { textContent: repo.description }) : null;
  return detailsPanel(
    [
      description,
      details ? null : make("p", { className: "hint", i18n: "projects-no-file-note" }),
      fieldList([
        ["projects-field-category", details ? { key: `projects-category-${details.category}` } : null],
        ["projects-field-author", details?.author],
        ["projects-field-release", release?.version ?? { key: "projects-no-release" }],
        ["projects-field-released", dateText(release?.publishedAt)],
        ["projects-field-cameras", details?.cameras.join(", ")],
        ["projects-field-platforms", details?.platforms.map(platformName).join(", ")],
        ["projects-field-license", repo.license],
        ["projects-field-language", repo.language],
        ["projects-field-updated", dateText(repo.pushedAt)],
        ["projects-field-forks", repo.forks > 0 ? repo.forks.toLocaleString(currentLocale) : null],
      ]),
    ],
    [
      { labelKey: "details-repository", url: repo.url },
      { labelKey: "details-releases", url: release?.url },
      { labelKey: "details-homepage", url: details?.homepage ?? repo.homepage },
    ],
  );
}

/** Name, stars and the state badges of a project, in one line. */
function headLine(repo: ProjectRepo, details: ProjectDetails | null): HTMLElement {
  const head = make("div", { className: "row" }, make("strong", { textContent: details?.name ?? repo.id }));
  if (details?.name) head.append(make("span", { className: "hint", textContent: repo.id }));
  head.append(make("span", { className: "stars", title: tr("sources-stars-title"), textContent: starsText(repo.stars) }));
  if (details) head.append(badge(`projects-category-${details.category}`));
  if (repo.archived) head.append(badge("sources-archived"));
  return head;
}

function releaseBadges(info: ProjectInfo): Node[] {
  if (!info.release) return [make("span", { className: "hint", i18n: "projects-no-release" })];
  const version = make("span", { className: "project-release", textContent: info.release.version });
  if (!info.newRelease) return [version];
  return [version, make("span", { className: "project-new", i18n: "projects-new", title: tr("projects-new-title") })];
}

/** The pinned projects, with a refresh that asks GitHub for all of them. */
function cardPinned(): { card: HTMLElement; reload: () => Promise<void> } {
  const status = statusLine();
  const list = make("div", { className: "plugin-list" });
  const refresh = make("button", { className: "primary", i18n: "btn-projects-refresh" });

  function row(pin: PinnedProject): HTMLElement {
    const info = known.get(pin.id.toLowerCase());
    const unpin = make("button", { i18n: "btn-project-unpin" });
    unpin.addEventListener("click", async () => {
      try {
        await invoke("projects_unpin", { id: pin.id });
        known.delete(pin.id.toLowerCase());
        showDone(status, trFill("projects-unpinned", { project: pin.id }));
        await reload();
      } catch (e) {
        showError(status, e);
      }
    });
    if (!info) {
      const head = make("div", { className: "row" }, make("strong", { textContent: pin.id }), unpin);
      return make("div", { className: "plugin-row" }, head);
    }
    const head = headLine(info.repo, info.details);
    head.append(...releaseBadges(info), unpin);
    const summary = info.details?.summary ?? info.repo.description;
    const panel = infoPanel(info);
    return make(
      "div",
      { className: "plugin-row" },
      head,
      summary ? make("p", { textContent: summary }) : "",
      detailsLine(panel),
      panel,
    );
  }

  async function reload(): Promise<void> {
    const pins = await invoke<PinnedProject[]>("projects_list");
    list.replaceChildren();
    if (pins.length === 0) {
      list.append(make("p", { className: "hint", i18n: "projects-pinned-none" }));
    } else if (pins.every((pin) => !known.has(pin.id.toLowerCase()))) {
      list.append(make("p", { className: "hint", i18n: "projects-pinned-hint" }));
    }
    list.append(...pins.map(row));
    refresh.disabled = pins.length === 0;
  }

  function failureLine(failure: ProjectFailure): HTMLElement {
    return make("p", { className: "error-text", textContent: `${failure.id}: ${errorText(failure)}` });
  }

  refresh.addEventListener("click", async () => {
    refresh.disabled = true;
    showBusy(status, tr("projects-refreshing"));
    try {
      const result = await invoke<ProjectList>("projects_refresh");
      for (const info of result.entries) known.set(info.repo.id.toLowerCase(), info);
      await reload();
      list.append(...result.failed.map(failureLine));
      if (result.failed.length === 0) showDone(status, tr("projects-refreshed"));
      else showStatus(status, tr("projects-refreshed-partly"));
    } catch (e) {
      showError(status, e);
    } finally {
      refresh.disabled = false;
    }
  });

  // Seed from the on-disk cache first, so pinned projects show their content at
  // once on open, without asking GitHub; a refresh is only for updates.
  void invoke<ProjectInfo[]>("projects_cached")
    .then((cached) => {
      for (const info of cached) known.set(info.repo.id.toLowerCase(), info);
    })
    .catch(() => undefined)
    .then(() => reload())
    .catch((e) => showError(status, e));
  return {
    card: card(
      make("h2", { i18n: "projects-pinned-heading" }),
      make("div", { className: "row" }, refresh),
      status,
      list,
    ),
    reload,
  };
}

/** The search on GitHub; a hit can be opened for details and pinned. */
function cardSearch(onPinned: () => Promise<void>): HTMLElement {
  const button = make("button", { i18n: "btn-projects-search" });
  const status = statusLine();
  const results = make("div", { className: "plugin-list" });

  function hitRow(hit: ProjectRepo): HTMLElement {
    const head = headLine(hit, null);
    const pinnedBadge = (): HTMLElement => make("span", { className: "hint", i18n: "projects-pinned-badge" });
    if (hit.pinned) {
      head.append(pinnedBadge());
    } else {
      const pin = make("button", { i18n: "btn-project-pin" });
      pin.addEventListener("click", async () => {
        pin.disabled = true;
        showBusy(status, tr("projects-pinning"));
        try {
          const info = await invoke<ProjectInfo>("projects_pin", { spec: hit.id });
          known.set(info.repo.id.toLowerCase(), info);
          pin.replaceWith(pinnedBadge());
          showDone(status, trFill("projects-pinned-done", { project: info.repo.id }));
          await onPinned();
        } catch (e) {
          pin.disabled = false;
          showError(status, e);
        }
      });
      head.append(pin);
    }

    // Filled on first opening: reading a project costs requests GitHub limits.
    const panel = make("div");
    const panelStatus = statusLine();
    let loaded = false;
    const toggle = disclosure(panel, false, (open) => {
      if (!open || loaded) return;
      loaded = true;
      panel.replaceChildren(panelStatus);
      showBusy(panelStatus, tr("projects-loading"));
      invoke<ProjectInfo>("project_info", { id: hit.id })
        .then((info) => {
          const summary = info.details?.summary;
          panel.replaceChildren(...(summary ? [make("p", { textContent: summary })] : []), infoPanel(info));
        })
        .catch((e) => {
          loaded = false;
          showError(panelStatus, e);
        });
    });
    return make(
      "div",
      { className: "plugin-row" },
      head,
      hit.description ? make("p", { textContent: hit.description }) : "",
      make("div", { className: "details-line" }, toggle),
      panel,
    );
  }

  button.addEventListener("click", async () => {
    button.disabled = true;
    results.replaceChildren();
    showBusy(status, tr("projects-searching"));
    try {
      const hits = await invoke<ProjectRepo[]>("projects_search");
      if (hits.length === 0) showStatus(status, tr("projects-search-none"));
      else showStatus(status, trFill("projects-search-found", { count: hits.length }));
      results.append(...hits.map(hitRow));
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "projects-search-heading" }),
    make("p", { className: "hint", i18n: "projects-search-hint" }),
    make("div", { className: "row" }, button),
    status,
    results,
  );
}
