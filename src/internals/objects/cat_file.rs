use crate::internals::hash::commit_hash::CommitHash;

#[derive(Debug, PartialEq, Clone)]
pub struct CatFileConfig {
    pub value: CommitHash,
    pub flags: CatFileMode,
}

impl CatFileConfig {
    pub fn new(value: CommitHash, flags: CatFileMode) -> Self {
        Self { value, flags }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatFileMode {
    /// Display the type of an object
    Type,

    /// Display the content of an object (pretty printed)
    PrettyPrint,

    /// Display the size of an object
    Size,
}
