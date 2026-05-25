use std::{
    collections::HashMap,
    ffi::OsString,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use log::info;

use crate::{
    constants::{BASE_DIR_NAME, INDEX_FILE},
    internals::{
        hash::commit_hash::CommitHash,
        objects::{diff::Diff, file::FileMetadata, tree::FileTree},
    },
};

/// Structure that represents the staging area.
/// The index is cached in the gitrs/index file.
pub struct Index {
    entries: Vec<IndexTreeEntry>,
}

impl Index {
    /// separates entries in the index file.
    pub const INDEX_ENTRY_SEPARATOR: u8 = b'\0';

    /// | IndexTreeEntry_1 | INDEX_ENTRY_SEPARATOR | ... | INDEX_ENTRY_SEPARATOR | IndexTreeEntry_n |
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self
            .entries
            .iter()
            .flat_map(|entry| {
                let mut b = entry.to_bytes();
                b.push(Self::INDEX_ENTRY_SEPARATOR);
                b
            })
            .collect();
        if !bytes.is_empty() {
            assert_eq!(bytes.pop(), Some(Self::INDEX_ENTRY_SEPARATOR));
        }
        bytes
    }

    /// Flushes an index to the gitrs/index file
    pub fn to_idx_file(&self) -> Result<(), std::io::Error> {
        let mut f_idx = Self::get_idx_file()?;
        let bytes = self.to_bytes();
        f_idx.write_all(&bytes)
    }

    /// Reads bytes from gitrs/index file.
    /// Then calls Self::from_bytes()
    pub fn from_idx_file() -> Result<Self, std::io::Error> {
        let mut f_idx = Self::get_idx_file()?;
        let mut bytes: Vec<u8> = vec![];
        f_idx.read_to_end(&mut bytes)?;
        Ok(Self::from_bytes(bytes).unwrap_or_else(|| {
            info!("Empty index file");
            Self { entries: vec![] }
        }))
    }

    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let entries: Option<Vec<IndexTreeEntry>> = bytes
            .split(|b| *b == Self::INDEX_ENTRY_SEPARATOR)
            .map(IndexTreeEntry::from_bytes)
            .collect();
        Some(Self { entries: entries? })
    }

    fn get_idx_file() -> Result<File, std::io::Error> {
        let mut path = PathBuf::from(BASE_DIR_NAME);
        path.push(INDEX_FILE);
        File::open(path)
    }

    /// Compares an index with a filetree and returns the difference between the two.
    pub fn compare_file_tree<'a, 'b>(&self, ft: FileTree) -> Diff<'a, 'b> {
        let s1 = ft.to_index().to_map();
        let s2 = self.to_map();
        todo!()
        // Diff::new(&s1, &s2)
    }

    /// Go from Vector of IndexTreeEntries to a map
    /// Used for index comparison
    fn to_map(&self) -> HashMap<&Path, IndexTreeEntry> {
        let hm: HashMap<&Path, IndexTreeEntry> = self
            .entries
            .iter()
            .clone()
            .map(|v| (Path::new(&v.filepath), v.clone()))
            .collect();
        hm
    }
}

/// Entry in the index structure.
#[derive(Clone, Debug)]
pub struct IndexTreeEntry {
    pub metadata: FileMetadata,
    pub hash: CommitHash,
    pub filepath: OsString,
}

impl IndexTreeEntry {
    pub fn new(metadata: FileMetadata, hash: CommitHash, filepath: OsString) -> Self {
        Self {
            metadata,
            hash,
            filepath,
        }
    }

    /// IndexTreeEntry -> bytes
    /// | metadata (16b) | hash (8b) | filepath (n bytes) |
    pub fn to_bytes(&self) -> Vec<u8> {
        let b_filepath: Vec<u8> = (*self.filepath).to_str().unwrap().bytes().collect();
        let bytes: Vec<u8> = [
            self.metadata.to_bytes().to_vec(),
            self.hash.to_bytes().to_vec(),
            b_filepath,
        ]
        .into_iter()
        .flatten()
        .collect();
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < FileMetadata::BYTE_LEN + CommitHash::HASH_LEN + 1 {
            return None;
        }
        let mut idx = 0;
        let metadata = {
            let md = FileMetadata::from_bytes(&bytes[..FileMetadata::BYTE_LEN]).ok()?;
            idx += FileMetadata::BYTE_LEN;
            md
        };
        let hash = {
            let h =
                CommitHash::from_bytes(&bytes[idx..idx + CommitHash::HASH_LEN].try_into().unwrap());
            idx += CommitHash::HASH_LEN;
            h
        };
        let filepath = String::from_utf8_lossy(&bytes[idx..]).to_string();
        Some(Self {
            metadata,
            hash,
            filepath: filepath.into(),
        })
    }
}
