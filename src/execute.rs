use std::path::{Path, PathBuf};

use log::{debug, info};

use crate::{
    cli::Token,
    command::Command,
    constants::CONTENT_DIR,
    gitrs::Gitrs,
    internals::{
        hash::{commit_hash::CommitHash, hash_object},
        objects::{
            cat_file::CatFileMode,
            tree::FileTree,
        },
    },
};

#[derive(Debug)]
pub enum ExecuteError {
    /// Any error that occurs during Initialization
    InitError {
        msg: String,
    },

    NonExistingHash {
        hash: CommitHash,
    },
}

pub fn execute<'a>(cmd: Command<'a>) -> Result<(), ExecuteError> {
    info!("Executing command: {:?}", cmd);
    match cmd {
        Command::Init { default_branch } => {
            Gitrs::init_new(cmd)?;
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
                todo!("remove hash file command");
                // let h = hash_file_content(&path).unwrap();
                // info!("Hashed file {fname}: {:?}", h);
            }
        }
        Command::BuildTree => {
            // let tree = GitrsTree::build_tree(Path::new(CONTENT_DIR)).unwrap();
            let tree = FileTree::build_initial(Path::new(CONTENT_DIR)).unwrap();
            info!("Built tree: {:?}", tree);
        }
        Command::HashObject(ho_config) => {
            let h = hash_object(ho_config).unwrap();
            info!("hash: {h}");
        }
        Command::CatFile(cat_file_config) => {
            for value in cat_file_config.values {
                let obj = Gitrs::find_object_by_hash(value).map_err(|_| {
                    ExecuteError::NonExistingHash {
                        hash: value,
                    }
                })?;
                debug!("Returned object: {:?}", obj);
                match cat_file_config.flags {
                    CatFileMode::Type => println!("{:?}", obj.to_object_type()),
                    CatFileMode::Size => println!("{}", obj.size()),
                    CatFileMode::PrettyPrint => println!("{}", obj.content()),
                }
            }
        }
        _ => todo!("Execution for command: {:?}", cmd),
    }
    Ok(())
}
