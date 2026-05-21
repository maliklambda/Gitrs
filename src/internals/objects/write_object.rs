use std::{fs::File, io::Write, path::PathBuf};

use log::{debug, info};

use crate::{
    constants::{BASE_DIR_NAME, OBJECTS_DIR},
    internals::{
        hash::commit_hash::CommitHash,
        objects::{Object, tree::FileTree},
    },
};

pub fn write_object(obj: Object) -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(BASE_DIR_NAME);
    path.push(OBJECTS_DIR);

    match obj {
        Object::Blob(ref fc) => {
            let h = CommitHash::new(&fc.content);
            path.push(h.to_path_buf());
            info!("Writing blob to {:?}", path);
            if !path.exists() {
                let mut f = File::create_new(&path)?;
                f.write_all(&obj.to_bytes())?;
                info!("Wrote {:?} successfully to {:?}", obj, path);
            } else {
                info!("File {:?} already exists", path);
            }
            Ok(())
        }
        Object::Tree(ref tree) => {
            let (_, fts) = tree.to_file_trees();
            info!("Writing tree to {:?}", path);
            for ft in fts {
                debug!("Writing ft to file: {:?}", &ft);
                write_ft(ft, path.clone()).unwrap();
            }
            Ok(())
        }
        _ => todo!("Write object: {:?}", obj),
    }
}

/// Write a file node to file
/// Expects path to NOT contain the FileTree's hash, as it is appended below
fn write_ft(ft: FileTree, mut path: PathBuf) -> Result<(), std::io::Error> {
    path.push(ft.to_hash().to_str());
    if !path.exists() {
        let bytes = ft.to_bytes();
        let mut f = File::create_new(&path)?;
        f.write_all(&bytes)?;
        info!("Wrote {:?} successfully to {:?}", &ft, path);
    } else {
        info!("File {:?} already exists", path);
    }
    Ok(())
}
