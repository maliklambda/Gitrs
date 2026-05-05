use std::{fs::File, path::{Path, PathBuf}};

use log::info;

use crate::{
    command::Command,
    constants::{BASE_DIR_NAME, CONFIG_FILE},
    internals::{branch::Branch, commit::Commit, stage::Stage},
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

    ///
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

    pub fn init_new(init_cmd: Command<'_>) -> Result<(), String> {
        let base_dir = Path::new(BASE_DIR_NAME);
        if std::path::Path::exists(base_dir) {
            return Err(String::from(
                "Current directory is already a gitrs repository.",
            ));
        }

        if let Command::Init { default_branch } = init_cmd {
            std::fs::create_dir(base_dir).map_err(|err| format!("FS-io err: {err}"))?;
            let mut config_path = base_dir.to_path_buf();
            config_path.push(CONFIG_FILE);
            File::create_new(config_path).map_err(|err| format!("FS-io err: {err}"))?;
            info!("Initializing with default branch '{default_branch}'");
            todo!("init")
        }
        panic!(
            "Initialize called with command other than 'init': {:?}",
            init_cmd
        );
    }
}
