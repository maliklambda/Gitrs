use std::{collections::HashMap, path::Path};

use crate::internals::objects::index::IndexTreeEntry;

/// The difference between two snapshots S1 and S2.
///
/// Example:
///     let S1 = [
///         (a.txt: 4fwed...),
///         (b.txt: ab2d5...),
///     ]
///     let S2 = [
///         (a.txt: gf437...),
///     ]
///
///     Then Diff(S1, S2) = {
///         a.txt: Modified(4fwed... -> gf437...),
///         b.txt: Deleted
///     }
#[derive(Debug)]
pub struct Diff<'a, 'b> {
    diffs: HashMap<&'a Path, SingleDiff<'b>>,
}

impl<'a, 'b> Diff<'a, 'b> {
    /// Expects all hashes in s1 to be written to file.
    pub fn new(
        s1: &'b HashMap<&'a Path, IndexTreeEntry>,
        s2: &'b HashMap<&'a Path, IndexTreeEntry>,
    ) -> Self
    where
        'b: 'a,
    {
        // todo: remove option wrapper
        let diffs_s1: Vec<(&Path, SingleDiff)> = s1
            .iter()
            .filter_map(|(k, v1)| {
                if let Some(v2) = s2.get(k) {
                    Some((*k, Self::cmp_entries(v1, v2)?))
                } else {
                    // in s1 but not in s2
                    // Since s2 is after s1, the value must have been deleted
                    Some((*k, SingleDiff::Deleted { before: v1 }))
                }
            })
            .collect();

        // values that have been added in s2
        // (that are present in s2 but not in s1)
        let diffs_s2: Vec<(&Path, SingleDiff)> = s2
            .iter()
            .filter_map(|(k, v2)| {
                if s1.contains_key(k) {
                    None
                } else {
                    Some((*k, SingleDiff::Added { after: v2 }))
                }
            })
            .collect();

        let diffs: HashMap<&Path, SingleDiff> =
            [diffs_s1, diffs_s2].into_iter().flatten().collect();

        Self { diffs }
    }

    /// Compare two IndexTreeEntries.
    /// Return the difference between both of them or None if they are the same.
    fn cmp_entries(e1: &'a IndexTreeEntry, e2: &'a IndexTreeEntry) -> Option<SingleDiff<'a>>
    where
        'b: 'a,
    {
        if e1.hash == e2.hash {
            None
        } else {
            Some(SingleDiff::Modified {
                before: e1,
                after: e2,
            })
        }
    }
}

#[derive(Debug)]
pub enum SingleDiff<'a> {
    Added {
        after: &'a IndexTreeEntry,
    },
    Modified {
        before: &'a IndexTreeEntry,
        after: &'a IndexTreeEntry,
    },
    Deleted {
        before: &'a IndexTreeEntry,
    },
}
