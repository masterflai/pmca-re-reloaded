//! Read-only reader for the camera's usage log ("KikiLog").
//!
//! [`crate::backup`]'s counterpart for the blob `pmca-usb`'s `get_usage_log`
//! downloads. Sony documents neither, but the two are unequal in how much of
//! them can be recovered: a backup image carries property identifiers the
//! toolkit already knows, whereas the usage log carries event numbers whose
//! meaning exists only inside Sony's service software.
//!
//! What this module therefore does *not* do is name events. The container is
//! regular enough to describe — a fixed header, then fixed-size entries, most
//! of which end in a Unix timestamp — and describing it is worth doing, because
//! the alternative for anyone holding a `usagelog.bin` is a hex editor. But an
//! entry that says `id=0x005d` is reported as `id=0x005d`. Guessing a label for
//! it would turn one person's hypothesis into something a reader mistakes for
//! documentation.
//!
//! That stance proved right. Ten exposures on one camera moved `0x005d` by
//! exactly ten, and the new entries carried their timestamps, so for that body
//! it is a per-exposure event. It is still not the shutter count: the same
//! camera writes almost twice the number into its photos that its log holds
//! such entries, because the log has a fixed size and covers only part of the
//! camera's life. And a `usagelog.bin` does not say which camera it came from,
//! so a label here would claim for every model what was measured on one.
//!
//! Two consequences of that stance are visible in the API. [`Entry`] exposes
//! its three fields as numbers and offers a date only when the last one falls
//! in a plausible range, because not every entry ends in a time: some carry
//! payload bytes belonging to the entry before them. And [`Entry::offset`] is
//! kept, so that anything this module declines to interpret can still be found
//! in the file by hand.
//!
//! Where the payload sits is known for one camera. In every capture from it
//! (nine files, about 28,500 records each), an entry whose second field is
//! `0x8d50` or `0x9630` is followed by exactly one payload slot, and nothing
//! else is. With that rule applied, every remaining value is either a
//! plausible timestamp or exactly zero. The `0x8d50` group of four describes
//! an inserted memory card. Its payloads hold the card's SD CID register
//! (manufacturer, product name, revision, serial number, manufacturing date)
//! and its capacity in 512-byte sectors. That is why a `usagelog.bin` must be
//! treated as identifying data, not as anonymous telemetry. The reader does
//! not apply the rule, for the same reason it does not name `0x005d`: a
//! single camera model is not enough evidence to build it into the format.
//!
//! Nothing here writes. The log is read out of a camera and parsed from a file;
//! no command exists in this toolkit — or, as far as the USB protocol reveals,
//! in the camera — to clear or rewrite it. Note also that the camera flushes
//! the log only on power-off: a download taken while it is still running shows
//! the state of the last flush, not of the last exposure.

use std::fmt;

use crate::util::{parse16le, parse32le};
use crate::{Error, Result};

/// The fixed header, six little-endian `u32` values.
const HEADER_SIZE: usize = 24;

/// One entry: two 16-bit fields and one 32-bit field, little-endian.
const ENTRY_SIZE: usize = 8;

/// Earliest value reported as a date: 2000-01-01T00:00:00Z.
///
/// Cameras predating that do not run this log format, so a smaller value is
/// far more likely to be payload bytes than a time.
const EARLIEST: u32 = 946_684_800;

/// Latest value reported as a date: 2100-01-01T00:00:00Z.
const LATEST: u32 = 4_102_444_800;

/// Seconds in a day.
const DAY: u32 = 86_400;

/// A parsed usage log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageLog {
    /// The header verbatim, so that the three fields this module does not
    /// interpret remain available to a caller who works out what they mean.
    pub header: [u32; 6],
    /// Every entry from the end of the header up to the write position.
    pub entries: Vec<Entry>,
}

impl UsageLog {
    /// Size of the region the entries are written into, in bytes.
    #[must_use]
    pub fn log_size(&self) -> u32 {
        self.header[2]
    }

    /// Offset just past the last entry written, counted from the file start.
    ///
    /// The name is the observation, not a claim about the camera's firmware:
    /// in every capture available the entries stop exactly here and the bytes
    /// beyond are untouched.
    #[must_use]
    pub fn cursor(&self) -> u32 {
        self.header[3]
    }

    /// Size of the second, smaller region that follows the entries.
    ///
    /// Its purpose is unknown; it is reported because header size plus log
    /// size plus this value accounts for the file exactly, which is the
    /// evidence that the header has been read correctly.
    #[must_use]
    pub fn spare_size(&self) -> u32 {
        self.header[4]
    }
}

