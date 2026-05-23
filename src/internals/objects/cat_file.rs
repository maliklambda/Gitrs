use crate::internals::hash::commit_hash::CommitHash;

#[derive(Debug, PartialEq, Clone)]
pub struct CatFileConfig {
    pub values: Vec<CommitHash>,
    pub flags: CatFileMode,
}

impl CatFileConfig {
    pub fn new(values: Vec<CommitHash>, flags: CatFileMode) -> Self {
        Self { values, flags }
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
