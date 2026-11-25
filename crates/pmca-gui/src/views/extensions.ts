/** Extensions page: app and plugin sources, installed plugins, and publishing your own. */

import { cardPlugins } from "./plugins";
import { cardPublish, cardSources } from "./sources";

/** Everything about where apps and plugins come from, apart from the apps themselves. */
export function viewExtensions(host: HTMLElement): void {
  host.append(cardSources(), cardPublish(), cardPlugins());
}
