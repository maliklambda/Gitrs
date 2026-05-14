use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{
    constants::{BASE_DIR_NAME, HEAD_FILE, head_prefixes::*},
    internals::hash::commit_hash::CommitHash,
};

/// Special head commit.
pub struct HeadCommit {
    /// Reference type
    prefix: HeadPrefix,

    /// Path to which HEAD refers
    path: PathBuf,
}

impl HeadCommit {
    pub fn new(prefix: HeadPrefix, path: PathBuf) -> Self {
        Self { prefix, path }
    }

    /// Reads the commit to which HEAD refers.
    /// Goes to path and reads the commit.
    pub fn read_commit(&self) -> CommitHash {
        let mut buffer = String::new();
        std::fs::File::open(&self.path)
            .expect("Could not open file that HEAD references.")
            .read_to_string(&mut buffer)
            .unwrap();
        todo!()
    }
}

/// The prefix that is written to the gitrs/HEAD file before the path to the reference
#[derive(Debug)]
pub enum HeadPrefix {
    Ref,
    Remote,
    Tag,
}

impl HeadPrefix {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Ref => HEAD_REF_STR,
            Self::Remote => HEAD_RMT_STR,
            Self::Tag => HEAD_TAG_STR,
        }
    }

    pub fn from_str(mut s: &str) -> Option<Self> {
        if let Some(s2) = s.strip_suffix(":") {
            s = s2;
        }
        match s {
            HEAD_REF_STR => Some(Self::Ref),
            HEAD_TAG_STR => Some(Self::Tag),
            HEAD_RMT_STR => Some(Self::Remote),
            _ => None,
        }
    }
}

/// Convert a path to the format in which it will be stored in .gitrs/HEAD
pub fn path_to_head_format(path: &Path, prefix: HeadPrefix) -> String {
    format!("{}: {}\n", prefix.to_str(), path.to_str().unwrap())
}

/// Reads path stored in .gitrs/HEAD
/// Usual format is "<prefix>: <path>\n" where prefix is a HeadPrefix
/// Panics if the HEAD file is corrupted
pub fn read_head_path() -> HeadCommit {
    let mut header_path = PathBuf::from(BASE_DIR_NAME);
    header_path.push(HEAD_FILE);
    let mut f = std::fs::File::open(header_path).expect("Corrupted HEAD file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let parts: Vec<&str> = buffer.split_whitespace().collect();
    assert_eq!(parts.len(), 2);
    let path = PathBuf::from(parts[1]);

    match &(parts[0])[0..HEAD_PREFIX_LEN] {
        HEAD_REF_STR => HeadCommit::new(HeadPrefix::Ref, path),
        HEAD_TAG_STR => HeadCommit::new(HeadPrefix::Tag, path),
        HEAD_RMT_STR => HeadCommit::new(HeadPrefix::Remote, path),
        _ => panic!("Corrupted HEAD format"),
    }
}

/// Write a new path to HEAD
pub fn write_head_path(f: &mut File, prefix: HeadPrefix, path: &Path) {
    f.write_all(path_to_head_format(path, prefix).as_bytes())
        .unwrap();
}
