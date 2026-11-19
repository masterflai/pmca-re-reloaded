//! Sony `WBI1` warm-boot images on disk: inspect, unpack, compare.
//!
//! A WBI1 image is a snapshot of the camera's memory, so it can hold Wi-Fi
//! keys, passwords and whatever else a running process had in RAM. Reports
//! therefore describe sections by address, size and digest, never by content;
//! only `unpack` writes the content, into a folder the user names. Nothing
//! here touches a camera.

use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use pmca_core::wbi::{self, ChangeKind, Section};
use pmca_i18n::Localizer;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{Error, Result};

/// The largest image read; a WBI1 file holds at most a camera's RAM.
const MAX_IMAGE_MIB: u64 = 512;

/// What a WBI1 image holds, section by section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WbiReport {
    /// The file that was read.
    pub source: String,
    /// SHA-256 of the whole file.
    pub sha256: String,
    /// Size of the file.
    pub image_bytes: usize,
    /// The fixed header.
    pub header: WbiHeader,
    /// The memory sections, in table order.
    pub sections: Vec<WbiSection>,
}

/// The header fields of a WBI1 image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WbiHeader {
    /// Number of sections in the table.
    pub section_count: u32,
    /// Image flags; bit 0 means LZ77-compressed.
    pub flags: u32,
    /// ARM resume address; not a time stamp.
    pub resume_vector: String,
    /// Format version.
    pub version: String,
    /// Sector size of the data region.
    pub sector_bytes: u32,
    /// Length of the compressed data region.
    pub compressed_bytes: u32,
    /// Total size of all sections once unpacked.
    pub expanded_bytes: u32,
    /// Kernel start address.
    pub kernel_start: String,
    /// Kernel length.
    pub kernel_size: u32,
    /// The header's kernel checksum; its algorithm is not known, so it is not checked.
    pub kernel_checksum_unverified: String,
}

/// One memory section of a WBI1 image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WbiSection {
    /// Position in the table, from zero.
    pub index: usize,
    /// Physical address the section is restored to.
    pub physical_address: String,
    /// Virtual address the captured kernel used.
    pub virtual_address: String,
    /// Offset of the compressed stream in the file.
    pub compressed_offset: usize,
    /// Length of the compressed stream.
    pub compressed_size: usize,
    /// Length once unpacked.
    pub expanded_size: usize,
    /// Section flags.
    pub flags: u32,
    /// The table's section checksum; not checked, its algorithm is not known.
    pub checksum_unverified: String,
    /// The table's metadata checksum; not checked either.
    pub metadata_checksum_unverified: String,
    /// SHA-256 of the unpacked section.
    pub sha256: String,
}

/// The sections two WBI1 images do not share.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WbiDiff {
    /// The earlier file.
    pub before: String,
    /// The later file.
    pub after: String,
    /// SHA-256 of the earlier file.
    pub sha256_before: String,
    /// SHA-256 of the later file.
    pub sha256_after: String,
    /// Whether the headers differ.
    pub header_changed: bool,
    /// The sections that were added, removed or changed, by physical address.
    pub changes: Vec<WbiChange>,
}

/// One physical address whose section differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WbiChange {
    /// Physical start address.
    pub physical_address: String,
    /// `added`, `removed` or `modified`.
    pub kind: String,
    /// Unpacked size before, if the section existed.
    pub old_size: Option<usize>,
    /// Unpacked size after, if the section exists.
    pub new_size: Option<usize>,
}

/// Read and describe a WBI1 image.
pub fn inspect(l: &Localizer, path: &Path) -> Result<WbiReport> {
    let data = load(l, path)?;
    let image = wbi::parse(&data).map_err(|e| invalid(l, &e))?;
    let sections =
        image.sections.iter().map(|s| section_view(l, &data, s)).collect::<Result<Vec<_>>>()?;
    let h = image.header;
    Ok(WbiReport {
        source: path.display().to_string(),
        sha256: hex(&Sha256::digest(&data)),
        image_bytes: data.len(),
        header: WbiHeader {
            section_count: h.sections,
            flags: h.flags,
            resume_vector: format!("{:#010x}", h.resume_vector),
            version: format!("{:#010x}", h.version),
            sector_bytes: h.sector_size,
            compressed_bytes: h.compressed_size,
            expanded_bytes: h.expanded_size,
            kernel_start: format!("{:#010x}", h.kernel_start),
            kernel_size: h.kernel_size,
            kernel_checksum_unverified: format!("{:#010x}", h.kernel_checksum),
        },
        sections,
    })
}