/// One 8-byte slot of the log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// Where this slot begins in the file, for cross-checking by hand.
    pub offset: usize,
    /// First field. For most slots this identifies what happened; for the
    /// minority that hold payload bytes it is part of the payload instead.
    pub event: u16,
    /// Second field. It stays constant across long runs of entries and
    /// changes between them, which fits a component identifier, but nothing
    /// in the log proves that reading, so it is reported as a number.
    pub source: u16,
    /// Third field. Usually a Unix timestamp; see [`Entry::time`].
    pub value: u32,
}

impl Entry {
    /// The entry's time, if [`Entry::value`] can plausibly be one.
    ///
    /// Returning [`None`] is not an error and does not mean the file is
    /// damaged: entries carrying payload put arbitrary bytes in this field,
    /// and a value outside a sane range is the only signal that distinguishes
    /// them.
    ///
    /// The test only ever rules a value *out*. A payload word can land inside
    /// the window by chance, and some in the captured logs do. They read as
    /// November 2007 or January 2008, but they are really bytes of a memory
    /// card's serial number (see the module documentation). The raw
    /// number therefore stays available either way, and a reader who cares
    /// about a particular entry should look at both.
    #[must_use]
    pub fn time(&self) -> Option<DateTime> {
        (EARLIEST..LATEST).contains(&self.value).then(|| DateTime::from_unix(self.value))
    }
}

/// A UTC date and time, to the second.
///
/// The camera stores seconds since the epoch and says nothing about the zone
/// they were recorded in, so they are rendered as UTC rather than silently
/// shifted into whichever zone the reader's machine happens to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    /// Year, four digits.
    pub year: u32,
    /// Month, 1–12.
    pub month: u32,
    /// Day of month, 1–31.
    pub day: u32,
    /// Hour, 0–23.
    pub hour: u32,
    /// Minute, 0–59.
    pub minute: u32,
    /// Second, 0–59.
    pub second: u32,
}

