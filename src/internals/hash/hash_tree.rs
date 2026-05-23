use std::path::PathBuf;

use log::info;

use crate::{
    constants::CONTENT_DIR,
    internals::{
        hash::commit_hash::CommitHash,
        objects::tree::{FileTree, GitrsTree},
    },
};

#[derive(Debug)]
pub enum HashTreeError<'a> {
    InvalidPath(&'a str),
}

/// Hashes a subtree.
/// Argument filepath refers to a path in the content directory.
/// Returns file tree root and its children
pub fn hash_tree<'a>(filepath: &'a str) -> Result<FileTree, HashTreeError<'a>> {
    let mut path = PathBuf::from(CONTENT_DIR);
    path.push(filepath);
    if !path.exists() || !path.is_dir() {
        return Err(HashTreeError::InvalidPath(filepath));
    }

    // Todo: change this to build from existing hash files.
    // Only if that does not work, build initial.
    let tree = FileTree::build_initial(&path).unwrap();
    info!("Tree: {:?}", tree);
    Ok(tree)
}
