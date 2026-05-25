use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use log::{debug, info};

use crate::{
    constants::{BASE_DIR_NAME, OBJECTS_DIR},
    internals::{
        hash::commit_hash::CommitHash,
        objects::{Object, ObjectType, file::FileContent, tree::FileTree},
    },
};

/// Writes an object to file.
/// In case of Object::Tree(tree), root of tree is expected to contain a value for blob_contents.
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
            write_ft_root(ft, path.clone()).unwrap();
            // // write all subtrees
            // for child in ft.children.as_ref().unwrap() {
            //     write_ft(child, path.clone(), &blobs).unwrap();
            // }
            Ok(())
        }
        _ => todo!("Write object: {:?}", obj),
    }
}

/// Writes an entire file tree from its root.
/// Expects root to have a value for both children and blobs.
fn write_ft_root(root: &FileTree, path: PathBuf) -> Result<(), std::io::Error> {
    let blobs = root.blobs.clone().expect(
        "FileTree needs to have a value for blob-contents when writing a tree object to file",
    );
    write_ft(root, path.clone(), &blobs).unwrap();
    // write all subtrees
    for child in root.children.as_ref().unwrap() {
        write_ft(child, path.clone(), &blobs).unwrap();
    }
    Ok(())
}

/// Write a file node to file
/// Expects path to NOT contain the FileTree's hash, as it is appended below
fn write_ft(
    ft: &FileTree,
    mut path: PathBuf,
    blobs: &HashMap<CommitHash, String>,
) -> Result<(), std::io::Error> {
    // write self
    path.push(ft.to_hash().to_str());
    if !path.exists() {
        let obj_bytes = Object::Tree(ft.clone()).to_bytes();
        let mut f = File::create_new(&path)?;
        f.write_all(&obj_bytes)?;
        debug!("Bytes: {:?}", obj_bytes);
        info!("Wrote {:?} successfully to {:?}", &ft, path);
    } else {
        info!("File {:?} already exists", path);
    }

    // iter over immediate children and filter for its blobs.
    // Then write those blobs.
    for b in ft
        .values
        .iter()
        .filter(|v| v.object_type == ObjectType::Blob)
    {
        let content = blobs
            .get(&b.hash)
            .expect(&format!("Did not find hash {:?} in blobs.", b.hash));
        write_object(Object::Blob(FileContent::new(
            b.filepath.clone(),
            content.to_owned(),
            b.metadata.clone(),
        )))?;
    }
    Ok(())
}
