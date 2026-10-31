/** Sources: the GitHub repositories apps and plugins come from (Settings). */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  confirmed,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  showStatus,
  statusLine,
} from "../dom";
import type { CommunityApp, Source, SourceRepo, SourcesState, SourceSuggestion } from "../dto";
import { currentLocale, tr, trFill } from "../i18n";

/** GitHub's answers per repository, asked once per window: its rate limit is low. */
const repos = new Map<string, Promise<SourceRepo | null>>();

/** GitHub's description of a source, `null` when it could not be asked. */
export function sourceRepo(id: string): Promise<SourceRepo | null> {
  const key = id.toLowerCase();
  let answer = repos.get(key);
  if (!answer) {
    answer = invoke<SourceRepo>("source_repo", { id }).catch(() => null);
    repos.set(key, answer);
  }
  return answer;
}

function starsText(stars: number): string {
  return `★ ${stars.toLocaleString(currentLocale)}`;
}

/** A source's stars, filled in once GitHub has answered. */
export function starsBadge(id: string): HTMLElement {
  const badge = make("span", { className: "stars", title: tr("sources-stars-title") });
  void sourceRepo(id).then((repo) => {
    if (repo) badge.textContent = starsText(repo.stars);
  });
  return badge;
}

/** The repository's address with a copy button: no opener plugin is bundled. */
function addressRow(labelKey: string, url: string, status: HTMLElement): HTMLElement {
  const copy = make("button", { i18n: "btn-copy-url" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(url);
      showDone(status, tr("msg-url-copied-browser"));
    } catch (e) {
      showError(status, e);
    }
  });
  return make(
    "div",
    { className: "row address-row" },
    make("span", { className: "hint", i18n: labelKey }),
    make("code", { textContent: url }),
    copy,
  );
}

/** An address to look at in the browser, under the label `labelKey`. */
export interface Link {
  labelKey: string;
  url: string | null | undefined;
}

/**
 * An entry's description and the addresses to look at, folded away under
 * {@link disclosure}. Each address appears once, even if two labels name it,
 * and copying one reports right there rather than in a status line elsewhere.
 */
export function detailsPanel(lines: (Node | null)[], links: Link[]): HTMLElement {
  const status = statusLine();
  const seen = new Set<string>();
  const rows: HTMLElement[] = [];
  for (const link of links) {
    const key = link.url?.trim().replace(/\/+$/, "").toLowerCase();
    if (!link.url || !key || seen.has(key)) continue;
    seen.add(key);
    rows.push(addressRow(link.labelKey, link.url.trim(), status));
  }
  return make(
    "div",
    { className: "details-panel" },
    ...lines.filter((line): line is Node => line !== null),
    ...rows,
    status,
  );
}

/** A "Details" button that shows and hides `target`, telling `onToggle` the new state. */
export function disclosure(
  target: HTMLElement,
  open = false,
  onToggle?: (open: boolean) => void,
): HTMLButtonElement {
  const arrow = make("span", { ariaHidden: "true" });
  const button = make(
    "button",
    { className: "details-toggle", type: "button" },
    make("span", { i18n: "btn-details" }),
    " ",
    arrow,
  );
  const show = (on: boolean): void => {
    target.hidden = !on;
    button.setAttribute("aria-expanded", String(on));
    arrow.textContent = on ? "▾" : "▸";
    onToggle?.(on);
  };
  button.addEventListener("click", () => show(target.hidden !== false));
  show(open);
  return button;
}

/** The "Details" button on a line of its own under an entry's name. */
export function detailsLine(target: HTMLElement, open = false): HTMLElement {
  return make("div", { className: "details-line" }, disclosure(target, open));
}

/** GitHub's description and last change of a source, filled in once known. */
function repoLines(id: string): Node[] {
  const description = make("p", { hidden: true });
  const updated = make("p", { className: "hint", hidden: true });
  void sourceRepo(id).then((repo) => {
    if (repo?.description) {
      description.textContent = repo.description;
      description.hidden = false;
    }
    if (repo?.pushedAt) {
      updated.textContent = updatedText(repo.pushedAt);
      updated.hidden = false;
    }
  });
  return [description, updated];
}

function updatedText(pushedAt: string): string {
  return trFill("sources-updated", { date: new Date(pushedAt).toLocaleDateString(currentLocale) });
}

