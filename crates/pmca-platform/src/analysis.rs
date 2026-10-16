//! Finding the backup property a menu setting is stored in.
//!
//! The procedure is the one measured by hand in tests 5.28 and 5.29: read the
//! complete backup image once per menu value, change the setting on the camera
//! between the reads, and finish with a read at the starting value again. A
//! property that follows the menu and is back at its first value at the end is
//! a candidate; one that changed but did not come back is the camera's own
//! running state (counters, timestamps) and is reported separately.
//!
//! Nothing here touches a camera. The functions work on images some backend
//! has read, so the decision which property to trust is fully unit-testable.

use std::collections::BTreeMap;

use pmca_core::backup::{BackupFile, attr};

use crate::Result;

/// Largest value the write test will write. Menu settings measured so far are
/// one byte; anything much larger is more likely a table than a setting.
pub const MAX_WRITE_SIZE: usize = 16;

/// One property as it was found in one image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The property's attribute bits.
    pub attr: u16,
    /// The stored value.
    pub data: Vec<u8>,
    /// The reset value, where the property has one.
    pub reset: Option<Vec<u8>>,
}

/// Every property of one backup image, read while the menu showed `label`.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// The menu value the user had set, as they named it.
    pub label: String,
    entries: BTreeMap<u32, Entry>,
}

impl Snapshot {
    /// Parse a complete backup image.
    pub fn parse(label: impl Into<String>, image: &[u8]) -> Result<Self> {
        let file = BackupFile::new(image)?;
        let entries = file
            .list_properties()?
            .into_iter()
            .map(|(id, p)| (id, Entry { attr: p.attr, data: p.data, reset: p.reset_data }))
            .collect();
        Ok(Self { label: label.into(), entries })
    }

    /// Number of properties in the image.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the image holds no property at all.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// One property, if the image holds it.
    pub fn get(&self, id: u32) -> Option<&Entry> {
        self.entries.get(&id)
    }
}

/// A property that followed the menu setting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// Property id.
    pub id: u32,
    /// Attribute bits as found in the first image.
    pub attr: u16,
    /// Reset value from the first image, where there is one.
    pub reset: Option<Vec<u8>>,
    /// The value in every snapshot, in the order they were taken.
    pub values: Vec<Vec<u8>>,
    /// Every menu value maps to its own property value, and a menu value named
    /// twice maps to the same one both times.
    pub consistent: bool,
}

impl Candidate {
    /// Size of the value in bytes.
    pub fn size(&self) -> usize {
        self.values.first().map_or(0, Vec::len)
    }
}

/// What a series of snapshots showed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Evaluation {
    /// Properties that followed the menu, consistent ones first.
    pub candidates: Vec<Candidate>,
    /// Properties that changed but did not return: the camera's running state,
    /// or the setting itself when the first snapshot was not taken at the
    /// first menu value. Never consistent.
    pub running: Vec<Candidate>,
    /// Properties present in every image and therefore compared.
    pub compared: usize,
}

/// Compare the snapshots of one run.
///
/// The first and the last snapshot must have been taken at the same menu
/// value. Fewer than three snapshots cannot tell a setting from a counter and
/// yield an empty evaluation.
pub fn evaluate(snapshots: &[Snapshot]) -> Evaluation {
    let mut out = Evaluation::default();
    let (Some(first), Some(last)) = (snapshots.first(), snapshots.last()) else {
        return out;
    };
    if snapshots.len() < 3 {
        return out;
    }
    let menu = &snapshots[..snapshots.len() - 1];

    for (&id, start) in &first.entries {
        let Some(values) =
            snapshots.iter().map(|s| s.get(id).map(|e| e.data.clone())).collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        out.compared += 1;
        if values.iter().all(|v| *v == start.data) {
            continue;
        }
        if last.get(id).map(|e| &e.data) != Some(&start.data) {
            out.running.push(Candidate {
                id,
                attr: start.attr,
                reset: start.reset.clone(),
                values,
                consistent: false,
            });
            continue;
        }
        let consistent = menu.iter().enumerate().all(|(i, a)| {
            menu.iter()
                .enumerate()
                .skip(i + 1)
                .all(|(j, b)| (a.label == b.label) == (values[i] == values[j]))
        });
        out.candidates.push(Candidate {
            id,
            attr: start.attr,
            reset: start.reset.clone(),
            values,
            consistent,
        });
    }
    out.candidates.sort_by_key(|c| (!c.consistent, c.id));
    out
}

/// Why the write test is not offered for an evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// No property followed the menu.
    NoCandidate,
    /// More than one property did; writing one of them proves nothing.
    Ambiguous(usize),
    /// The only candidate did not map menu values to values one to one.
    Inconsistent,
    /// The value is larger than [`MAX_WRITE_SIZE`].
    TooLarge(usize),
    /// The camera marks the property read-only.
    ReadOnly,
    /// The camera marks the property protected.
    Protected,
}

