use crate::internals::{
    hash::{commit_hash::CommitHash, hash_blob::hash_blob},
    objects::{Object, ObjectType, write_object::write_object},
};

pub mod commit_hash;
pub mod hash_blob;

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
        _ => todo!("Hash object other than blob"),
    }
}
