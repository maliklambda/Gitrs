use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

/// Main structure used for hashing.
/// Any gitrs hashing is done via a CommitHash (not just commits, also blobs, trees, etc.)
/// Wraps the hash in a struct field to allow impl blocks below
/// Called "CommitHash" instead of "Hash" to avoid ambiguity with stdlib Hash
#[derive(Debug, PartialEq)]
pub struct CommitHash {
    hash: u64,
}

impl std::fmt::Display for CommitHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl CommitHash {
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
}