/** The details of an app or plugin a source offers: text, author, where it lives. */
export function entryDetails(entry: CommunityApp): HTMLElement {
  return detailsPanel(
    [
      entry.description ? make("p", { textContent: entry.description }) : null,
      entry.author
        ? make("p", { className: "hint" }, make("span", { i18n: "plugin-author" }), `: ${entry.author}`)
        : null,
      // Shown once GitHub reports the repository as archived.
      make("p", { className: "archived-note", i18n: "app-archived-note", hidden: true }),
    ],
    [
      { labelKey: "details-repository", url: entry.repository },
      { labelKey: "details-homepage", url: entry.homepage },
      { labelKey: "details-source", url: `https://github.com/${entry.source}` },
    ],
  );
}

/** `owner/repo` out of what was typed, as the backend reads it; `null` if none. */
function parseSpec(spec: string): string | null {
  const match = spec
    .trim()
    .match(/^(?:(?:https?:\/\/)?(?:www\.)?github\.com\/|github:)?([A-Za-z0-9-]+)\/([\w.-]+?)(?:\.git)?(?:\/.*)?$/i);
  return match ? `${match[1]}/${match[2]}` : null;
}

/**
 * Ask, naming the repository and its address, then add it; `true` once added.
 *
 * The program checks nothing, so the decision has to be made by someone who
 * has looked.
 */
async function confirmAndAdd(spec: string, id: string, status: HTMLElement): Promise<boolean> {
  const url = `https://github.com/${id}`;
  if (!(await confirmed(trFill("sources-add-confirm", { source: id, url }), tr("btn-confirm-source-add")))) {
    return false;
  }
  showBusy(status, tr("sources-adding"));
  try {
    const source = await invoke<Source>("sources_add", { spec });
    showDone(status, trFill("sources-added", { source: source.id }));
    return true;
  } catch (e) {
    showError(status, e);
    return false;
  }
}

/** The search for sources on GitHub: its button, status line and hits. */
export interface SourceSearch {
  button: HTMLButtonElement;
  status: HTMLElement;
  results: HTMLElement;
}

/**
 * Search GitHub for repositories with the source topic.
 *
 * With `onlyNew` the sources already added are left out, and a search that
 * finds nothing else says so. `onAdded` runs after a hit has been added.
 */
export function sourceSearch(onlyNew: boolean, onAdded: () => void): SourceSearch {
  const button = make("button", { i18n: "btn-source-search" });
  const status = statusLine();
  const results = make("div", { className: "plugin-list" });

  function hitRow(hit: SourceRepo): HTMLElement {
    const panel = detailsPanel(
      [
        hit.description ? make("p", { textContent: hit.description }) : null,
        hit.pushedAt ? make("p", { className: "hint", textContent: updatedText(hit.pushedAt) }) : null,
      ],
      [{ labelKey: "details-repository", url: hit.url }],
    );
    const head = make(
      "div",
      { className: "row" },
      make("strong", { textContent: hit.id }),
      make("span", { className: "stars", title: tr("sources-stars-title"), textContent: starsText(hit.stars) }),
    );
    if (hit.archived) head.append(make("span", { className: "hint", i18n: "sources-archived" }));
    // Open from the start: looking at the repository is the step the search asks for.
    const row = make("div", { className: "plugin-row" }, head, detailsLine(panel, true), panel);
    const already = (): HTMLElement => make("span", { className: "hint", i18n: "sources-already" });
    if (hit.added) {
      head.append(already());
    } else {
      const addHit = make("button", { i18n: "btn-source-add" });
      addHit.addEventListener("click", async () => {
        if (!(await confirmAndAdd(hit.id, hit.id, status))) return;
        if (onlyNew) row.remove();
        else addHit.replaceWith(already());
        onAdded();
      });
      head.append(addHit);
    }
    return row;
  }

  button.addEventListener("click", async () => {
    button.disabled = true;
    results.replaceChildren();
    showBusy(status, tr("sources-searching"));
    try {
      const hits = await invoke<SourceRepo[]>("sources_search");
      const shown = onlyNew ? hits.filter((hit) => !hit.added) : hits;
      if (hits.length === 0) showStatus(status, tr("sources-search-none"));
      else if (shown.length === 0) showStatus(status, tr("sources-search-all-added"));
      else showStatus(status, trFill("sources-search-found", { count: shown.length }));
      if (onlyNew && shown.length > 0) results.append(make("p", { className: "hint", i18n: "sources-search-hint" }));
      results.append(...shown.map(hitRow));
    } catch (e) {
      showError(status, e);
    } finally {
      button.disabled = false;
    }
  });

  return { button, status, results };
}