/// The one property a write test may write, or why there is none.
///
/// Deliberately narrow: exactly one consistent candidate, small, and not
/// marked read-only or protected. Anything else is left to a person reading
/// the report.
pub fn write_target(evaluation: &Evaluation) -> std::result::Result<&Candidate, Refusal> {
    let candidate = match evaluation.candidates.as_slice() {
        [] => return Err(Refusal::NoCandidate),
        [one] => one,
        many => return Err(Refusal::Ambiguous(many.len())),
    };
    if !candidate.consistent {
        return Err(Refusal::Inconsistent);
    }
    if candidate.size() == 0 || candidate.size() > MAX_WRITE_SIZE {
        return Err(Refusal::TooLarge(candidate.size()));
    }
    if candidate.attr & attr::READ_ONLY != 0 {
        return Err(Refusal::ReadOnly);
    }
    if candidate.attr & attr::PROTECTED != 0 {
        return Err(Refusal::Protected);
    }
    Ok(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::sample_image;

    /// `sample_image` with some values replaced.
    fn image(changes: &[(u32, &[u8])]) -> Vec<u8> {
        let mut file = BackupFile::new(sample_image()).unwrap();
        for (id, data) in changes {
            file.set_property(*id, data).unwrap();
        }
        file.update_checksum();
        file.into_inner()
    }

    fn snap(label: &str, changes: &[(u32, &[u8])]) -> Snapshot {
        Snapshot::parse(label, &image(changes)).unwrap()
    }

    const SETTING: u32 = 0x0164_0001;
    const COUNTER: u32 = 0x003c_04b6;

    #[test]
    fn finds_the_setting_and_sets_the_counter_aside() {
        let snaps = [
            snap("Standard", &[]),
            snap("High", &[(SETTING, &[1]), (COUNTER, &[0x2d, 0x01])]),
            snap("Standard", &[(COUNTER, &[0x2e, 0x01])]),
        ];
        let eval = evaluate(&snaps);
        assert_eq!(eval.candidates.len(), 1);
        let c = &eval.candidates[0];
        assert_eq!(c.id, SETTING);
        assert_eq!(c.values, vec![vec![0], vec![1], vec![0]]);
        assert!(c.consistent);
        assert_eq!(eval.running.iter().map(|c| c.id).collect::<Vec<_>>(), vec![COUNTER]);
        assert_eq!(eval.compared, snaps[0].len());
        assert_eq!(write_target(&eval).unwrap().id, SETTING);
    }

    #[test]
    fn a_counter_that_happens_to_return_is_still_inconsistent_with_three_values() {
        let snaps = [
            snap("Low", &[]),
            snap("Mid", &[(SETTING, &[1]), (COUNTER, &[1, 0])]),
            snap("High", &[(SETTING, &[2]), (COUNTER, &[1, 0])]),
            snap("Low", &[]),
        ];
        let eval = evaluate(&snaps);
        assert_eq!(eval.candidates.len(), 2);
        assert_eq!(eval.candidates[0].id, SETTING);
        assert!(eval.candidates[0].consistent);
        assert!(!eval.candidates[1].consistent);
        assert_eq!(write_target(&eval), Err(Refusal::Ambiguous(2)));
    }

    #[test]
    fn a_wrong_starting_value_leaves_the_setting_among_the_running_values() {
        // The camera already stood at the second value when the run began.
        let snaps = [
            snap("Standard", &[(SETTING, &[1])]),
            snap("High", &[(SETTING, &[1])]),
            snap("Standard", &[]),
        ];
        let eval = evaluate(&snaps);
        assert!(eval.candidates.is_empty());
        assert_eq!(eval.running.len(), 1);
        assert_eq!(eval.running[0].id, SETTING);
        assert_eq!(eval.running[0].values, vec![vec![1], vec![1], vec![0]]);
    }

    #[test]
    fn nothing_changed_means_no_candidate() {
        let snaps = [snap("A", &[]), snap("B", &[]), snap("A", &[])];
        let eval = evaluate(&snaps);
        assert!(eval.candidates.is_empty());
        assert_eq!(write_target(&eval), Err(Refusal::NoCandidate));
    }

    #[test]
    fn two_snapshots_are_not_enough() {
        let snaps = [snap("A", &[]), snap("B", &[(SETTING, &[1])])];
        assert_eq!(evaluate(&snaps), Evaluation::default());
    }

    #[test]
    fn refuses_read_only_and_large_values() {
        // 0x0107_0148 is read-only in the sample image.
        let ro = 0x0107_0148;
        let snaps = [snap("A", &[]), snap("B", &[(ro, &[1])]), snap("A", &[])];
        assert_eq!(write_target(&evaluate(&snaps)), Err(Refusal::ReadOnly));

        let big = 0x02cf_0bd2;
        let snaps = [snap("A", &[]), snap("B", &[(big, &[1; 0x2f])]), snap("A", &[])];
        assert_eq!(write_target(&evaluate(&snaps)), Err(Refusal::TooLarge(0x2f)));
    }

    #[test]
    fn the_same_menu_value_twice_must_give_the_same_value() {
        let snaps = [
            snap("A", &[]),
            snap("B", &[(SETTING, &[1])]),
            snap("A", &[(SETTING, &[2])]),
            snap("A", &[]),
        ];
        let eval = evaluate(&snaps);
        assert_eq!(write_target(&eval), Err(Refusal::Inconsistent));
    }
}
