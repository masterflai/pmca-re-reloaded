//! Read-only reader for what a Sony camera records about itself in a photo.
//!
//! The shutter count is not something the camera's USB protocol reports. What
//! the camera does do is write the current count into every image it saves, in
//! the Exif maker note under tag `0x9050`. That block is scrambled with a
//! byte-substitution cipher (`c = b³ mod 249` for `b < 249`, other bytes left
//! alone), and the position of the count inside it depends on the model.
//!
//! The layouts below follow the public research collected in ExifTool's Sony
//! module (`Tag9050a` to `Tag9050d`). They are not verified on hardware for every
//! model, which is why a model outside the known groups yields
//! [`ShutterCount::UnknownModel`] instead of a number read from a guessed
//! offset: a wrong count is worse than none.
//!
//! The container (JPEG, TIFF-based raw such as ARW, HEIF) is unpacked by the
//! `kamadak-exif` crate; walking Sony's maker note is done here, with every
//! offset bounds-checked before use.

use std::io::Cursor;

use exif::{In, Tag, Value};

use crate::{Error, Result};

/// Maker-note tag holding the enciphered block with the shutter count.
const TAG_9050: u16 = 0x9050;

/// Upper bound on maker-note entries; real files carry well under a hundred.
const MAX_NOTE_ENTRIES: usize = 512;

/// Headers that precede the maker-note IFD in JPEG and HEIF files. Raw files
/// start the IFD directly.
const NOTE_HEADERS: [&[u8]; 3] = [b"SONY DSC \0\0\0", b"SONY CAM \0\0\0", b"SONY MOBILE\0"];

/// Length shared by all of [`NOTE_HEADERS`].
const NOTE_HEADER_LEN: usize = 12;

/// The result of looking for the shutter count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutterCount {
    /// The count stored in the image.
    Counted(u32),
    /// The image was not taken by a Sony camera.
    NotSony,
    /// No layout is known for this model, so no offset is trusted.
    UnknownModel,
    /// The model is known, but this file carries no count — an image taken
    /// with the electronic shutter on some bodies, or one whose maker note was
    /// dropped by editing software.
    NotRecorded,
}

/// Which shutter took the image, where the file says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shutter {
    /// The mechanical shutter; this is what the count counts.
    Mechanical,
    /// The electronic (silent) shutter, which does not advance the count.
    Electronic,
}

/// Details read from one image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhotoInfo {
    /// Manufacturer, as written by the camera.
    pub make: Option<String>,
    /// Camera model (e.g. `ILCE-5100`).
    pub model: Option<String>,
    /// Firmware that wrote the file (e.g. `ILCE-5100 v3.10`).
    pub software: Option<String>,
    /// Lens model, if the lens reported one.
    pub lens: Option<String>,
    /// Capture time as `YYYY-MM-DD hh:mm:ss`, with the UTC offset appended
    /// when the camera recorded one.
    pub taken: Option<String>,
    /// The shutter count, or why there is none.
    pub shutter_count: ShutterCount,
    /// Shutter type, if the layout records it.
    pub shutter: Option<Shutter>,
}

/// Where the count sits inside the deciphered `0x9050` block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// SLT, NEX and ILCE bodies up to mid-2015 (e.g. ILCE-5100, ILCE-7M2).
    A,
    /// ILCE-6100–6600, 7M3, 7RM2–7RM4, 7SM2, 9, 9M2, ILCA-99M2, ZV-E10.
    B,
    /// ILCE-1, 7M4, 7RM5, 7SM3, ILME-FX3.
    C,
    /// ILCE-6700, 7CM2, 7CR, ILME-FX2, and newer bodies with the mechanical
    /// shutter; the count only advances with the mechanical shutter.
    D,
}

impl Layout {
    /// Offset of the 24-bit count.
    fn count_offset(self) -> usize {
        match self {
            Layout::A => 0x0032,
            Layout::B | Layout::C => 0x003a,
            Layout::D => 0x000a,
        }
    }

    /// Offset of the three 16-bit shutter words, where the layout has them.
    fn shutter_offset(self) -> Option<usize> {
        match self {
            Layout::A => Some(0x0020),
            Layout::B | Layout::C => Some(0x0026),
            Layout::D => None,
        }
    }
}

