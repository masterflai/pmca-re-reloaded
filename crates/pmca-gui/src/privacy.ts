/**
 * Values that may be sensitive: whether this session shows them, the box that
 * explains what they are, and the switches that say whether an export carries
 * them.
 *
 * The list itself lives in the backend (`pmca_app::sensitive`), so the window,
 * the console and every export describe the same thing.
 */

import { invoke } from "@tauri-apps/api/core";
import type { Catalog, SensitiveKind } from "./dto";
import { confirmed, make } from "./dom";
import { tr } from "./i18n";

/** Shown as they are only on request, and only until the program is closed. */
let revealed = false;

/** Whether this session shows values that may be sensitive. */
export function revealSensitive(): boolean {
  return revealed;
}

/** A kind of sensitive value in words. */
export function kindLabel(kind: SensitiveKind | undefined): string {
  switch (kind) {
    case "serial":
      return tr("sensitive-kind-serial");
    case "owner":
      return tr("sensitive-kind-owner");
    case "credentials":
      return tr("sensitive-kind-credentials");
    case "networkName":
      return tr("sensitive-kind-network-name");
    case "address":
      return tr("sensitive-kind-address");
    case "account":
      return tr("sensitive-kind-account");
    case "iptc":
      return tr("sensitive-kind-iptc");
    case "pairing":
      return tr("sensitive-kind-pairing");
    case "location":
      return tr("sensitive-kind-location");
    case "text":
      return tr("sensitive-kind-text");
    default:
      return tr("sensitive-kind-file");
  }
}

/** Stars in place of a value, with the reason on hover. */
export function hiddenValue(kind: SensitiveKind | undefined): HTMLElement {
  return make("span", {
    className: "memory-redacted-badge",
    textContent: "••••••",
    title: `${tr("sensitive-hidden")} ${kindLabel(kind)}`,
  });
}

/** The mark beside a value that is shown although it may be sensitive. */
export function shownMark(kind: SensitiveKind | undefined): HTMLElement {
  return make("span", {
    className: "cmp-tag sensitive-tag",
    i18n: "sensitive-tag",
    title: `${tr("sensitive-shown")} ${kindLabel(kind)}`,
  });
}

/**
 * The box that says what may be sensitive, that it is replaced by default and
 * why that cannot be exact, and which files always hold everything.
 */
export function sensitiveNotice(open = false): HTMLElement {
  const body = make("div");
  const details = make(
    "details",
    { className: "notice-block sensitive-notice" },
    make("summary", { i18n: "sensitive-heading" }),
    body,
  );
  details.open = open;
  invoke<Catalog>("privacy_catalog")
    .then((catalog) => {
      body.replaceChildren(
        ...catalog.intro.map((line) => make("p", { textContent: line })),
        make(
          "ul",
          {},
          ...catalog.kinds.map((k) =>
            make("li", {}, make("strong", { textContent: k.label }), ` — ${k.detail}`),
          ),
        ),
        make("h4", { textContent: catalog.completeHeading }),
        make("ul", {}, ...catalog.complete.map((line) => make("li", { textContent: line }))),
      );
    })
    .catch(() => body.replaceChildren(make("p", { i18n: "sensitive-uncertain-short" })));
  return details;
}

/**
 * The switch that shows the values on screen. Turning it on asks first;
 * `changed` runs after every change that stuck.
 */
export function revealSwitch(changed: () => Promise<void>): HTMLElement {
  const box = make("input", { type: "checkbox", checked: revealed });
  box.addEventListener("change", async () => {
    if (box.checked && !(await confirmed(tr("sensitive-reveal-confirm"), tr("btn-sensitive-reveal")))) {
      box.checked = false;
      return;
    }
    revealed = box.checked;
    await changed();
  });
  return make(
    "div",
    { className: "cmp-way" },
    make("label", { className: "row" }, box, make("span", { i18n: "sensitive-reveal" })),
    make("p", { className: "hint", i18n: "sensitive-reveal-hint" }),
  );
}

/** The switch an export carries the values by; off unless ticked for this export. */
export function includeSwitch(): { element: HTMLElement; checked: () => boolean } {
  const box = make("input", { type: "checkbox" });
  return {
    element: make(
      "div",
      { className: "cmp-way" },
      make("label", { className: "row" }, box, make("span", { i18n: "sensitive-include" })),
      make("p", { className: "hint", i18n: "sensitive-include-hint" }),
    ),
    checked: () => box.checked,
  };
}
