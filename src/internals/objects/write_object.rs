use std::{fs::File, io::Write, path::PathBuf};

use log::info;

use crate::{
    constants::{BASE_DIR_NAME, OBJECTS_DIR},
    internals::{hash::commit_hash::CommitHash, objects::Object},
};

pub fn write_object(obj: Object) -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(BASE_DIR_NAME);
    path.push(OBJECTS_DIR);

    match obj {
        Object::Blob(fc) => {
            let h = CommitHash::new(&fc.content);
            path.push(h.to_path_buf());
            info!("Writing blob to {:?}", path);
            if !path.exists() {
                let mut f = File::create_new(&path)?;
                f.write_all(fc.content.as_bytes())?;
                info!("Wrote successfully to {:?}", path);
            } else {
                info!("File {:?} already exists", path);
            }
            Ok(())
        }
        _ => todo!("Write object: {:?}", obj),
    }
}
