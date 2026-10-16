//! `analyze`: the function analysis, its prompts and the cross-check.

use crate::*;

/// The tag in front of every step, so reading and writing are told apart at a
/// glance.
pub(crate) fn access_tag(l: &Localizer, access: pmca_app::analysis::Access) -> String {
    use pmca_app::analysis::Access;
    let key = match access {
        Access::Read => "analysis-access-read",
        Access::Write => "analysis-access-write",
        Access::User => "analysis-access-user",
        Access::Local => "analysis-access-local",
    };
    format!("[{}]", l.tr(key))
}

/// Print instructions framed, so they stand out from the progress lines.
///
/// Catalogue entries are single lines, so numbered steps ("1. … 2. …") are put
/// on lines of their own here.
pub(crate) fn print_box(text: &str) {
    let rule = "=".repeat(72);
    println!("\n{rule}");
    for line in numbered_lines(text).lines() {
        println!("  {line}");
    }
    println!("{rule}");
}

/// `text` with a line break before every ` N. ` that starts a numbered step.
pub(crate) fn numbered_lines(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find(' ') {
        let after = &rest[pos + 1..];
        let digits = after.chars().take_while(char::is_ascii_digit).count();
        out.push_str(&rest[..pos]);
        out.push(if digits > 0 && after[digits..].starts_with(". ") { '\n' } else { ' ' });
        rest = after;
    }
    out.push_str(rest);
    out
}

pub(crate) fn prompt(text: &str) {
    print!("{text} ");
    let _ = std::io::stdout().flush();
}

/// Wait for Enter; `false` when the user typed `q` or input ended.
pub(crate) fn wait_enter(l: &Localizer) -> bool {
    prompt(&l.tr("analysis-cli-continue"));
    read_stdin_line().is_some_and(|line| !line.trim().eq_ignore_ascii_case("q"))
}

/// Ask for the confirmation word. Nothing else counts as yes; the English
/// word is accepted too, for scripts whose word is awkward to type.
pub(crate) fn confirm_word(l: &Localizer) -> bool {
    let word = l.tr("analysis-confirm-word");
    prompt(&l.tr_arg("analysis-cli-confirm", "word", &word));
    read_stdin_line().is_some_and(|line| {
        let typed = line.trim().to_lowercase();
        typed == word.to_lowercase() || typed == "yes"
    })
}

/// Ask which menu value the camera shows.
pub(crate) fn ask_menu(l: &Localizer, values: &[String]) -> String {
    println!("{}", l.tr("analysis-menu-question"));
    for (i, value) in values.iter().enumerate() {
        println!("  {}  {value}", i + 1);
    }
    println!("  0  {}", l.tr("analysis-menu-other"));
    loop {
        prompt(">");
        let Some(line) = read_stdin_line() else { return String::new() };
        match line.trim().parse::<usize>() {
            Ok(0) => {
                prompt(&format!("{}:", l.tr("analysis-menu-other")));
                return read_stdin_line().unwrap_or_default().trim().to_string();
            }
            Ok(n) if n <= values.len() => return values[n - 1].clone(),
            _ => {}
        }
    }
}

