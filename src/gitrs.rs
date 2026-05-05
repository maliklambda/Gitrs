use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use log::{debug, info};

use crate::{
    command::Command,
    constants::{BASE_DIR_NAME, CONFIG_FILE, HEAD_FILE, HEADS_DIR, REFS_DIR},
    execute::ExecuteError,
    internals::{
        branch::Branch,
        commit::Commit,
        head::{HeadPrefix, write_head_path},
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

    /// Keep references to all known branches
    /// Note: necessary? Why not read it only when explicitly needed?
    branches: Vec<Branch>,

    default_branch: &'a Branch,

    /// Keep all staged changes
    stage: Stage,
}

impl<'a> Gitrs<'a> {
    pub fn new(
        head: Commit,
        branches: Vec<Branch>,
        default_branch: &'a Branch,
        stage: Stage,
    ) -> Self {
        Gitrs {
            head,
            branches,
            default_branch,
            stage,
        }
    }

    pub fn init_new(init_cmd: Command<'_>) -> Result<(), ExecuteError> {
        let base_dir = Path::new(BASE_DIR_NAME);
        if std::path::Path::exists(base_dir) {
            return Err(ExecuteError::InitError {
                msg: String::from("Current directory is already a gitrs repository.")
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
}
