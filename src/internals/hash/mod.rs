use crate::internals::{
    hash::{commit_hash::CommitHash, hash_blob::hash_blob},
    objects::{Object, ObjectType, write_object::write_object},
};

pub mod commit_hash;
pub mod hash_blob;

pub fn hash_object(tp: ObjectType, value: &str, write: bool) -> Result<CommitHash, std::io::Error> {
    match tp {
        ObjectType::Blob => {
            let (h, fc) = hash_blob(value).unwrap();
            if write {
                write_object(Object::Blob(fc))?;
            }
            Ok(h)
        }
        _ => todo!("Hash object other than blob"),
    }
}
