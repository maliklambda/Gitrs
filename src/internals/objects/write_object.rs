use std::{fs::File, io::Write, path::PathBuf};

use log::{debug, info};

use crate::{
    constants::{BASE_DIR_NAME, OBJECTS_DIR},
    internals::{
        hash::commit_hash::CommitHash,
        objects::{Object, ObjectType, tree::{FileTree, FileTreeNode}},
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
        Object::Tree(ref ft) => {
            info!("Writing tree to {:?}", path);
            write_ft(ft, path.clone()).unwrap();
            for child in ft.children.as_ref().unwrap() {
                write_ft(child, path.clone()).unwrap();
            }
            Ok(())
        }
        _ => todo!("Write object: {:?}", obj),
    }
}

/// Write a file node to file
/// Expects path to NOT contain the FileTree's hash, as it is appended below
fn write_ft(ft: &FileTree, mut path: PathBuf) -> Result<(), std::io::Error> {
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

fn write_ft_node(node: &FileTreeNode, mut path: PathBuf) -> Result<(), std::io::Error> {
    path.push(node.hash.to_str());
    if path.exists() {
        debug!("Unchanged hash ({}) for {:?}", node.hash.to_str(), node);
        return Ok(())
    }
    debug!("Writing node to file: {:?}", node);

    match node.object_type {
        ObjectType::Tree => {
            let ft = FileTree::from_hash(node.hash).unwrap();
            write_ft(&ft, path)
        }
        ObjectType::Blob => {
            todo!("Write filetreenode::blob");
        }
        ObjectType::Commit => todo!("handle commit in write ft node"),
    }
}


