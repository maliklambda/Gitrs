use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

/// Main structure used for hashing.
/// Any gitrs hashing is done via a CommitHash (not just commits, also blobs, trees, etc.)
/// Wraps the hash in a struct field to allow impl blocks below
/// Called "CommitHash" instead of "Hash" to avoid ambiguity with stdlib Hash
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Eq, Hash)]
pub struct CommitHash {
    pub hash: u64,
}

impl std::fmt::Display for CommitHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl CommitHash {
    /// Return the length of a hash in bytes
    pub const HASH_LEN: usize = std::mem::size_of::<u64>();

    pub fn new(s: &str) -> Self {
        let h = {
            let mut h = DefaultHasher::new();
            s.hash(&mut h);
            h.finish()
        };
        Self { hash: h }
    }

    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.hash.to_string())
    }

    pub fn to_str(&self) -> String {
        self.hash.to_string()
    }

    pub fn to_bytes(&self) -> [u8; Self::HASH_LEN] {
        self.hash.to_be_bytes()
    }

    pub fn from_bytes(bytes: &[u8; Self::HASH_LEN]) -> Self {
        Self {
            hash: u64::from_be_bytes(*bytes),
        }
    }
}