/**
 * Sources the user has added, the suggestion not yet switched on, adding one
 * by name and an explicit search on GitHub. Every way of adding asks first.
 */
export function cardSources(): HTMLElement {
  const status = statusLine();
  const list = make("div", { className: "plugin-list" });
  const suggestions = make("div");
  const input = make("input", {
    type: "text",
    className: "path-input",
    placeholder: "owner/repository",
    spellcheck: false,
  });
  const addButton = make("button", { className: "primary", i18n: "btn-source-add" });
  const search = sourceSearch(false, () => void refresh().catch((e) => showError(status, e)));

  async function refresh(): Promise<void> {
    const state = await invoke<SourcesState>("sources_list");
    list.replaceChildren();
    if (state.sources.length === 0) list.append(make("p", { className: "hint", i18n: "sources-none" }));
    for (const source of state.sources) list.append(sourceRow(source));
    suggestions.replaceChildren(...state.suggestions.map(suggestionRow));
  }

  async function add(spec: string, id: string): Promise<void> {
    if (!(await confirmAndAdd(spec, id, status))) return;
    input.value = "";
    try {
      await refresh();
    } catch (e) {
      showError(status, e);
    }
  }

  function sourceRow(source: Source): HTMLElement {
    const remove = make("button", { i18n: "btn-remove" });
    remove.addEventListener("click", async () => {
      if (!(await confirmed(trFill("sources-remove-confirm", { source: source.id }), tr("btn-remove")))) {
        return;
      }
      try {
        await invoke("sources_remove", { id: source.id });
        showDone(status, trFill("sources-removed", { source: source.id }));
        await refresh();
      } catch (e) {
        showError(status, e);
      }
    });
    const panel = detailsPanel(repoLines(source.id), [{ labelKey: "details-repository", url: source.url }]);
    const head = make(
      "div",
      { className: "row" },
      make("strong", { textContent: source.id }),
      starsBadge(source.id),
      remove,
    );
    return make("div", { className: "plugin-row" }, head, detailsLine(panel), panel);
  }

  function suggestionRow(suggestion: SourceSuggestion): HTMLElement {
    const activate = make("button", { i18n: "btn-source-activate" });
    activate.addEventListener("click", () => void add(suggestion.source.id, suggestion.source.id));
    const panel = detailsPanel(repoLines(suggestion.source.id), [
      { labelKey: "details-repository", url: suggestion.source.url },
    ]);
    return make(
      "div",
      { className: "plugin-row" },
      make("h3", { i18n: "sources-suggestion-heading" }),
      make(
        "div",
        { className: "row" },
        make("strong", { textContent: suggestion.source.id }),
        starsBadge(suggestion.source.id),
        activate,
      ),
      detailsLine(panel),
      panel,
      make("p", { i18n: suggestion.noteKey }),
    );
  }

  addButton.addEventListener("click", () => {
    const spec = input.value.trim();
    if (spec === "") return;
    const id = parseSpec(spec);
    if (id === null) {
      showErrorKey(status, "source-invalid");
      return;
    }
    void add(spec, id);
  });
  input.addEventListener("keydown", (event) => {
    if (event.key === "Enter") addButton.click();
  });

  refresh().catch((e) => showError(status, e));
  return card(
    make("h2", { i18n: "sources-heading" }),
    make("div", { className: "notice-block" }, make("p", { i18n: "sources-intro" })),
    list,
    suggestions,
    make("label", { className: "field-label after-list", i18n: "sources-add-label" }),
    make("div", { className: "row" }, input, addButton),
    status,
    make("h3", { i18n: "sources-search-heading" }),
    make("p", { className: "hint", i18n: "sources-search-hint" }),
    make("div", { className: "row" }, search.button),
    search.status,
    search.results,
  );
}
