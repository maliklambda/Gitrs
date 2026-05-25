use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use log::info;

use crate::{
    constants::CONTENT_DIR,
    internals::{hash::commit_hash::CommitHash, objects::file::FileContent},
};

pub fn hash_blob(filepath: &str) -> Result<(CommitHash, FileContent), std::io::Error> {
    let mut path = PathBuf::from(CONTENT_DIR);
    path.push(filepath);
    info!("path: {:?}", path);
    let f = File::open(&path)?;
    let file_content = FileContent::from_file(&path, f)?;
    let content_hash = CommitHash::new(&file_content.content);
    Ok((content_hash, file_content))
}

pub fn file_content(filepath: &Path) -> Result<String, std::io::Error> {
    info!("Hashing file: {:?}", filepath);
    let mut buffer = String::new();
    File::open(filepath)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}
