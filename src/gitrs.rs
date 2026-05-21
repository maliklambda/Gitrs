use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

use log::{debug, info};

use crate::{
    command::Command,
    config::GitrsConfig,
    constants::{BASE_DIR_NAME, CONFIG_FILE, HEAD_FILE, HEADS_DIR, OBJECTS_DIR, REFS_DIR},
    execute::ExecuteError,
    internals::{
        branch::Branch,
        hash::commit_hash::CommitHash,
        head::{HeadPrefix, read_head_path, write_head_path},
        objects::{Object, commit::Commit},
        stage::Stage,
    },
};

/// The main struct that holds all relevant configs.
/// It is initialized by reading the files from the .gitrs folder
/// (or creating them in case of the init-command).
#[derive(Debug)]
pub struct Gitrs<'a> {
    /// the currently selected commit
    head: Commit,

    /// All configurations live here
    config: GitrsConfig,

    /// Keep references to all known branches
    /// Note: necessary? Why not read it only when explicitly needed?
    branches: Vec<Branch>,

    default_branch: &'a Branch,

    /// Keep all staged changes
    stage: Stage,
    // /// Keep the path to the objects directory
    // /// This is done to speed up queries from the objects
    // objects_path: Path,
}

impl<'a> Gitrs<'a> {
    pub fn new(
        head: Commit,
        config: GitrsConfig,
        branches: Vec<Branch>,
        default_branch: &'a Branch,
        stage: Stage,
    ) -> Self {
        Gitrs {
            head,
            config,
            branches,
            default_branch,
            stage,
        }
    }

    /// Inits a new gitrs structure in a dir that is not yet a gitrs repository.
    pub fn init_new(init_cmd: Command<'_>) -> Result<(), ExecuteError> {
        let base_dir = Path::new(BASE_DIR_NAME);
        if std::path::Path::exists(base_dir) {
            return Err(ExecuteError::InitError {
                msg: String::from("Current directory is already a gitrs repository."),
            });
        }

        if let Command::Init { default_branch } = init_cmd {
            info!("Initializing with default branch '{default_branch}'");
            // base dir
            std::fs::create_dir(base_dir).map_err(|err| ExecuteError::InitError {
                msg: err.to_string(),
            })?;

            // config file
            let mut config_path = base_dir.to_path_buf();
            config_path.push(CONFIG_FILE);
            let config_file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .read(true)
                .open(config_path)
                .map_err(|err| ExecuteError::InitError {
                    msg: err.to_string(),
                })?;

            debug!("Config: {:?}", config_file);

            // refs dir
            let mut refs_heads_dir = base_dir.to_path_buf();
            refs_heads_dir.push(REFS_DIR);
            refs_heads_dir.push(HEADS_DIR);
            std::fs::create_dir_all(&refs_heads_dir).map_err(|err| ExecuteError::InitError {
                msg: err.to_string(),
            })?;

            // objects dir
            let mut objects_dir = base_dir.to_path_buf();
            objects_dir.push(OBJECTS_DIR);
            std::fs::create_dir_all(&objects_dir).map_err(|err| ExecuteError::InitError {
                msg: err.to_string(),
            })?;

            // default branch file
            let mut default_branch_path = refs_heads_dir;
            default_branch_path.push(default_branch);
            File::create_new(&default_branch_path).map_err(|err| ExecuteError::InitError {
                msg: err.to_string(),
            })?;

            // HEAD file that points to refs/heads/<default_branch>
            let mut head_path = base_dir.to_path_buf();
            head_path.push(HEAD_FILE);
            let mut header_file = File::create_new(head_path).unwrap();
            let default_branch_path_relative = default_branch_path.strip_prefix(base_dir).unwrap();
            write_head_path(
                &mut header_file,
                HeadPrefix::Ref,
                default_branch_path_relative,
            );

            return Ok(());
        }
        panic!(
            "Initialize called with command other than 'init': {:?}",
            init_cmd
        );
    }

    /// Inits a new gitrs structure from an existing gitrs project.
    /// TODO: Implement gitrs_cache to not have to read all files every time.
    pub fn init_existing() -> Result<Self, ExecuteError> {
        let base_dir = Path::new(BASE_DIR_NAME);
        if !std::path::Path::exists(base_dir) {
            return Err(ExecuteError::InitError {
                msg: String::from(
                    "Current dir is NOT a gitrs repository. Trying to init an existing repo, but none found.",
                ),
            });
        }
        let head_commit = read_head_commit().unwrap();

        todo!();
    }

    /// Finds an object by its hash
    /// Object must be written to the objects/ dir
    pub fn find_object_by_hash(h: CommitHash) -> Result<Object, std::io::Error> {
        let mut path = PathBuf::from(BASE_DIR_NAME);
        path.extend([OBJECTS_DIR, &h.to_string()]);
        let bytes = {
            let mut buf = vec![];
            File::open(&path)?.read_to_end(&mut buf)?;
            debug!("Read {} byts from {:?}", buf.len(), &path);
            buf
        };
        Ok(Object::from_bytes(bytes).expect("Invalid Object conversion"))
    }
}

fn read_head_commit() -> Result<Commit, std::io::Error> {
    let hc = read_head_path();
    todo!()
}
