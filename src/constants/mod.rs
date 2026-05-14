pub mod const_cmds;

pub mod special_chars {
    pub const CLI_FLAG_PREFIX_SHORT: &str = "-";
    pub const CLI_FLAG_PREFIX_LONG: &str = "--";
    pub const SPACE: char = ' ';
    pub const EQUAL: char = '=';
}

/// Hash that indicates that no parent exists
/// A commit that has a null hash as a parent is the root (or initial) commit
pub const NULL_HASH: &str = "NULL"; // TODO: change this to a real hash

/// Default option for the "main/master" branch
pub const DEFAULT_BRANCH: &str = "main";

/// Base dir, like ".git/" (should be changed to ".gitrs/")
pub const BASE_DIR_NAME: &str = "gitrs";

/// config file name
pub const CONFIG_FILE: &str = "gitrsconfig";

/// similar to git's .git/HEAD file. Tracks the latest commit
pub const HEAD_FILE: &str = "HEAD";

/// like .gitrs/refs. Tracks referencable hashes (tags, branch-heads, etc.)
pub const REFS_DIR: &str = "refs";

/// Subdir of .gitrs/refs; like ./git/refs/heads; keeps track of heads of local branches
pub const HEADS_DIR: &str = "heads";

/// Subdir of .gitrs/; Contains the objects database
/// (key value pairs, the key being a CommitHash and the value being an object)
pub const OBJECTS_DIR: &str = "objects";

/// Temporary folder that holds the content.
/// Makes it easier to track changes.
pub const CONTENT_DIR: &str = "gitrs_content";

/// Prefixed to the path held in .gitrs/HEAD
pub mod head_prefixes {
    pub const HEAD_PREFIX_LEN: usize = 3;
    pub const HEAD_REF_STR: &str = "ref";
    pub const HEAD_TAG_STR: &str = "tag";
    pub const HEAD_RMT_STR: &str = "rmt";
}

pub mod object_types {
    pub const BLOB_STR: &str = "blob";
    pub const TREE_STR: &str = "tree";
    pub const COMMIT_STR: &str = "commit";
}