/// Write every section of a WBI1 image into `dir`, which must not exist yet;
/// the number of sections written.
///
/// Every section is unpacked once before the folder is created, so a damaged
/// image leaves nothing behind.
pub fn unpack(l: &Localizer, path: &Path, dir: &Path) -> Result<usize> {
    let data = load(l, path)?;
    let image = wbi::parse(&data).map_err(|e| invalid(l, &e))?;
    for section in &image.sections {
        wbi::decode_section(&data, section).map_err(|e| invalid(l, &e))?;
    }
    fs::create_dir(dir)?;
    for section in &image.sections {
        let name = format!(
            "section-{:03}-phys-{:08x}-virt-{:08x}.bin",
            section.index + 1,
            section.physical_address,
            section.virtual_address
        );
        let mut file = File::options().write(true).create_new(true).open(dir.join(name))?;
        file.write_all(&wbi::decode_section(&data, section).map_err(|e| invalid(l, &e))?)?;
        file.sync_all()?;
    }
    Ok(image.sections.len())
}

/// Compare two WBI1 images section by section.
pub fn diff(l: &Localizer, before: &Path, after: &Path) -> Result<WbiDiff> {
    let old = load(l, before)?;
    let new = load(l, after)?;
    let changes = wbi::diff(&old, &new).map_err(|e| invalid(l, &e))?;
    let old_header = wbi::parse(&old).map_err(|e| invalid(l, &e))?.header;
    let new_header = wbi::parse(&new).map_err(|e| invalid(l, &e))?.header;
    Ok(WbiDiff {
        before: before.display().to_string(),
        after: after.display().to_string(),
        sha256_before: hex(&Sha256::digest(&old)),
        sha256_after: hex(&Sha256::digest(&new)),
        header_changed: old_header != new_header,
        changes: changes
            .iter()
            .map(|c| WbiChange {
                physical_address: format!("{:#010x}", c.physical_address),
                kind: kind_name(c.kind).to_string(),
                old_size: c.before.map(|s| s.expanded_size),
                new_size: c.after.map(|s| s.expanded_size),
            })
            .collect(),
    })
}

/// The report as text, one line per section.
#[must_use]
pub fn render_report(l: &Localizer, report: &WbiReport) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{}",
        l.tr_args(
            "wbi-summary",
            &[
                ("sections", &report.header.section_count.to_string()),
                ("bytes", &report.header.expanded_bytes.to_string()),
            ],
        )
    );
    let _ = writeln!(out, "{}", l.tr_arg("wbi-source-sha", "sha", &report.sha256));
    let _ = writeln!(out, "{}", l.tr("wbi-no-time"));
    for s in &report.sections {
        let _ = writeln!(
            out,
            "  {:>3}  phys {}  virt {}  {:>10}  {}",
            s.index + 1,
            s.physical_address,
            s.virtual_address,
            s.expanded_size,
            s.sha256
        );
    }
    out
}

/// The comparison as text, one line per differing section.
#[must_use]
pub fn render_diff(l: &Localizer, diff: &WbiDiff) -> String {
    let mut out = String::new();
    let _ =
        writeln!(out, "{}", l.tr_arg("wbi-diff-summary", "count", &diff.changes.len().to_string()));
    if diff.header_changed {
        let _ = writeln!(out, "{}", l.tr("wbi-header-changed"));
    }
    for c in &diff.changes {
        let _ =
            writeln!(out, "  {}  {}", c.physical_address, l.tr(&format!("wbi-change-{}", c.kind)));
    }
    out
}

fn section_view(l: &Localizer, data: &[u8], s: &Section) -> Result<WbiSection> {
    let digest = wbi::digest_section(data, s).map_err(|e| invalid(l, &e))?;
    Ok(WbiSection {
        index: s.index,
        physical_address: format!("{:#010x}", s.physical_address),
        virtual_address: format!("{:#010x}", s.virtual_address),
        compressed_offset: s.compressed_offset,
        compressed_size: s.compressed_size,
        expanded_size: s.expanded_size,
        flags: s.flags,
        checksum_unverified: format!("{:#010x}", s.checksum),
        metadata_checksum_unverified: format!("{:#010x}", s.metadata_checksum),
        sha256: hex(&digest),
    })
}

fn kind_name(kind: ChangeKind) -> &'static str {
    match kind {
        ChangeKind::Added => "added",
        ChangeKind::Removed => "removed",
        ChangeKind::Modified => "modified",
    }
}

fn load(l: &Localizer, path: &Path) -> Result<Vec<u8>> {
    if fs::metadata(path)?.len() > MAX_IMAGE_MIB * 1024 * 1024 {
        return Err(Error::Operation(l.tr_arg("wbi-too-large", "max", &MAX_IMAGE_MIB.to_string())));
    }
    Ok(fs::read(path)?)
}

fn invalid(l: &Localizer, e: &pmca_core::Error) -> Error {
    Error::Operation(format!("{}: {e}", l.tr("wbi-invalid")))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