/// Find the backup value behind a menu setting, interactively.
///
/// The report is saved whatever happens, so an aborted run still says how far
/// it got.
pub(crate) fn analyze_command(
    l: &Localizer,
    setting: &str,
    values: &[String],
    write_test: bool,
    out: Option<&std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut run = pmca_app::analysis::AnalysisRun::new(l, setting, values, write_test)?;

    println!("{}", l.tr("analysis-title"));
    println!("{}", l.tr("analysis-experimental"));
    println!("{}", l.tr("analysis-purpose"));
    println!("{}", l.tr("analysis-own-hardware"));
    if pmca_app::analysis::has_pending_restore() {
        eprintln!("{} (pmca-console analyze --restore)", l.tr("analysis-pending"));
    }
    println!();
    for step in run.plan(l) {
        println!("  {:<14} {}", access_tag(l, step.access), step.text);
    }
    if write_test {
        println!();
        eprintln!("{}", l.tr("analysis-write-warning"));
        if !confirm_word(l) {
            return Err(l.tr("analysis-write-declined").into());
        }
    }
    eprintln!("{}", l.tr("usb-mode-advice"));

    let result =
        analyze_steps(l, &mut run, setting).and_then(|()| analyze_cross_checks(l, &mut run));

    let saved = run.save_report();
    if let Some(out) = out {
        std::fs::write(out, run.report_json()?)?;
        println!("{}", l.tr_arg("analysis-report-saved", "path", &out.display().to_string()));
    }
    match saved {
        Ok(path) => {
            println!("{}", l.tr_arg("analysis-report-saved", "path", &path.display().to_string()))
        }
        Err(e) => eprintln!("{}: {}", l.tr("cli-error-prefix"), error_text(&e, l)),
    }
    println!("{}", l.tr("analysis-report-hint"));
    if run.needs_restore() {
        eprintln!("{} (pmca-console analyze --restore)", l.tr("analysis-pending"));
    }
    result
}

/// The camera part of [`analyze_command`]: snapshots, evaluation, write test.
pub(crate) fn analyze_steps(
    l: &Localizer,
    run: &mut pmca_app::analysis::AnalysisRun,
    setting: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use pmca_app::analysis::Access;
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);

    while let Some(label) = run.next_label() {
        let key = if run.taken() == 0 { "analysis-do-prepare" } else { "analysis-do-change" };
        print_box(&l.tr_args(key, &[("setting", setting), ("value", &label)]));
        if !wait_enter(l) {
            println!("{}", l.tr("analysis-aborted"));
            return Ok(());
        }
        let (index, total) = (run.taken() + 1, run.labels().len());
        println!(
            "{} {}",
            access_tag(l, Access::Read),
            l.tr_args(
                "analysis-reading",
                &[("index", &index.to_string()), ("total", &total.to_string())]
            )
        );
        run.take_snapshot(l, &mut sink).map_err(|e| app_driver_hint(l, e))?;
    }

    let eval = run.evaluate(l);
    println!();
    println!(
        "{}",
        l.tr_args(
            "analysis-result",
            &[
                ("compared", &eval.compared.to_string()),
                ("count", &eval.candidates.len().to_string()),
                ("running", &eval.running.to_string()),
            ]
        )
    );
    for c in &eval.candidates {
        let fit = l.tr(if c.consistent {
            "analysis-candidate-consistent"
        } else {
            "analysis-candidate-inconsistent"
        });
        print_analysis_values(c, &fit);
    }
    if eval.candidates.is_empty() && !eval.changed.is_empty() {
        println!("{}", l.tr("analysis-changed-hint"));
        for c in &eval.changed {
            print_analysis_values(c, "");
        }
    } else if eval.running > 0 {
        println!("{}", l.tr("analysis-changed-info"));
    }

    if !run.write_requested() {
        return Ok(());
    }
    let Some(preview) = eval.write else {
        println!("{}", eval.write_refusal.unwrap_or_default());
        return Ok(());
    };
    print_box(&format!(
        "{} {}\n{}",
        access_tag(l, Access::Write),
        l.tr_args(
            "analysis-write-preview",
            &[
                ("id", &preview.id),
                ("size", &preview.size.to_string()),
                ("old", &preview.old_hex),
                ("old-label", &preview.old_label),
                ("new", &preview.new_hex),
                ("new-label", &preview.new_label),
            ]
        ),
        l.tr("analysis-own-hardware")
    ));
    if !confirm_word(l) {
        println!("{}", l.tr("analysis-write-declined"));
        return Ok(());
    }
    print_box(&l.tr("analysis-do-reconnect"));
    if !wait_enter(l) {
        println!("{}", l.tr("analysis-aborted"));
        return Ok(());
    }
    println!("{} {}", access_tag(l, Access::Write), l.tr("tweaks-writing"));
    run.write(l, &mut sink).map_err(|e| app_driver_hint(l, e))?;

    let values = run.labels()[..run.labels().len() - 1].to_vec();
    print_box(&l.tr_args("analysis-do-check", &[("setting", setting)]));
    let seen = ask_menu(l, &values);
    run.record_menu(&seen);

    print_box(&l.tr("analysis-do-reconnect"));
    // No way out here: the written value has to go back.
    let _ = wait_enter(l);
    println!("{} {}", access_tag(l, Access::Write), l.tr("tweaks-writing"));
    let survived = run.restore(l, &mut sink).map_err(|e| app_driver_hint(l, e))?;
    println!("{}", l.tr(if survived { "analysis-survived" } else { "analysis-not-survived" }));

    print_box(&l.tr_args("analysis-do-check", &[("setting", setting)]));
    let seen = ask_menu(l, &values);
    run.record_menu(&seen);
    Ok(())
}

