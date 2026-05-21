use std::path::PathBuf;

use log::info;

use crate::{
    constants::CONTENT_DIR,
    internals::{hash::commit_hash::CommitHash, objects::tree::GitrsTree},
};

#[derive(Debug)]
pub enum HashTreeError<'a> {
    InvalidPath(&'a str),
}

pub fn hash_tree<'a>(filepath: &'a str) -> Result<(CommitHash, GitrsTree), HashTreeError<'a>> {
    let mut path = PathBuf::from(CONTENT_DIR);
    path.push(filepath);
    if !path.exists() || !path.is_dir() {
        return Err(HashTreeError::InvalidPath(filepath));
    }

    let tree = GitrsTree::build_tree(&path).unwrap();
    info!("Tree: {:?}", tree);
    let hash = tree.to_hash();
    Ok((hash, tree))
}
