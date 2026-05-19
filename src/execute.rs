use std::path::{Path, PathBuf};

use log::info;

use crate::{
    cli::Token,
    command::Command,
    constants::CONTENT_DIR,
    gitrs::Gitrs,
    internals::{
        hash::{HashObjectConfig, hash_blob::hash_file_content, hash_object},
        objects::tree::GitrsTree,
    },
};

#[derive(Debug)]
pub enum ExecuteError {
    InitError { msg: String },
}

pub fn execute<'a>(cmd: Command<'a>) -> Result<(), ExecuteError> {
    info!("Executing command: {:?}", cmd);
    match cmd {
        Command::Init { default_branch } => {
            Gitrs::init_new(cmd).unwrap();
            info!("Initialization successfull. You are now on branch '{default_branch}'");
        }
        Command::Status => {
            let _gitrs = Gitrs::init_existing().unwrap();
            // gitrs.status(cmd);
        }
        Command::HashFile { filename } => {
            if let Token::TString(fname) = filename {
                let mut path = PathBuf::from(CONTENT_DIR);
                path.push(fname);
                let h = hash_file_content(&path).unwrap();
                info!("Hashed file {fname}: {:?}", h);
            }
        }
        Command::BuildTree => {
            let tree = GitrsTree::build_tree(Path::new(CONTENT_DIR)).unwrap();
            info!("Built tree: {:?}", tree);
        }
        Command::HashObject(ho_config) => {
            let h = hash_object(ho_config).unwrap();
            info!("hash: {h}");
            // todo!("Hash object: {value} of type {:?}; write = {:?}", tp, write);
        }
        _ => todo!("Execution for command: {:?}", cmd),
    }
    Ok(())
}
