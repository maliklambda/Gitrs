use std::{
    ffi::OsString,
    fs::{DirEntry, read_dir},
    path::Path,
};

use log::{debug, info};

use crate::internals::hash::{commit_hash::CommitHash, hash_blob::hash_file_content};

/// A snapshot of the file system.
/// A tree consists of all root level nodes which in term reference their children
/// (if they themselves are sub-directories)
#[derive(Debug, PartialEq)]
pub struct GitrsTree {
    pub content: Vec<TreeNode>,
}

impl GitrsTree {
    pub fn build_tree(path: &Path) -> Result<Self, std::io::Error> {
        debug!("Building (sub-)tree from path: {:?}", path);
        let dir = read_dir(path).unwrap();
        let content: Vec<TreeNode> = dir.map(|entry| TreeNode::new(entry.unwrap())).collect();
        for node in &content {
            info!("Entry node: {:?}", node);
        }
        Ok(Self { content })
    }
}

/// Nodes in the tree structure.
/// Since the tree represents a file-system, a node can be either
/// A) a file with a link to its content, or
/// B) a subdirectory (or sub-tree) with links to other TreeNodes.
#[derive(Debug, PartialEq)]
pub enum TreeNode {
    File {
        /// Filename
        name: OsString,

        // TODO: Should be enabled
        // /// a hash that refers to the file itself
        // hash: CommitHash,
        /// Hash value of the file's content as a BLOB
        content: CommitHash,
    },
    Subdir {
        /// Subdirectory name
        name: OsString,

        /// The directory's content
        content: GitrsTree,
    },
}

impl TreeNode {
    pub fn new(entry: DirEntry) -> Self {
        if entry.file_type().unwrap().is_file() {
            info!("File node: {:?}", entry.file_name());
            let h = hash_file_content(&entry.path());
            TreeNode::File {
                name: entry.file_name(),
                content: h.unwrap(),
            }
        } else if entry.file_type().unwrap().is_dir() {
            let new_path = entry.path();
            info!("Subdir node: {:?}", new_path);
            let tree = GitrsTree::build_tree(&new_path).unwrap();
            info!("Build subdir: {:?}", tree);
            Self::Subdir {
                name: entry.file_name(),
                content: tree,
            }
        } else {
            panic!(
                "Unallowed dir entry (only files and dirs are supported): {:?}",
                entry
            );
        }
    }
}
