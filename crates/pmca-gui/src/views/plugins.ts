/** Plugins: managing them (Settings) and running their tools (Tools). */

import { invoke } from "@tauri-apps/api/core";
import {
  card,
  clearStatusState,
  confirmed,
  make,
  showBusy,
  showDone,
  showError,
  showErrorKey,
  showNotice,
  statusLine,
} from "../dom";
import { browseButton } from "../drop";
import type {
  PluginEntry,
  PluginInfo,
  PluginList,
  PluginRunOutcome,
  PluginToolResult,
} from "../dto";
import { tr, trFill } from "../i18n";

/** The translated lines saying what a plugin may do. */
function permissionLines(info: PluginInfo): string[] {
  return info.permissions.map((p) => `• ${tr(`plugin-perm-${p}`)}`);
}

/**
 * Installed plugins, installing from a file and removing.
 *
 * A plugin comes only from a file the user picks; there is no store. Before
 * installing, the dialog lists what the plugin may do, so the decision is made
 * knowing it. Changes apply at the next start, which the card says whenever
 * the installed set differs from the one in effect.
 */
export function cardPlugins(): HTMLElement {
  const status = statusLine();
  const list = make("div", { className: "plugin-list" });
  const restart = make("p", { className: "warning", i18n: "plugin-restart" });
  restart.hidden = true;

  async function refresh(): Promise<void> {
    const state = await invoke<PluginList>("plugins_list");
    list.replaceChildren();
    restart.hidden = !state.restartRequired;
    if (state.plugins.length === 0 && state.failed.length === 0) {
      list.append(make("p", { className: "hint", i18n: "plugin-none" }));
    }
    for (const plugin of state.plugins) list.append(pluginRow(plugin));
    for (const failed of state.failed) {
      const reason = failed.unknownField
        ? trFill("plugin-unknown-field", { field: failed.unknownField })
        : trFill("plugin-invalid", { reason: failed.reason });
      list.append(make("p", { className: "error-text" }, `${failed.file}: `, reason));
    }
  }

  function pluginRow(plugin: PluginInfo): HTMLElement {
    const remove = make("button", { i18n: "btn-remove" });
    remove.addEventListener("click", async () => {
      if (!(await confirmed(trFill("plugin-remove-confirm", { name: plugin.name }), tr("btn-remove")))) return;
      try {
        await invoke("plugins_remove", { id: plugin.id });
        showDone(status, trFill("plugin-removed", { id: plugin.id }));
        await refresh();
      } catch (e) {
        showError(status, e);
      }
    });
    const title = make("strong", { textContent: `${plugin.name} ${plugin.version}` });
    const head = make("div", { className: "row" }, title, make("span", { className: "hint", textContent: plugin.id }));
    if (!plugin.active) head.append(make("span", { className: "hint", i18n: "plugin-inactive" }));
    head.append(remove);
    const body: (Node | string)[] = [head];
    if (plugin.author) body.push(make("p", { className: "hint" }, make("span", { i18n: "plugin-author" }), `: ${plugin.author}`));
    if (plugin.description) body.push(make("p", { textContent: plugin.description }));
    body.push(make("ul", {}, ...plugin.permissions.map((p) => make("li", { i18n: `plugin-perm-${p}` }))));
    return make("div", { className: "plugin-row" }, ...body);
  }

  const install = make("button", { className: "primary", i18n: "btn-plugin-install" });
  install.addEventListener("click", async () => {
    install.disabled = true;
    try {
      const path = await invoke<string | null>("pick_file", { kind: "plugin" });
      if (path === null) return;
      const info = await invoke<PluginInfo>("plugins_inspect", { path });
      const message = [
        `${info.name} ${info.version} (${info.id})`,
        info.author ? `${tr("plugin-author")}: ${info.author}` : "",
        "",
        tr("plugin-install-confirm"),
        ...permissionLines(info),
      ]
        .filter((line, i) => line !== "" || i === 2)
        .join("\n");
      if (!(await confirmed(message, tr("btn-confirm-plugin-install")))) return;
      await invoke<PluginInfo>("plugins_install", { path });
      showDone(status, tr("plugin-installed"));
      await refresh();
    } catch (e) {
      showError(status, e);
    } finally {
      install.disabled = false;
    }
  });

  refresh().catch((e) => showError(status, e));
  return card(
    make("h2", { i18n: "plugin-heading" }),
    make("p", { i18n: "plugin-intro" }),
    list,
    restart,
    make("div", { className: "row" }, install),
    status,
  );
}

