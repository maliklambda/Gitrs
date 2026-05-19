use crate::internals::hash::commit_hash::CommitHash;

#[derive(Debug, PartialEq, Clone)]
pub struct CatFileConfig {
    value: CommitHash,
    flags: CatFileFlags,
}

impl CatFileConfig {
    pub fn new(value: CommitHash, flags: CatFileFlags) -> Self {
        Self { value, flags }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CatFileFlags {
    /// Display the type of an object
    Type,

    /// Display the content of an object (pretty printed)
    PrettyPrint,

    /// Display the size of an object
    Size,
}
