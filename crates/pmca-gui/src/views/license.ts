/** Licence and about page. */

import { invoke } from "@tauri-apps/api/core";
import { card, make, showDone, showError, statusLine } from "../dom";
import type { AboutInfo, AboutLink } from "../dto";
import { tr } from "../i18n";

/**
 * Where this application comes from, and under what terms it may be used.
 *
 * The order of the sections and the addresses are decided in `pmca_app::about`
 * so this page and `pmca-console license` say the same thing; only the wording
 * lives in the catalogue, which is why the paragraphs are addressed by key and
 * follow a language change on their own.
 */
export function viewLicense(host: HTMLElement): void {
  const body = make("div");
  const status = statusLine();
  host.append(card(make("h2", { i18n: "nav-license" }), body, status));

  invoke<AboutInfo>("about_info")
    .then((about) => body.replaceChildren(...aboutNodes(about, status)))
    .catch((e) => showError(status, e));
}

/** Render the sections, the licence, the projects and the version. */
function aboutNodes(about: AboutInfo, status: HTMLElement): Node[] {
  const nodes: Node[] = [];

  for (const section of about.sections) {
    nodes.push(make("h3", { i18n: section.heading }));
    for (const key of section.body) nodes.push(make("p", { i18n: key }));
    if (section.people.length > 0) {
      nodes.push(make("p", { textContent: section.people.join(", ") }));
    }
  }

  nodes.push(make("h3", { i18n: "about-license-full" }));
  for (const line of about.copyright) {
    nodes.push(make("p", { className: "hint", textContent: line }));
  }
  // Reproduced verbatim and left untranslated: it is the licence being
  // claimed, and a rewording of it would be a different one.
  nodes.push(make("pre", { className: "license-text", textContent: about.licenseText }));

  nodes.push(make("h3", { i18n: "about-links-heading" }));
  for (const link of about.links) {
    nodes.push(linkRow(link, status));
  }

  // Several hundred entries, collapsed. Left expanded they would bury the
  // links and the version below them, and the reader who wants them is
  // looking for them; the reader who does not should not have to scroll past.
  nodes.push(make("h3", { i18n: "about-third-party-heading" }));
  nodes.push(make("p", { i18n: "about-third-party-body" }));
  nodes.push(
    make(
      "details",
      {},
      make("summary", { i18n: "about-third-party-toggle" }),
      // Untranslated for the same reason as the licence above: these are other
      // people's copyright notices, and restating them is not reproducing them.
      make("pre", { className: "license-text", textContent: about.thirdPartyText }),
    ),
  );

  nodes.push(
    make(
      "p",
      { className: "hint" },
      make("span", { i18n: "about-version" }),
      `: ${about.version} (${about.license})`,
    ),
  );
  return nodes;
}

/**
 * A project, its address and a way to take that address elsewhere.
 *
 * The address is text rather than a link on purpose. Opening a browser would
 * mean either letting the window navigate away from the application, or taking
 * on a plug-in with permission to launch programs — a lot of exposure for the
 * convenience of one click. Copying it needs neither.
 */
function linkRow(link: AboutLink, status: HTMLElement): HTMLElement {
  const copy = make("button", { i18n: "btn-copy-url" });
  copy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(link.url);
      showDone(status, tr("msg-url-copied"));
    } catch (e) {
      showError(status, e);
    }
  });
  return make(
    "div",
    { className: "link-row" },
    make("div", {}, make("div", { i18n: link.label }), make("code", { textContent: link.url })),
    copy,
  );
}