/// Offer to read the candidates from other cameras, one after another.
pub(crate) fn analyze_cross_checks(
    l: &Localizer,
    run: &mut pmca_app::analysis::AnalysisRun,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
    while run.can_cross_check() {
        print_box(&format!(
            "{} {}\n{}\n{}",
            access_tag(l, pmca_app::analysis::Access::Read),
            l.tr("analysis-cross-heading"),
            l.tr("analysis-cross-intro"),
            l.tr("analysis-cross-do")
        ));
        if !wait_enter(l) {
            break;
        }
        // A failed check leaves the run as it was; the next camera may work.
        match run.cross_check(l, &mut sink) {
            Ok(check) => print_cross_check(l, &check),
            Err(e) => {
                let e = app_driver_hint(l, e);
                eprintln!("{}: {}", l.tr("cli-error-prefix"), error_text(e.as_ref(), l));
            }
        }
    }
    Ok(())
}

/// One other camera and what it holds for each candidate.
pub(crate) fn print_cross_check(l: &Localizer, check: &pmca_app::analysis::CrossCheckView) {
    let c = &check.camera;
    let firmware = c.firmware.as_deref().map(|f| format!("FW {f}")).unwrap_or_default();
    println!("  {} {firmware} {}", c.model_name.as_deref().unwrap_or("?"), c.region);
    for v in &check.values {
        let text = if v.present {
            let value = v.hex.clone().unwrap_or_else(|| format!("{} B", v.size.unwrap_or(0)));
            let fit = match &v.matches {
                Some(label) => l.tr_arg("analysis-cross-match", "label", label),
                None => l.tr("analysis-cross-nomatch"),
            };
            format!("{} · {fit}", l.tr_arg("analysis-cross-present", "value", &value))
        } else {
            l.tr("analysis-cross-absent")
        };
        println!("      {:<12} {text}", v.id);
    }
}

/// One property with its value at every snapshot.
pub(crate) fn print_analysis_values(c: &pmca_app::analysis::CandidateView, note: &str) {
    println!("  {}  {} B  attr {} ({})  {note}", c.id, c.size, c.attr, c.attr_names.join(", "));
    for v in &c.values {
        let shown = match &v.hex {
            Some(hex) => hex.clone(),
            None => format!("{:?}", v.changed_offsets),
        };
        println!("      {:<24} {shown}", v.label);
    }
}

/// Write back what an interrupted write test left on the camera.
pub(crate) fn analyze_restore_command(l: &Localizer) -> Result<(), Box<dyn std::error::Error>> {
    let mut sink = |event: pmca_app::ProgressEvent| println!("{}", event.message);
    eprintln!("{}", l.tr("usb-mode-advice"));
    println!("{} {}", access_tag(l, pmca_app::analysis::Access::Write), l.tr("tweaks-writing"));
    pmca_app::analysis::restore_pending(l, &mut sink).map_err(|e| app_driver_hint(l, e))?;
    Ok(())
}
