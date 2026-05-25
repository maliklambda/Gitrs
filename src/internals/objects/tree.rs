use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{DirEntry, read_dir},
    io::Read,
    path::{Path, PathBuf},
};

use log::{debug, info};

use crate::{
    constants::{BASE_DIR_NAME, OBJECTS_DIR},
    internals::{
        hash::{commit_hash::CommitHash, hash_blob::file_content},
        objects::{
            ObjectType,
            file::FileMetadata,
            index::{Index, IndexTreeEntry},
        },
    },
};

/// A snapshot of the file system.
/// A tree consists of all root level nodes which in term reference their children
/// (if they themselves are sub-directories)
#[derive(Debug, PartialEq, Clone)]
pub struct GitrsTree {
    pub dir_name: OsString,
    pub content: Vec<TreeNode>,
}

impl GitrsTree {
    /// Build in-memory GitrsTree from existing files.
    /// This is the initial build. It assumes no existing files in gitrs/objects/
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
    /// To be storable, they must be converted to a FileTree.
    /// ^1 While it can be, it is advantageous to not store the entire tree,
    /// but rather hash-chains of subtrees. This saves space & complexity for the storage.
    pub fn to_file_trees(&self) -> (CommitHash, Vec<FileTree>, HashMap<CommitHash, String>) {
        let mut file_trees: Vec<FileTree> = vec![];

        // keep track of blob contents.
        // Since the blob content is lost when converting to a FileTree (only the hash remains).
        let mut blob_contents: HashMap<CommitHash, String> = HashMap::new();
        let root = self.flatten_recursive(&mut file_trees, &mut blob_contents);
        (root.hash, file_trees, blob_contents)
    }

