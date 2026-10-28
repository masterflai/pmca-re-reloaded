/** Key material: import and status, shown when the program has none. */

import { invoke } from "@tauri-apps/api/core";
import { card, make, showDone, showError, showErrorKey, statusLine, withProgressTarget } from "./dom";
import type { KeyStatus } from "./dto";
import { tr } from "./i18n";

/** The current state of the key store. */
export async function keyStatus(): Promise<KeyStatus> {
  return invoke<KeyStatus>("keys_status");
}

/** Which features need which key material, folded away until asked for. */
function keyNeeds(): HTMLDetailsElement {
  const list = make("dl");
  for (const feature of ["info", "install", "service", "updater"]) {
    list.append(
      make("dt", { i18n: `keys-needs-${feature}` }),
      make("dd", { i18n: `keys-needs-${feature}-keys` }),
    );
  }
  return make(
    "details",
    { className: "key-needs" },
    make("summary", { i18n: "keys-needs-toggle" }),
    list,
  );
}

/**
 * The card that explains the missing key material and imports it.
 *
 * No source is suggested: the user enters one they have. `onChange` lets the
 * caller (the welcome page) drop the card once every class is present.
 */
export function keyCard(onChange?: (status: KeyStatus) => void): HTMLElement {
  const status = statusLine();
  const list = make("div", { className: "key-status" });
  const urls = make("textarea", {
    className: "path-input key-urls",
    rows: 2,
    placeholder: "https://",
  });
  const importButton = make("button", { className: "primary", i18n: "btn-keys-import" });

  const render = (s: KeyStatus): void => {
    list.replaceChildren(
      ...s.classes.map((c) =>
        make(
          "div",
          { className: "key-row" },
          make("span", { i18n: `keys-class-${c.slug}` }),
          make("span", {
            className: c.present ? "key-present" : "key-missing",
            i18n: c.present ? "keys-present" : "keys-missing",
          }),
        ),
      ),
    );
  };

  importButton.addEventListener("click", async () => {
    const entered = urls.value.split(/\s+/).map((u) => u.trim()).filter(Boolean);
    if (entered.length === 0) {
      showErrorKey(status, "keys-url-required");
      return;
    }
    importButton.disabled = true;
    try {
      const result = await withProgressTarget(status, () =>
        invoke<KeyStatus>("keys_import", { urls: entered }),
      );
      render(result);
      showDone(status, tr(result.complete ? "keys-complete" : "keys-incomplete"));
      onChange?.(result);
    } catch (e) {
      showError(status, e);
    } finally {
      importButton.disabled = false;
    }
  });

  void keyStatus().then(render).catch(() => undefined);

  return card(
    make("h2", { i18n: "keys-heading" }),
    make("p", { i18n: "keys-intro" }),
    keyNeeds(),
    make("div", { className: "notice-block" }, make("p", { i18n: "keys-law-notice" })),
    list,
    make("label", { className: "field-label", i18n: "keys-url-label" }),
    make("div", { className: "row" }, urls),
    make("div", { className: "row" }, importButton),
    status,
  );
}
