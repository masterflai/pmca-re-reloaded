/** Extensions page: app and plugin sources, installed plugins, and publishing your own. */

import { cardPluginSources, cardPlugins } from "./plugins";
import { cardPublish, cardSources } from "./sources";

/** Everything about where apps and plugins come from, apart from the apps themselves. */
export function viewExtensions(host: HTMLElement): void {
  const plugins = cardPlugins();
  host.append(
    cardSources(),
    plugins.element,
    cardPluginSources(() => void plugins.refresh()),
    cardPublish(),
  );
}
