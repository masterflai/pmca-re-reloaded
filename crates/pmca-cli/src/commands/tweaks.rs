//! `tweaks`: list, enable, disable and set hidden camera settings.

use crate::*;

/// Split one `--set ID=VALUE` argument and resolve the value.
///
/// Only the splitting is the command line's business; what the value means
/// belongs to the tweak, so that part is asked of the toolkit.
pub(crate) fn parse_set(
    l: &Localizer,
    arg: &str,
) -> Result<(String, bool), Box<dyn std::error::Error>> {
    let (id, value) =
        arg.split_once('=').ok_or_else(|| l.tr_arg("tweaks-set-malformed", "arg", arg))?;
    Ok((id.to_string(), pmca_app::tweaks::resolve_choice(l, id, value)?))
}

/// How `tweaks` writes to the camera.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TweakWrite {
    /// The camera's default path.
    Default,
    /// Through the whole backup image (`--whole-image`).
    WholeImage,
    /// Write back the saved values (`--undo`).
    Undo,
}

/// List the connected camera's tweaks, or apply the requested changes.
///
/// Applying a tweak rewrites the camera's backup memory, so the destructive
/// path is gated behind an explicit `--confirm` after the warning is printed.
pub(crate) fn tweaks_command(
    l: &Localizer,
    enable: &[String],
    disable: &[String],
    set: &[String],
    confirm: bool,
    accept_caution: bool,
    write: TweakWrite,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);

    // A tweak that is a choice has no "enabled" state worth the name, so
    // asking for one is refused rather than guessed at.
    for id in enable.iter().chain(disable.iter()) {
        pmca_app::tweaks::reject_toggle_of_choice(l, id)?;
    }
    eprintln!("{}", l.tr("usb-mode-advice"));

    let mut changes: Vec<pmca_app::TweakChange> = enable
        .iter()
        .map(|id| (id.clone(), true))
        .chain(disable.iter().map(|id| (id.clone(), false)))
        .map(|(id, enabled)| pmca_app::TweakChange { id, enabled, accept_caution })
        .collect();
    for arg in set {
        let (id, enabled) = parse_set(l, arg)?;
        changes.push(pmca_app::TweakChange { id, enabled, accept_caution });
    }

    let report = if write == TweakWrite::Undo {
        if !confirm {
            return Err(l.tr("tweaks-confirm-required").into());
        }
        pmca_app::tweaks::undo_tweaks(l, &mut sink)?
    } else if changes.is_empty() {
        pmca_app::tweaks::read_tweaks(l, &mut sink)?
    } else {
        eprintln!("{}", l.tr("tweaks-warning"));
        if !confirm {
            return Err(l.tr("tweaks-confirm-required").into());
        }
        let result = if write == TweakWrite::WholeImage {
            pmca_app::tweaks::apply_tweaks_whole_image(l, &changes, &mut sink)
        } else {
            pmca_app::tweaks::apply_tweaks(l, &changes, &mut sink)
        };
        match result {
            Ok(report) => report,
            Err(e) => {
                // The refusal leaves every value as it was, so pointing at the
                // other path is safe; taking it stays the user's decision.
                // The message is already localized, so it can carry the hint.
                if e.recovery() == Some(pmca_app::RECOVERY_WHOLE_IMAGE) {
                    return Err(format!("{e}\n{}", l.tr("tweaks-whole-image-hint")).into());
                }
                return Err(e.into());
            }
        }
    };

    if report.direct_forced {
        println!("{}", l.tr("tweaks-direct-forced"));
    } else if report.direct {
        println!("{}", l.tr("tweaks-direct-mode"));
    }
    if report.can_undo {
        println!("{} (--undo --confirm)", l.tr("tweaks-undo-available"));
    }

    // The camera's identifying values come from the same backup image. They
    // are read-only: no tweak can change them, so they are printed as plain
    // fields rather than as entries of the toggleable list below.
    for property in &report.properties {
        println!("{}: {}", l.tr(&property.label_key), property.value);
    }

    if report.tweaks.is_empty() {
        println!("{}", l.tr("msg-no-tweaks"));
        return Ok(());
    }
    for state in report.tweaks {
        // A choice names its current state; a switch says whether it is on.
        // Printing "disabled" next to a video system would claim the camera
        // has none.
        let flag = match &state.choice {
            Some(c) => l.tr(if state.enabled { &c.on_key } else { &c.off_key }),
            None if state.enabled => l.tr("tweaks-state-enabled"),
            None => l.tr("tweaks-state-disabled"),
        };
        // For a choice the bracket already holds the value, so repeating it in
        // the trailing column would only say the same thing twice.
        let label = l.tr(&state.label_key);
        match state.choice {
            Some(_) => println!("{:<18} [{}] {}", state.id, flag, label),
            None => println!("{:<18} [{}] {} ({})", state.id, flag, label, state.value),
        }
        // Before the caution, and unconditionally: a note is not something to
        // agree to, it is something to have read beforehand. Printing it only
        // when the tweak is being changed would put it after the moment it is
        // useful — the memory card has to come out before the write, not after.
        if let Some(key) = &state.note_key {
            println!("{:<18}   {}", "", l.tr(key));
        }
        // Printed whether or not the user asked for the tweak: without it,
        // someone who enabled all languages and later counts them has no way
        // of knowing why two are missing.
        if let Some(key) = &state.caution_key {
            println!("{:<18}   {}", "", l.tr(key));
            if !accept_caution {
                println!("{:<18}   --accept-caution: {}", "", l.tr("tweaks-caution-accept"));
            }
        }
    }
    Ok(())
}
