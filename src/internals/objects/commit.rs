use std::time;

use crate::internals::{hash::commit_hash::CommitHash, objects::tree::GitrsTree};

#[derive(Debug, PartialEq)]
pub struct Commit {
    hash: CommitHash,

    /// Hash chained to parent
    /// Parent is none if this is the initial commit
    parent: Option<CommitHash>,

    /// Snapshot of the directory
    tree: GitrsTree,
    commit_time: time::SystemTime,
    author: String,
    message: String,
}