/// Read the details of the image in `data`.
///
/// Fails only when the file is not an image this reader understands or has no
/// Exif data at all. A Sony image without a usable count still succeeds; the
/// reason is reported in [`PhotoInfo::shutter_count`].
pub fn read(data: &[u8]) -> Result<PhotoInfo> {
    let mut reader = exif::Reader::new();
    // Raw files carry private IFDs that do not always parse cleanly; the
    // fields needed here are readable regardless.
    reader.continue_on_error(true);
    let exif = reader
        .read_from_container(&mut Cursor::new(data))
        .or_else(|e| e.distill_partial_result(|_| {}))
        .map_err(exif_error)?;
    Ok(from_exif(&exif))
}

/// Map a container or Exif failure onto the crate's error type.
fn exif_error(e: exif::Error) -> Error {
    match e {
        exif::Error::InvalidFormat(s)
        | exif::Error::NotFound(s)
        | exif::Error::BlankValue(s)
        | exif::Error::TooBig(s)
        | exif::Error::NotSupported(s)
        | exif::Error::UnexpectedValue(s) => Error::Invalid(s),
        exif::Error::Io(_) => Error::OutOfBounds { context: "image file" },
        _ => Error::Invalid("unreadable Exif data"),
    }
}

/// Collect the details from parsed Exif data.
fn from_exif(exif: &exif::Exif) -> PhotoInfo {
    let text = |tag| exif.get_field(tag, In::PRIMARY).and_then(|f| ascii(&f.value));
    let make = text(Tag::Make);
    let model = text(Tag::Model);
    let taken = text(Tag::DateTimeOriginal)
        .or_else(|| text(Tag::DateTime))
        .map(|t| format_time(&t, text(Tag::OffsetTimeOriginal).as_deref()));
    // Sony writes "----" for a lens that does not identify itself.
    let lens = text(Tag::LensModel).filter(|l| !l.trim_matches('-').is_empty());

    let sony = make.as_deref().is_some_and(|m| m.to_ascii_uppercase().starts_with("SONY"));
    let (shutter_count, shutter) = if sony {
        let block = maker_note(exif)
            .and_then(|(offset, len)| {
                note_entry(exif.buf(), exif.little_endian(), offset, len, TAG_9050)
            })
            .map(decipher);
        count(model.as_deref().unwrap_or(""), block.as_deref())
    } else {
        (ShutterCount::NotSony, None)
    };

    PhotoInfo { make, model, software: text(Tag::Software), lens, taken, shutter_count, shutter }
}

/// Read the count and shutter type out of a deciphered `0x9050` block.
fn count(model: &str, block: Option<&[u8]>) -> (ShutterCount, Option<Shutter>) {
    let usable = |l: &Layout| *l != Layout::D || d_has_count(model);
    // Whether this model has a known layout at all. The zeroed probe stands in
    // for a mechanical-shutter block, which is what selects layout D on the
    // bodies that use it only for those.
    let missing = if layout(model, &[0; 5]).filter(usable).is_some() {
        ShutterCount::NotRecorded
    } else {
        ShutterCount::UnknownModel
    };
    let Some((block, layout)) = block.and_then(|b| Some((b, layout(model, b).filter(usable)?)))
    else {
        return (missing, None);
    };
    let shutter = layout.shutter_offset().and_then(|at| {
        let words = block.get(at..at + 6)?;
        Some(if words.iter().all(|&b| b == 0) { Shutter::Electronic } else { Shutter::Mechanical })
    });
    let at = layout.count_offset();
    let status = match block.get(at..at + 4) {
        Some(b) => {
            ShutterCount::Counted(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) & 0x00ff_ffff)
        }
        None => ShutterCount::NotRecorded,
    };
    (status, shutter)
}