/** Show a plugin's verdict in the status line and its report below. */
function showOutcome(status: HTMLElement, report: HTMLElement, outcome: PluginRunOutcome): void {
  if (outcome.status === "error") showErrorKey(status, "plugin-run-error");
  else if (outcome.status === "warning") showNotice(status, tr("plugin-run-warning"));
  else showDone(status, tr("plugin-run-done"));
  report.textContent = outcome.report.trimEnd();
}

/** One select of plugin entries, named in the user's language. */
function entrySelect(entries: PluginEntry[], hint: HTMLElement): HTMLSelectElement {
  const select = make("select", { className: "field-select" });
  entries.forEach((entry, i) => select.append(make("option", { value: String(i), i18n: entry.labelKey })));
  const showHint = (): void => {
    const key = entries[Number(select.value)]?.hintKey;
    hint.textContent = key ? tr(key) : "";
    if (key) hint.dataset.i18n = key;
    else delete hint.dataset.i18n;
  };
  select.addEventListener("change", showHint);
  showHint();
  return select;
}

/**
 * The tools and diagnostics of the installed plugins.
 *
 * Nothing is shown while there are none, so the page looks as it always did
 * for everyone who never installs a plugin. The cards are added once the
 * backend has answered.
 */
export function pluginToolCards(host: HTMLElement): void {
  Promise.all([
    invoke<PluginEntry[]>("plugins_entries", { diagnostics: false }),
    invoke<PluginEntry[]>("plugins_entries", { diagnostics: true }),
  ])
    .then(([tools, diagnostics]) => {
      if (tools.length > 0) host.append(toolCard(tools));
      if (diagnostics.length > 0) host.append(diagnosticCard(diagnostics));
    })
    .catch(() => undefined);
}

function toolCard(tools: PluginEntry[]): HTMLElement {
  const status = statusLine();
  const report = make("pre", { className: "shell-output" });
  const hint = make("p", { className: "hint" });
  const select = entrySelect(tools, hint);
  const inputPath = make("input", { type: "text", className: "path-input" });
  const browse = browseButton(
    status,
    () => invoke<string | null>("pick_file", { kind: "any" }),
    (path) => {
      inputPath.value = path;
    },
  );

  const run = make("button", { className: "primary", i18n: "btn-service-run" });
  run.addEventListener("click", async () => {
    const input = inputPath.value.trim();
    if (input === "") {
      showErrorKey(status, "tools-file-required");
      return;
    }
    const entry = tools[Number(select.value)];
    run.disabled = true;
    report.replaceChildren();
    showBusy(status, tr("msg-working"));
    try {
      const result = await invoke<PluginToolResult>("plugins_run_tool", {
        plugin: entry.plugin,
        tool: entry.id,
        input,
      });
      showOutcome(status, report, result.outcome);
      if (result.written !== null) report.append(`\n\n${trFill("tools-written", { path: result.written })}`);
    } catch (e) {
      showError(status, e);
    } finally {
      run.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "plugin-tools-heading" }),
    make("p", { i18n: "plugin-tools-hint" }),
    make("label", { className: "field-label", i18n: "tools-operation" }),
    select,
    hint,
    make("div", { className: "row" }, inputPath, browse),
    make("div", { className: "row" }, run),
    status,
    report,
  );
}

function diagnosticCard(diagnostics: PluginEntry[]): HTMLElement {
  const status = statusLine();
  const report = make("pre", { className: "shell-output" });
  const hint = make("p", { className: "hint" });
  const select = entrySelect(diagnostics, hint);

  const run = make("button", { className: "primary", i18n: "btn-service-run" });
  run.addEventListener("click", async () => {
    const entry = diagnostics[Number(select.value)];
    run.disabled = true;
    report.replaceChildren();
    clearStatusState(status);
    showBusy(status, tr("msg-working"));
    try {
      showOutcome(
        status,
        report,
        await invoke<PluginRunOutcome>("plugins_run_diagnostic", {
          plugin: entry.plugin,
          diagnostic: entry.id,
        }),
      );
    } catch (e) {
      showError(status, e);
    } finally {
      run.disabled = false;
    }
  });

  return card(
    make("h2", { i18n: "plugin-diagnostics-heading" }),
    make("p", { i18n: "plugin-tools-hint" }),
    select,
    hint,
    make("div", { className: "row" }, run),
    status,
    report,
  );
}
