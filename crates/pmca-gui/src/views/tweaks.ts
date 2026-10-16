/** Tweaks page. */

import { invoke } from "@tauri-apps/api/core";
import {
  RECOVERY_WHOLE_IMAGE,
  card,
  confirmed,
  fieldList,
  make,
  showBusy,
  showDone,
  showError,
  showStatus,
  statusLine,
  writing,
  type TweakWrite,
} from "../dom";
import type { TweakReport } from "../dto";
import { isCommandError, tr } from "../i18n";

export function viewTweaks(host: HTMLElement): void {
  const status = statusLine();
  const list = make("div", { className: "app-list" });
  // The camera's identifying values come out of the same backup image as the
  // tweaks. They are shown read-only: the toolkit knows no factory value to
  // undo a change with, so none of them is offered as something to edit.
  const identity = make("div", { className: "tweak-identity" });
  // Only tweaks the user actually toggled are sent, so an unchanged checkbox
  // never causes a write to the camera's backup memory.
  const pending = new Map<string, { enabled: boolean; acceptCaution: boolean }>();

  const apply = make("button", { className: "primary", i18n: "btn-apply" });
  apply.disabled = true;
  const undo = make("button", { i18n: "btn-undo-tweaks", hidden: true });
  const directHint = make("p", { className: "hint", i18n: "tweaks-direct-mode", hidden: true });

  const render = (report: TweakReport): void => {
    pending.clear();
    apply.disabled = true;
    undo.hidden = !report.canUndo;
    directHint.hidden = !report.direct;
    directHint.dataset.i18n = report.directForced ? "tweaks-direct-forced" : "tweaks-direct-mode";
    directHint.textContent = tr(directHint.dataset.i18n);

    identity.replaceChildren();
    if (report.properties.length > 0) {
      identity.append(
        make("h3", { i18n: "tweaks-identity-heading" }),
        fieldList(report.properties.map((p) => [p.labelKey, p.value])),
      );
    }

    list.replaceChildren();
    if (report.tweaks.length === 0) {
      list.append(make("p", { i18n: "msg-no-tweaks" }));
      return;
    }
    for (const tweak of report.tweaks) {
      // A switch is a checkbox in a label, so clicking the text toggles it.
      // A choice cannot be: one label holds one control, and a choice has
      // two. It therefore becomes a plain row with a pair of radios, each
      // carrying its own label.
      let box: HTMLInputElement | null = null;
      let radioOn: HTMLInputElement | null = null;
      let entry: HTMLElement;
      if (tweak.choice === null) {
        box = make("input", { type: "checkbox", checked: tweak.enabled });
        entry = make(
          "label",
          { className: "app-entry" },
          box,
          make("span", { i18n: tweak.labelKey }),
          make("span", { className: "price", textContent: tweak.value }),
        );
      } else {
        const group = `tweak-choice-${tweak.id}`;
        const radioOff = make("input", {
          type: "radio",
          name: group,
          checked: !tweak.enabled,
        });
        radioOn = make("input", { type: "radio", name: group, checked: tweak.enabled });
        entry = make(
          "div",
          { className: "app-entry" },
          make("span", { i18n: tweak.labelKey }),
          make(
            "span",
            { className: "tweak-choice" },
            make("label", {}, radioOff, make("span", { i18n: tweak.choice.offKey })),
            make("label", {}, radioOn, make("span", { i18n: tweak.choice.onKey })),
          ),
        );
      }
      // Written as a function because the answer changes with every click.
      const selected = (): boolean => box?.checked ?? radioOn?.checked ?? false;

      // Shown from the outset and never hidden. A note describes what the
      // camera will do afterwards regardless of which way the user decides,
      // so unfolding it on selection would reveal it after the point at which
      // it could still be acted on.
      const note =
        tweak.noteKey === null
          ? null
          : make("p", { className: "tweak-note", i18n: tweak.noteKey });

      // Some tweaks mean something different on this particular camera and
      // need the user's consent first. The warning lives next to the entry
      // rather than in a dialog on apply, so the decision is made while
      // looking at the thing it concerns.
      let acceptBox: HTMLInputElement | null = null;
      let caution: HTMLElement | null = null;
      if (tweak.cautionKey !== null) {
        acceptBox = make("input", { type: "checkbox" });
        caution = make(
          "div",
          { className: "tweak-caution", hidden: !tweak.enabled },
          make("p", { className: "warning", i18n: tweak.cautionKey }),
          make(
            "label",
            { className: "app-entry" },
            acceptBox,
            make("span", { i18n: "tweaks-caution-accept" }),
          ),
        );
      }

      const sync = (): void => {
        const accepted = acceptBox?.checked ?? false;
        // Accepting the caution is itself a request to write, even when the
        // tweak is already on: the camera then holds the reduced value and
        // the user has just asked for the full one.
        if (selected() !== tweak.enabled || (selected() && accepted)) {
          pending.set(tweak.id, { enabled: selected(), acceptCaution: accepted });
        } else {
          pending.delete(tweak.id);
        }
        apply.disabled = pending.size === 0;
      };

      const changed = (): void => {
        if (caution !== null && acceptBox !== null) {
          caution.hidden = !selected();
          // Switching the tweak off withdraws the consent with it; leaving it
          // ticked would silently re-apply on the next enable.
          if (!selected()) acceptBox.checked = false;
        }
        sync();
      };
      for (const control of entry.querySelectorAll("input")) {
        control.addEventListener("change", changed);
      }

      acceptBox?.addEventListener("change", async () => {
        if (
          acceptBox.checked &&
          !(await confirmed(tr("tweaks-caution-confirm"), tr("btn-confirm-languages")))
        ) {
          acceptBox.checked = false;
        }
        sync();
      });

      list.append(
        caution === null && note === null
          ? entry
          : make(
              "div",
              { className: "tweak-row" },
              entry,
              ...(note === null ? [] : [note]),
              ...(caution === null ? [] : [caution]),
            ),
      );
    }
  };

  const load = make("button", { i18n: "btn-list-tweaks" });
  load.addEventListener("click", async () => {
    load.disabled = true;
    showBusy(status, tr("tweaks-reading"));
    try {
      render(await invoke<TweakReport>("list_tweaks"));
      showStatus(status, "");
    } catch (e) {
      showError(status, e, () => load.click());
    } finally {
      load.disabled = false;
    }
  });

  apply.addEventListener("click", async () => {
    // Writing the backup memory is destructive, so it requires an explicit
    // confirmation in addition to the warning shown above the list.
    if (!(await confirmed(tr("tweaks-warning"), tr("btn-confirm-tweaks")))) return;
    const changes = [...pending].map(([id, change]) => ({ id, ...change }));
    await write("apply_tweaks", changes);
  });

  /** Write `changes` with the given command and show the outcome. */
  async function write(command: string, changes: TweakWrite[]): Promise<void> {
    apply.disabled = true;
    load.disabled = true;
    showBusy(status, tr("tweaks-writing"));
    try {
      const report = await invoke<TweakReport>(command, { changes });
      render(report);
      showDone(status, tr(report.direct ? "tweaks-direct-restart" : "tweaks-done"));
    } catch (e) {
      showError(status, e);
      if (isCommandError(e) && e.recovery === RECOVERY_WHOLE_IMAGE) offerWholeImage(changes);
    } finally {
      load.disabled = false;
    }
  }

  /**
   * Offer the whole-image write below a single-value refusal.
   *
   * Never taken on its own: the refusal changed nothing, and the whole image
   * is the riskier path, so it needs a click and a confirmation of its own.
   */
  function offerWholeImage(changes: TweakWrite[]): void {
    const button = make("button", { i18n: "btn-whole-image" });
    button.addEventListener("click", async () => {
      if (!(await confirmed(tr("tweaks-whole-image-confirm"), tr("btn-whole-image")))) return;
      await write("apply_tweaks_whole_image", changes);
    });
    status.after(make("div", { className: "recovery" }, writing(button)));
  }

  undo.addEventListener("click", async () => {
    if (!(await confirmed(tr("tweaks-undo-confirm"), tr("btn-undo-tweaks")))) return;
    undo.disabled = true;
    apply.disabled = true;
    load.disabled = true;
    showBusy(status, tr("tweaks-writing"));
    try {
      render(await invoke<TweakReport>("undo_tweaks"));
      showDone(status, tr("tweaks-undo-done"));
    } catch (e) {
      showError(status, e);
    } finally {
      undo.disabled = false;
      load.disabled = false;
    }
  });

  host.append(
    card(
      make("p", { className: "warning", i18n: "tweaks-warning" }),
      make("p", { className: "hint", i18n: "usb-mode-advice" }),
      make("div", { className: "row" }, load, writing(apply), writing(undo)),
      status,
      directHint,
      identity,
      list,
    ),
  );
}