/// Pick the layout for `model`, following the order and conditions of
/// ExifTool's `0x9050` table: the first group that matches wins.
fn layout(model: &str, block: &[u8]) -> Option<Layout> {
    const A_FAMILIES: [&str; 6] = ["SLT-", "ILCA-", "NEX-", "ILCE-", "Lunar", "HV"];
    const A_EXCLUDED: [&str; 20] = [
        "ILCE-1",
        "ILCE-6100",
        "ILCE-6300",
        "ILCE-6400",
        "ILCE-6500",
        "ILCE-6600",
        "ILCE-6700",
        "ILCE-7C",
        "ILCE-7M3",
        "ILCE-7M4",
        "ILCE-7M5",
        "ILCE-7RM2",
        "ILCE-7RM3",
        "ILCE-7RM4",
        "ILCE-7RM5",
        "ILCE-7RM6",
        "ILCE-7SM2",
        "ILCE-7SM3",
        "ILCE-9",
        "ILCA-99M2",
    ];
    const B: [&str; 17] = [
        "ILCE-6100",
        "ILCE-6100A",
        "ILCE-6300",
        "ILCE-6400",
        "ILCE-6400A",
        "ILCE-6500",
        "ILCE-6600",
        "ILCE-7C",
        "ILCE-7M3",
        "ILCE-7RM2",
        "ILCE-7RM3",
        "ILCE-7RM3A",
        "ILCE-7RM4",
        "ILCE-7RM4A",
        "ILCE-7SM2",
        "ILCE-9",
        "ILCE-9M2",
    ];
    const B_OTHER: [&str; 2] = ["ILCA-99M2", "ZV-E10"];
    const C_PREFIX: [&str; 4] = ["ILCE-7M4", "ILCE-7RM5", "ILCE-7SM3", "ILME-FX3"];
    const D: [&str; 6] = ["ILCE-6700", "ILCE-7CM2", "ILCE-7CR", "ILME-FX2", "ZV-E1", "ZV-E10M2"];

    if A_FAMILIES.iter().any(|p| model.starts_with(p))
        && !A_EXCLUDED.iter().any(|p| model.starts_with(p))
    {
        return Some(Layout::A);
    }
    if B.iter().chain(B_OTHER.iter()).any(|w| is_word(model, w)) {
        return Some(Layout::B);
    }
    if is_word(model, "ILCE-1") || C_PREFIX.iter().any(|p| model.starts_with(p)) {
        return Some(Layout::C);
    }
    if D.iter().any(|w| is_word(model, w))
        || (mechanical_only(model) && block.len() >= 5 && block[..5] == [0; 5])
    {
        return Some(Layout::D);
    }
    None
}

/// Bodies that use layout D only for images taken with the mechanical shutter.
fn mechanical_only(model: &str) -> bool {
    ["ILCE-1M2", "ILCE-7M5", "ILCE-7RM6"].iter().any(|p| model.starts_with(p))
}

/// Layout-D bodies whose block is known to carry the count.
fn d_has_count(model: &str) -> bool {
    ["ILCE-1M2", "ILCE-6700", "ILCE-7CM2", "ILCE-7CR", "ILCE-7M5", "ILCE-7RM6", "ILME-FX2"]
        .iter()
        .any(|p| model.starts_with(p))
}

