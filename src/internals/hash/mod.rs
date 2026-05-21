pub mod commit_hash;
pub mod hash_blob;
pub mod hash_tree;

use log::debug;

use crate::internals::{
    hash::{commit_hash::CommitHash, hash_blob::hash_blob, hash_tree::hash_tree},
    objects::{Object, ObjectType, write_object::write_object},
};

#[derive(Debug, PartialEq, Clone)]
pub struct HashObjectConfig<'a> {
    pub value: &'a str,
    pub flags: HashObjectFlags,
}

impl<'a> HashObjectConfig<'a> {
    pub fn new(value: &'a str, flags: HashObjectFlags) -> Self {
        Self { value, flags }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HashObjectFlags {
    pub tp: ObjectType,
    pub write: bool,
}

impl HashObjectFlags {
    pub fn new(tp: ObjectType, write: bool) -> Self {
        Self { tp, write }
    }
}

pub fn hash_object(
    HashObjectConfig { value, flags }: HashObjectConfig,
) -> Result<CommitHash, std::io::Error> {
    match flags.tp {
        ObjectType::Blob => {
            let (h, fc) = hash_blob(value).unwrap();
            if flags.write {
                write_object(Object::Blob(fc))?;
            }
            Ok(h)
        }
        ObjectType::Tree => {
            let (h, t) = hash_tree(value).unwrap();

            // for debug
            let (_, fs) = t.to_file_trees();
            for st in fs {
                debug!("file Tree: {:?}", st);
            }
            // end debug

            if flags.write {
                write_object(Object::Tree(t.clone()))?;
            }
            Ok(h)
        }
        _ => todo!("Hash object other than blob"),
    }
}
