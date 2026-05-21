use std::{
    ffi::OsString,
    fs::{DirEntry, read_dir},
    path::Path,
};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::internals::{
    hash::{commit_hash::CommitHash, hash_blob::hash_file_content},
    objects::ObjectType,
};

/// A snapshot of the file system.
/// A tree consists of all root level nodes which in term reference their children
/// (if they themselves are sub-directories)
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitrsTree {
    pub dir_name: OsString,
    pub content: Vec<TreeNode>,
}

impl GitrsTree {
    /// Build in-memory GitrsTree from existing files.
    pub fn build_tree(path: &Path) -> Result<Self, std::io::Error> {
        debug!("Building (sub-)tree from path: {:?}", path);
        let dir = read_dir(path).unwrap();
        let content: Vec<TreeNode> = dir.map(|entry| TreeNode::new(entry.unwrap())).collect();
        for node in &content {
            info!("Entry node: {:?}", node);
        }
        Ok(Self {
            content,
            dir_name: path.iter().next_back().unwrap().to_os_string(),
        })
    }

    /// GitrsTree to File Trees from scratch.
    /// Returns the root hash & all sub file trees.
    /// GitrsTrees cannot simply be stored as an object^1.
    /// To be storable, they must be converted to an
    /// ^1 While it can be, it is advantageous to not store the entire tree,
    /// but rather hash-chains of subtrees. This saves space & complexity for the storage.
    pub fn to_file_trees(&self) -> (CommitHash, Vec<FileTree>) {
        let mut file_trees: Vec<FileTree> = vec![];
        let root_hash = self.flatten_recursive(&mut file_trees);
        (root_hash.hash, file_trees)
    }

    fn flatten_recursive(&self, trees: &mut Vec<FileTree>) -> FileTreeNode {
        let mut ftns: Vec<FileTreeNode> = vec![];
        for node in &self.content {
            match node {
                TreeNode::File { name, content } => ftns.push(FileTreeNode {
                    object_type: ObjectType::Blob,
                    hash: *content,
                    filename: name.to_owned().into_string().unwrap(),
                }),
                TreeNode::Subdir { name: _, content } => {
                    let ftn = content.flatten_recursive(trees);
                    ftns.push(ftn);
                }
            }
        }
        let new_ft = FileTree { values: ftns };
        let hash = new_ft.to_hash();
        trees.push(new_ft);
        FileTreeNode {
            object_type: ObjectType::Tree,
            hash,
            filename: self.dir_name.to_str().unwrap().to_string(),
        }
    }

    /// Compute hash of tree
    pub fn to_hash(&self) -> CommitHash {
        // compute string from content
        let s = format!("{:?}", self.content);
        CommitHash::new(&s)
    }


    /// Takes a file tree (that is presumably read from a file) and builds a GitrsTree from that.
    /// For the full tree, the objects that are given as a hash will need to be pulled recursively 
    /// from their respective files.
    pub fn from_file_tree(ft: FileTree) -> Result<Self, std::io::Error> {
        todo!("filetree -> gitrstree");
    }
}

/// Nodes in the tree structure.
/// Since the tree represents a file-system, a node can be either
/// A) a file with a link to its content, or
/// B) a subdirectory (or sub-tree) with links to other TreeNodes.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum TreeNode {
    File {
        /// Filename
        name: OsString,

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

/// GitrsTree representaion on file.
/// Instead of recursive tree references, the hash of the subtree is kept.
/// References to files do not much change.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileTree {
    pub values: Vec<FileTreeNode>,
}

impl FileTree {
    pub const FT_NODE_SEPARATOR: u8 = b'\n';

    pub fn to_hash(&self) -> CommitHash {
        CommitHash::new(
            str::from_utf8(
                &self
                    .values
                    .iter()
                    .flat_map(|node| node.hash.to_str().into_bytes())
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.values
            .iter()
            .flat_map(|node| {
                let mut bytes = node.to_bytes();
                bytes.push(Self::FT_NODE_SEPARATOR); // newline as node separator
                bytes
            })
            .collect::<Vec<u8>>()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, std::io::Error> {
        let values: Vec<FileTreeNode> = bytes
            .split(|b| *b == Self::FT_NODE_SEPARATOR)
            .filter_map(|bytes| {
                if !bytes.is_empty() {
                    Some(FileTreeNode::from_bytes(bytes).unwrap())
                } else {
                    None
                }
            })
            .collect();
        Ok(Self { values })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FileTreeNode {
    // /// file/dir permissions
    // permissions: u32,
    /// File (blob) or Subdir
    pub object_type: ObjectType,

    /// The file or subdir hashed
    pub hash: CommitHash,

    /// File or dir name
    pub filename: String,
}

impl FileTreeNode {
    /// | object_type (1 byte) | hash (8 bytes) | filename (n bytes; until EOF)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![self.object_type.to_u8()];
        bytes.extend(self.hash.to_bytes());
        bytes.extend(self.filename.as_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        let object_type = ObjectType::from_u8(bytes[0]).unwrap();
        let hash = CommitHash::from_bytes(&bytes[1..CommitHash::HASH_LEN + 1].try_into().unwrap());
        let filename = str::from_utf8(&bytes[1 + std::mem::size_of::<CommitHash>()..])
            .unwrap()
            .to_string();
        Ok(Self {
            object_type,
            hash,
            filename,
        })
    }
}

#[test]
fn file_tree_node_serialization() {
    let ft_nodes = [
        FileTreeNode {
            object_type: ObjectType::Tree,
            hash: CommitHash::new("a tree"),
            filename: String::from("subdir"),
        },
        FileTreeNode {
            object_type: ObjectType::Blob,
            hash: CommitHash::new("a blob"),
            filename: String::from("file.txt"),
        },
    ];

    for node_old in ft_nodes {
        let bytes = node_old.to_bytes();
        let node_new =
            FileTreeNode::from_bytes(&bytes).expect("Failed to convert: bytes -> FT Node");
        assert_eq!(node_old, node_new);
    }
}

#[test]
fn file_tree_serialization() {
    let ft_nodes = vec![
        FileTreeNode {
            object_type: ObjectType::Tree,
            hash: CommitHash::new("a tree"),
            filename: String::from("subdir"),
        },
        FileTreeNode {
            object_type: ObjectType::Blob,
            hash: CommitHash::new("a blob"),
            filename: String::from("file.txt"),
        },
    ];
    let ft = FileTree { values: ft_nodes };
    let bytes = ft.to_bytes();
    let ft_new = FileTree::from_bytes(bytes).expect("Failed to convert: bytes -> FT");
    assert_eq!(ft, ft_new);
}