/// Whether `model` starts with `word` followed by a word boundary, as in the
/// `\b` of ExifTool's patterns: `ILCE-7C` must not match `ILCE-7CM2`.
fn is_word(model: &str, word: &str) -> bool {
    model
        .strip_prefix(word)
        .is_some_and(|rest| !rest.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_'))
}

/// Offset and length of the maker note inside the TIFF data.
fn maker_note(exif: &exif::Exif) -> Option<(usize, usize)> {
    match &exif.get_field(Tag::MakerNote, In::PRIMARY)?.value {
        Value::Undefined(bytes, offset) => Some((*offset as usize, bytes.len())),
        _ => None,
    }
}

/// Find `tag` in Sony's maker-note IFD and return its value bytes.
///
/// Offsets inside the maker note are relative to the start of the TIFF data,
/// not to the maker note itself.
fn note_entry(buf: &[u8], le: bool, offset: usize, len: usize, tag: u16) -> Option<&[u8]> {
    let note = buf.get(offset..offset.checked_add(len)?)?;
    let skip = if NOTE_HEADERS.iter().any(|h| note.starts_with(h)) { NOTE_HEADER_LEN } else { 0 };
    let ifd = offset + skip;
    let entries = usize::from(read_u16(buf, ifd, le)?);
    if entries == 0 || entries > MAX_NOTE_ENTRIES {
        return None;
    }
    for i in 0..entries {
        let entry = ifd + 2 + i * 12;
        if read_u16(buf, entry, le)? != tag {
            continue;
        }
        let unit = type_size(read_u16(buf, entry + 2, le)?)?;
        let size = (read_u32(buf, entry + 4, le)? as usize).checked_mul(unit)?;
        let start = if size <= 4 { entry + 8 } else { read_u32(buf, entry + 8, le)? as usize };
        return buf.get(start..start.checked_add(size)?);
    }
    None
}

/// Size in bytes of one value of a TIFF field type.
fn type_size(kind: u16) -> Option<usize> {
    match kind {
        1 | 2 | 6 | 7 => Some(1),
        3 | 8 => Some(2),
        4 | 9 | 11 | 13 => Some(4),
        5 | 10 | 12 => Some(8),
        _ => None,
    }
}

fn read_u16(buf: &[u8], at: usize, le: bool) -> Option<u16> {
    let b: [u8; 2] = buf.get(at..at.checked_add(2)?)?.try_into().ok()?;
    Some(if le { u16::from_le_bytes(b) } else { u16::from_be_bytes(b) })
}

fn read_u32(buf: &[u8], at: usize, le: bool) -> Option<u32> {
    let b: [u8; 4] = buf.get(at..at.checked_add(4)?)?.try_into().ok()?;
    Some(if le { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
}

/// Undo Sony's substitution cipher.
///
/// Cubing modulo 249 is a bijection on `0..249` (249 = 3 · 83, and 3 is
/// coprime to both 2 and 82), so the inverse table is complete.
fn decipher(data: &[u8]) -> Vec<u8> {
    let mut table: [u8; 256] = std::array::from_fn(|i| i as u8);
    for b in 0u32..249 {
        table[(b * b * b % 249) as usize] = b as u8;
    }
    data.iter().map(|&c| table[usize::from(c)]).collect()
}

/// The first non-empty string of an ASCII field, trimmed.
fn ascii(value: &Value) -> Option<String> {
    let Value::Ascii(parts) = value else { return None };
    parts.iter().map(|p| String::from_utf8_lossy(p).trim().to_string()).find(|s| !s.is_empty())
}

/// Turn Exif's `YYYY:MM:DD hh:mm:ss` into `YYYY-MM-DD hh:mm:ss`, followed by
/// the UTC offset when one was recorded.
fn format_time(raw: &str, offset: Option<&str>) -> String {
    let mut time: String = raw
        .char_indices()
        .map(|(i, c)| if (i == 4 || i == 7) && c == ':' { '-' } else { c })
        .collect();
    if let Some(offset) = offset {
        time.push_str(" (UTC");
        time.push_str(offset);
        time.push(')');
    }
    time
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encipher(data: &[u8]) -> Vec<u8> {
        data.iter().map(|&b| if b < 249 { (u32::from(b).pow(3) % 249) as u8 } else { b }).collect()
    }

    /// Append an IFD with its out-of-line values; returns the IFD's offset.
    fn put_ifd(buf: &mut Vec<u8>, entries: &[(u16, u16, Vec<u8>)]) -> u32 {
        let at = buf.len();
        let data_start = at + 2 + entries.len() * 12 + 4;
        let mut data = Vec::new();
        buf.extend((entries.len() as u16).to_le_bytes());
        for (tag, kind, payload) in entries {
            let unit = type_size(*kind).unwrap();
            buf.extend(tag.to_le_bytes());
            buf.extend(kind.to_le_bytes());
            buf.extend(((payload.len() / unit) as u32).to_le_bytes());
            if payload.len() <= 4 {
                let mut inline = payload.clone();
                inline.resize(4, 0);
                buf.extend(inline);
            } else {
                buf.extend(((data_start + data.len()) as u32).to_le_bytes());
                data.extend(payload);
                if data.len() % 2 == 1 {
                    data.push(0);
                }
            }
        }
        buf.extend(0u32.to_le_bytes());
        buf.extend(data);
        at as u32
    }

    /// A minimal little-endian TIFF as a Sony body would write it.
    fn sony_tiff(model: &str, block: &[u8], note_header: bool) -> Vec<u8> {
        let mut buf = b"II\x2a\0\0\0\0\0".to_vec();
        let block_at = buf.len() as u32;
        buf.extend(encipher(block));

        let mut note = Vec::new();
        if note_header {
            note.extend(b"SONY DSC \0\0\0");
        }
        note.extend(1u16.to_le_bytes());
        note.extend(TAG_9050.to_le_bytes());
        note.extend(7u16.to_le_bytes());
        note.extend((block.len() as u32).to_le_bytes());
        note.extend(block_at.to_le_bytes());
        note.extend(0u32.to_le_bytes());

        let exif_ifd = put_ifd(
            &mut buf,
            &[
                (0x9003, 2, b"2026:01:02 10:20:30\0".to_vec()),
                (0x9011, 2, b"+02:00\0".to_vec()),
                (0x927c, 7, note),
                (0xa434, 2, b"E 35mm F1.8 OSS\0".to_vec()),
            ],
        );
        let mut model_z = model.as_bytes().to_vec();
        model_z.push(0);
        let ifd0 = put_ifd(
            &mut buf,
            &[
                (0x010f, 2, b"SONY\0".to_vec()),
                (0x0110, 2, model_z),
                (0x0131, 2, b"ILCE-5100 v3.10\0".to_vec()),
                (0x8769, 4, exif_ifd.to_le_bytes().to_vec()),
            ],
        );
        buf[4..8].copy_from_slice(&ifd0.to_le_bytes());
        buf
    }

    fn layout_a_block(count: u32, shutter: [u16; 3]) -> Vec<u8> {
        let mut block = vec![0x11; 0x200];
        for (i, word) in shutter.iter().enumerate() {
            block[0x20 + i * 2..0x22 + i * 2].copy_from_slice(&word.to_le_bytes());
        }
        block[0x32..0x36].copy_from_slice(&(count | 0x5a00_0000).to_le_bytes());
        block
    }

    #[test]
    fn decipher_undoes_the_cube_cipher() {
        let all: Vec<u8> = (0..=255).collect();
        assert_eq!(decipher(&encipher(&all)), all);
        // Spot checks against the published substitution table.
        assert_eq!(encipher(&[0x02, 0x07, 0x0b]), [0x08, 0x5e, 0x56]);
    }

    #[test]
    fn layouts_follow_the_model_groups() {
        let zero = [0u8; 8];
        let cases = [
            ("ILCE-5100", Some(Layout::A)),
            ("ILCE-7M2", Some(Layout::A)),
            ("NEX-6", Some(Layout::A)),
            ("ILCE-6400A", Some(Layout::B)),
            ("ILCE-7C", Some(Layout::B)),
            ("ZV-E10", Some(Layout::B)),
            ("ILCE-1", Some(Layout::C)),
            ("ILCE-7M4", Some(Layout::C)),
            ("ILCE-7CM2", Some(Layout::D)),
            ("ZV-E10M2", Some(Layout::D)),
            ("ILCE-7M5", Some(Layout::D)),
            ("ILCE-9M3", None),
            ("DSC-RX10M4", None),
            ("XQ-AT51", None),
        ];
        for (model, expected) in cases {
            assert_eq!(layout(model, &zero), expected, "{model}");
        }
        assert_eq!(layout("ILCE-7M5", &[1, 0, 0, 0, 0]), None);
    }

    #[test]
    fn reads_count_date_and_shutter_from_a_raw_style_tiff() {
        let tiff = sony_tiff("ILCE-5100", &layout_a_block(12_345, [1, 2, 3]), false);
        let info = read(&tiff).unwrap();
        assert_eq!(info.model.as_deref(), Some("ILCE-5100"));
        assert_eq!(info.software.as_deref(), Some("ILCE-5100 v3.10"));
        assert_eq!(info.lens.as_deref(), Some("E 35mm F1.8 OSS"));
        assert_eq!(info.taken.as_deref(), Some("2026-01-02 10:20:30 (UTC+02:00)"));
        assert_eq!(info.shutter_count, ShutterCount::Counted(12_345));
        assert_eq!(info.shutter, Some(Shutter::Mechanical));
    }

    #[test]
    fn reads_the_count_from_a_jpeg_with_a_note_header() {
        let tiff = sony_tiff("ILCE-5100", &layout_a_block(42, [0, 0, 0]), true);
        let mut jpeg = vec![0xff, 0xd8, 0xff, 0xe1];
        jpeg.extend(((2 + 6 + tiff.len()) as u16).to_be_bytes());
        jpeg.extend(b"Exif\0\0");
        jpeg.extend(&tiff);
        jpeg.extend([0xff, 0xd9]);
        let info = read(&jpeg).unwrap();
        assert_eq!(info.shutter_count, ShutterCount::Counted(42));
        assert_eq!(info.shutter, Some(Shutter::Electronic));
    }

    #[test]
    fn an_unknown_model_yields_no_number() {
        let tiff = sony_tiff("ILCE-9M3", &layout_a_block(42, [1, 1, 1]), false);
        assert_eq!(read(&tiff).unwrap().shutter_count, ShutterCount::UnknownModel);
    }

    #[test]
    fn a_short_block_is_not_recorded_rather_than_out_of_bounds() {
        let tiff = sony_tiff("ILCE-5100", &[0x11; 0x10], false);
        assert_eq!(read(&tiff).unwrap().shutter_count, ShutterCount::NotRecorded);
    }

    #[test]
    fn a_non_image_is_rejected() {
        assert!(read(b"not an image at all").is_err());
    }
}