impl DateTime {
    /// Convert seconds since 1970-01-01T00:00:00Z into a civil date.
    ///
    /// Leap seconds are ignored, as they are in the Unix time the camera
    /// writes; the conversion is the standard days-to-civil algorithm, which
    /// is exact for every value a `u32` can hold.
    #[must_use]
    pub fn from_unix(seconds: u32) -> Self {
        let (days, rest) = (seconds / DAY, seconds % DAY);
        // Shift the epoch to 0000-03-01 so that the leap day falls at the end
        // of the year and the month lengths repeat on a fixed 153-day cycle.
        let z = u64::from(days) + 719_468;
        let era = z / 146_097;
        let day_of_era = z - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
        let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
        let year = year_of_era + era * 400 + u64::from(month <= 2);
        Self {
            year: year as u32,
            month: month as u32,
            day: day as u32,
            hour: rest / 3600,
            minute: rest / 60 % 60,
            second: rest % 60,
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

/// Parse a `usagelog.bin` as downloaded from a camera.
///
/// The checks are the ones the layout itself justifies: the header must be
/// present, the region it describes must fit inside the file, and the write
/// position must land on an entry boundary inside that region. They are enough
/// to reject a file that is not a usage log at all, and deliberately no
/// stricter — the two leading header words look like a version and would make
/// an obvious magic number, but with captures from a single camera model on
/// hand, requiring them would reject other cameras for no reason.
pub fn parse(data: &[u8]) -> Result<UsageLog> {
    if data.len() < HEADER_SIZE {
        return Err(Error::OutOfBounds { context: "usage log header" });
    }
    let mut header = [0u32; 6];
    for (i, word) in header.iter_mut().enumerate() {
        *word = parse32le(&data[i * 4..])?;
    }
    let log_size = header[2] as usize;
    let cursor = header[3] as usize;
    let spare_size = header[4] as usize;

    if log_size == 0 || !log_size.is_multiple_of(ENTRY_SIZE) {
        return Err(Error::Invalid("usage log region size"));
    }
    if HEADER_SIZE + log_size + spare_size > data.len() {
        return Err(Error::OutOfBounds { context: "usage log region" });
    }
    if cursor < HEADER_SIZE
        || cursor > HEADER_SIZE + log_size
        || !(cursor - HEADER_SIZE).is_multiple_of(ENTRY_SIZE)
    {
        return Err(Error::Invalid("usage log write position"));
    }

    let mut entries = Vec::with_capacity((cursor - HEADER_SIZE) / ENTRY_SIZE);
    for offset in (HEADER_SIZE..cursor).step_by(ENTRY_SIZE) {
        entries.push(Entry {
            offset,
            event: parse16le(&data[offset..])?,
            source: parse16le(&data[offset + 2..])?,
            value: parse32le(&data[offset + 4..])?,
        });
    }
    Ok(UsageLog { header, entries })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A log with `entries` entries written into a region of `slots` slots.
    fn image(slots: usize, entries: &[(u16, u16, u32)]) -> Vec<u8> {
        let log_size = slots * ENTRY_SIZE;
        let cursor = HEADER_SIZE + entries.len() * ENTRY_SIZE;
        let mut data = Vec::new();
        for word in [0x0002_0000, 0x0003_0001, log_size as u32, cursor as u32, 0, 0] {
            data.extend_from_slice(&word.to_le_bytes());
        }
        for (event, source, value) in entries {
            data.extend_from_slice(&event.to_le_bytes());
            data.extend_from_slice(&source.to_le_bytes());
            data.extend_from_slice(&value.to_le_bytes());
        }
        data.resize(HEADER_SIZE + log_size, 0);
        data
    }

    #[test]
    fn reads_the_header_and_every_entry_up_to_the_cursor() {
        let data = image(8, &[(0x005d, 0x0830, 1_527_364_591), (0x0068, 0x0830, 1_527_364_591)]);
        let log = parse(&data).unwrap();

        assert_eq!(log.log_size(), 64);
        assert_eq!(log.cursor(), 40);
        assert_eq!(log.spare_size(), 0);
        assert_eq!(log.entries.len(), 2);
        assert_eq!(log.entries[0].offset, HEADER_SIZE);
        assert_eq!(log.entries[0].event, 0x005d);
        assert_eq!(log.entries[1].source, 0x0830);
    }

    /// The untouched tail beyond the write position is not log content, so
    /// reading it would invent thousands of entries reading `0x0000`.
    #[test]
    fn entries_stop_at_the_write_position() {
        let data = image(64, &[(1, 2, EARLIEST)]);
        assert_eq!(parse(&data).unwrap().entries.len(), 1);
    }

    #[test]
    fn a_file_that_is_not_a_usage_log_is_rejected() {
        assert!(parse(b"not any of the formats").is_err());
        assert!(parse(&[0xff; 512]).is_err());

        // A region that does not fit inside the file.
        let mut data = image(4, &[]);
        data.truncate(HEADER_SIZE + 8);
        assert!(parse(&data).is_err());

        // A write position that is not on an entry boundary.
        let mut data = image(4, &[]);
        data[12..16].copy_from_slice(&(HEADER_SIZE as u32 + 3).to_le_bytes());
        assert!(parse(&data).is_err());
    }

    /// The value is only called a time when it can be one; a payload slot far
    /// outside the window gains no false date.
    #[test]
    fn only_plausible_values_become_dates() {
        // The second pair is taken verbatim from a captured log: a payload
        // slot holding part of a memory card's CID (ASCII name characters and
        // manufacturer ID 0x27). It contains no serial number.
        let data = image(4, &[(0x0203, 0x0920, 1_527_364_591), (0x4453, 0x5048, 0x0027_5048)]);
        let log = parse(&data).unwrap();

        assert!(log.entries[0].time().is_some());
        assert_eq!(log.entries[1].time(), None);
        assert_eq!(log.entries[1].value, 0x0027_5048);
    }

    /// The dates the four captured logs decode to, spot-checked against an
    /// independent conversion.
    #[test]
    fn timestamps_convert_to_the_expected_dates() {
        let cases = [
            (946_684_800, "2000-01-01 00:00:00 UTC"),
            (1_527_364_591, "2018-05-26 19:56:31 UTC"),
            (1_580_515_199, "2020-01-31 23:59:59 UTC"),
            (1_583_020_800, "2020-03-01 00:00:00 UTC"),
            (1_785_936_343, "2026-08-05 13:25:43 UTC"),
        ];
        for (seconds, expected) in cases {
            assert_eq!(DateTime::from_unix(seconds).to_string(), expected, "for {seconds}");
        }
    }

    /// 2000 is a leap year and 2100 is not; the shifted-epoch arithmetic is
    /// where that distinction is easy to get wrong.
    #[test]
    fn leap_years_land_on_the_right_day() {
        assert_eq!(DateTime::from_unix(951_782_400).to_string(), "2000-02-29 00:00:00 UTC");
        assert_eq!(DateTime::from_unix(4_107_542_400).to_string(), "2100-03-01 00:00:00 UTC");
    }
}