    fn flatten_recursive(
        &self,
        trees: &mut Vec<FileTree>,
        blob_contents: &mut HashMap<CommitHash, String>,
    ) -> FileTreeNode {
        let mut ftns: Vec<FileTreeNode> = vec![];
        for node in &self.content {
            match node {
                TreeNode::File {
                    name,
                    content,
                    metadata,
                } => {
                    let h = CommitHash::new(content);
                    blob_contents.insert(h, content.to_string());
                    ftns.push(FileTreeNode {
                        object_type: ObjectType::Blob,
                        hash: h,
                        filepath: name.to_owned().into_string().unwrap().into(),
                        metadata: metadata.clone(),
                    })
                }
                TreeNode::Subdir { name: _, content } => {
                    let ftn = content.flatten_recursive(trees, blob_contents);
                    ftns.push(ftn);
                }
            }
        }
        let new_ft = FileTree {
            values: ftns,
            children: None,
            blobs: None,
        };
        let hash = new_ft.to_hash();
        trees.push(new_ft);
        FileTreeNode {
            object_type: ObjectType::Tree,
            hash,
            filepath: self.dir_name.clone(),
            metadata: FileMetadata::default(),
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
    pub fn from_file_tree(_ft: FileTree) -> Result<Self, std::io::Error> {
        todo!("filetree -> gitrstree");
    }
}

/// Nodes in the tree structure.
/// Since the tree represents a file-system, a node can be either
/// A) a file with a link to its content, or
/// B) a subdirectory (or sub-tree) with links to other TreeNodes.
#[derive(Debug, PartialEq, Clone)]
pub enum TreeNode {
    File {
        /// Filename
        name: OsString,

        /// Hash value of the file's content as a BLOB
        content: String,

        /// metadata
        metadata: FileMetadata,
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
            let f_content = file_content(&entry.path()).unwrap();
            let metadata = {
                let fmd = entry.metadata().unwrap();
                FileMetadata::new(fmd.modified().unwrap(), fmd.len())
            };
            TreeNode::File {
                name: entry.file_name(),
                content: f_content,
                metadata,
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
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileTree {
    /// Immediate children.
    /// Only chained by hash value
    pub values: Vec<FileTreeNode>,

    /// In memory version of all of the FT's children
    pub children: Option<Vec<FileTree>>,

    /// Reference to Blob contents by their hashes
    /// Only stored in root
    pub blobs: Option<HashMap<CommitHash, String>>,
}

impl FileTree {
    pub const FT_NODE_SEPARATOR: u8 = b'\n';

    /// Builds a tree from an existing directory.
    /// Assumes that gitrs/objects/ is empty (no existing object files).
    pub fn build_initial(path: &Path) -> Result<Self, std::io::Error> {
        let gitrs_tree = GitrsTree::build_tree(path)?;
        let (_, mut fts, blob_contents) = gitrs_tree.to_file_trees();
        let mut root = fts.pop().unwrap();

        // set values for root only
        root.blobs = Some(blob_contents);
        root.children = Some(fts);
        Ok(root)
    }

    /// Compute a filetree's hash
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

    /// Takes a hash and tries to read the ft from file.
    /// File is identified by the hash.
    pub fn from_hash(h: CommitHash) -> Result<Self, String> {
        let hash_str = h.to_str();

        let mut path = PathBuf::from(BASE_DIR_NAME);
        path.extend([OBJECTS_DIR, &hash_str]);

        if !path.exists() {
            return Err(format!(
                "Cannot build FileTree from {hash_str}: Path {:?} does not exist.",
                path
            ));
        }

        let mut bytes: Vec<u8> = vec![];
        std::fs::File::open(path)
            .unwrap()
            .read_to_end(&mut bytes)
            .unwrap();
        Ok(Self::from_bytes(bytes).unwrap())
    }

    /// | FileTreeNode 0 | FT_NODE_SEPARATOR (1 byte) | FileTreeNode 1 | FT_NODE_SEPARATOR | ... | FT_NODE_SEPARATOR | FileTreeNode n |
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = self
            .values
            .iter()
            .flat_map(|node| {
                let mut bytes = node.to_bytes();
                bytes.push(Self::FT_NODE_SEPARATOR); // newline as node separator
                bytes
            })
            .collect::<Vec<u8>>();
        debug!("FT bytes: {:?}", bytes);
        bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, std::io::Error> {
        debug!("FT from bytes: {:?}", bytes);
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
        Ok(Self {
            values,
            children: None,
            blobs: None,
        })
    }

    /// Flatten a FileTree into an Index
    /// Used for comparing two indices
    pub fn to_index(&self) -> Index {
        todo!()
    }
}

impl std::fmt::Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .values
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileTreeNode {
    // /// file/dir permissions
    // permissions: u32,
    /// File (blob) or Subdir
    pub object_type: ObjectType,

    /// The file or subdir hashed
    pub hash: CommitHash,

    /// File or dir name
    pub filepath: OsString,

    /// Metadata of dir or file
    /// Can be omitted for testing
    pub metadata: FileMetadata,
}

impl FileTreeNode {
    /// | object_type (1 byte) | metadata (20 bytes) | hash (8 bytes) | filename (n bytes; until EOF)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![self.object_type.to_u8()];
        bytes.extend(self.metadata.to_bytes());
        bytes.extend(self.hash.to_bytes());
        bytes.extend(self.filepath.to_str().unwrap().bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        debug!("Node from bytes: {:?}", bytes);
        let object_type = ObjectType::from_u8(bytes[0]).unwrap();
        let mut idx = 1;
        let metadata = {
            let md = FileMetadata::from_bytes(&bytes[idx..FileMetadata::BYTE_LEN + idx]).unwrap();
            idx += FileMetadata::BYTE_LEN;
            md
        };
        let hash = {
            let h =
                CommitHash::from_bytes(&bytes[idx..CommitHash::HASH_LEN + idx].try_into().unwrap());
            idx += CommitHash::HASH_LEN;
            h
        };
        let filepath: OsString = str::from_utf8(&bytes[idx..]).unwrap().to_string().into();
        Ok(Self {
            object_type,
            hash,
            filepath,
            metadata,
        })
    }

    /// Build Index entry
    pub fn to_index_entry(&self) -> Option<IndexTreeEntry> {
        todo!()
        // self.object_type == ObjectType::Blob {
        //     return Some(IndexTreeEntry::new(IndexTreeMetadata::new())
        // }
    }
}

impl std::fmt::Display for FileTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.object_type, self.hash, self.filepath)
    }
}

#[test]
fn file_tree_node_serialization() {
    let ft_nodes = [
        FileTreeNode {
            object_type: ObjectType::Tree,
            hash: CommitHash::new("a tree"),
            filepath: OsString::from("subdir"),
            metadata: FileMetadata::default(),
        },
        FileTreeNode {
            object_type: ObjectType::Blob,
            hash: CommitHash::new("a blob"),
            filepath: OsString::from("file.txt"),
            metadata: FileMetadata::default(),
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
            filepath: OsString::from("subdir"),
            metadata: FileMetadata::default(),
        },
        FileTreeNode {
            object_type: ObjectType::Blob,
            hash: CommitHash::new("a blob"),
            filepath: OsString::from("file.txt"),
            metadata: FileMetadata::default(),
        },
    ];
    let ft = FileTree {
        values: ft_nodes,
        children: None,
        blobs: None,
    };
    let bytes = ft.to_bytes();
    let ft_new = FileTree::from_bytes(bytes).expect("Failed to convert: bytes -> FT");
    assert_eq!(ft, ft_new);
}
