//! File-only Sony WBI1 image inspection, extraction, and RAM-section diff.
//!
//! These commands never contact a camera. WBI1 images may contain user data,
//! credentials, and process memory: do not share their output files publicly.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use pmca_core::wbi::{self, ChangeKind, Section};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::WbiAction;

fn load(path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    const MAX_IMAGE: u64 = 512 * 1024 * 1024;
    if fs::metadata(path)?.len() > MAX_IMAGE {
        return Err("WBI image exceeds 512 MiB read budget".into());
    }
    Ok(fs::read(path)?)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn section_info(data: &[u8], section: &Section) -> Result<Value, Box<dyn std::error::Error>> {
    let digest = wbi::digest_section(data, section)?;
    Ok(json!({
        "index": section.index,
        "physical_address": format!("0x{:08x}", section.physical_address),
        "virtual_address": format!("0x{:08x}", section.virtual_address),
        "compressed_offset": section.compressed_offset,
        "compressed_size": section.compressed_size,
        "expanded_size": section.expanded_size,
        "flags": section.flags,
        "checksum_unverified": format!("0x{:08x}", section.checksum),
        "metadata_checksum_unverified": format!("0x{:08x}", section.metadata_checksum),
        "sha256_expanded": hex(&digest),
    }))
}

fn inspect(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let data = load(path)?;
    let parsed = wbi::parse(&data)?;
    let sections =
        parsed.sections.iter().map(|s| section_info(&data, s)).collect::<Result<Vec<_>, _>>()?;
    Ok(json!({
        "format": "WBI1",
        "source": path.display().to_string(),
        "sha256_source": hex(&Sha256::digest(&data)),
        "image_bytes": data.len(),
        "created_at": Value::Null, // WBI header has no known creation timestamp
        "captured_at": Value::Null, // Export metadata is external to the image
        "header": {
            "section_count": parsed.header.sections,
            "flags": parsed.header.flags,
            "resume_vector": format!("0x{:08x}", parsed.header.resume_vector),
            "version": format!("0x{:08x}", parsed.header.version),
            "sector_bytes": parsed.header.sector_size,
            "compressed_bytes": parsed.header.compressed_size,
            "expanded_bytes": parsed.header.expanded_size,
            "kernel_start": format!("0x{:08x}", parsed.header.kernel_start),
            "kernel_size": parsed.header.kernel_size,
            "kernel_checksum_unverified": format!("0x{:08x}", parsed.header.kernel_checksum),
        },
        "sections": sections,
    }))
}

fn unpack(path: &Path, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let data = load(path)?;
    let parsed = wbi::parse(&data)?;
    // Decode and verify every section before creating any output files.
    for section in &parsed.sections {
        wbi::decode_section(&data, section)?;
    }
    fs::create_dir(dir)?; // refuse to overwrite an existing directory
    for section in &parsed.sections {
        let name = format!(
            "section-{:03}-phys-{:08x}-virt-{:08x}.bin",
            section.index + 1,
            section.physical_address,
            section.virtual_address
        );
        let path = dir.join(name);
        let mut file = File::options().write(true).create_new(true).open(path)?;
        file.write_all(&wbi::decode_section(&data, section)?)?;
        file.sync_all()?;
    }
    Ok(())
}

fn compare(before: &Path, after: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let old = load(before)?;
    let new = load(after)?;
    let changes = wbi::diff(&old, &new)?;
    let old_header = wbi::parse(&old)?.header;
    let new_header = wbi::parse(&new)?.header;
    Ok(json!({
        "format": "WBI1-section-diff",
        "header_changed": old_header != new_header,
        "before": before.display().to_string(),
        "after": after.display().to_string(),
        "sha256_before": hex(&Sha256::digest(&old)),
        "sha256_after": hex(&Sha256::digest(&new)),
        "changes": changes.iter().map(|c| json!({
            "physical_address": format!("0x{:08x}", c.physical_address),
            "kind": match c.kind {
                ChangeKind::Added => "added",
                ChangeKind::Removed => "removed",
                ChangeKind::Modified => "modified",
            },
            "old_size": c.before.map(|s| s.expanded_size),
            "new_size": c.after.map(|s| s.expanded_size),
        })).collect::<Vec<_>>(),
    }))
}

/// Run one read-only WBI operation on local files.
pub(crate) fn wbi_command(action: &WbiAction) -> Result<(), Box<dyn std::error::Error>> {
    let (report, outfile): (Option<Value>, Option<&PathBuf>) = match action {
        WbiAction::Inspect { file, json, out } => {
            let report = inspect(file)?;
            if !json && out.is_none() {
                println!(
                    "WBI1: {} sections, {} expanded bytes",
                    report["header"]["section_count"], report["header"]["expanded_bytes"]
                );
                println!(
                    "Source SHA-256: {}",
                    report["sha256_source"].as_str().unwrap_or("unknown")
                );
                println!("Snapshot creation time: unknown (not present in header)");
                (None, None)
            } else {
                (Some(report), out.as_ref())
            }
        }
        WbiAction::Unpack { file, out } => {
            unpack(file, out)?;
            println!("Extracted WBI memory sections into {}", out.display());
            (None, None)
        }
        WbiAction::Diff { before, after, json, out } => {
            let report = compare(before, after)?;
            if !json && out.is_none() {
                let changes = report["changes"].as_array().ok_or("missing WBI changes array")?;
                println!("WBI1: {} changed physical memory sections", changes.len());
                for c in changes {
                    println!("{}: {}", c["physical_address"], c["kind"]);
                }
                (None, None)
            } else {
                (Some(report), out.as_ref())
            }
        }
    };
    if let Some(report) = report {
        let rendered = serde_json::to_string_pretty(&report)?;
        if let Some(path) = outfile {
            let mut f = File::options().write(true).create_new(true).open(path)?;
            f.write_all(rendered.as_bytes())?;
            f.write_all(b"\n")?;
            f.sync_all()?;
        } else {
            println!("{rendered}");
        }
    }
    Ok(())
}
