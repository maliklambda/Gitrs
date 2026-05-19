pub mod cat_file;
pub mod commit;
pub mod file;
pub mod tree;
pub mod write_object;

use std::os::unix::ffi::OsStrExt;

use crate::internals::objects::{commit::Commit, file::FileContent, tree::GitrsTree};

/// Object enum without any values associated with a type.
/// Must have the same variants as struct Object.
/// Check out the -t flag at https://git-scm.com/docs/git-hash-object
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Commit,
    Blob,
    Tree,
}

impl ObjectType {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Commit => 0,
            Self::Blob => 1,
            Self::Tree => 2,
        }
    }

    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Commit),
            1 => Some(Self::Blob),
            2 => Some(Self::Tree),
            _ => None,
        }
    }
}

/// All objects are stored in the objects dir
/// An object must always be hashable
#[derive(Debug, PartialEq)]
pub enum Object {
    /// contents of a file
    Blob(FileContent),

    /// A snapshot of the working tree
    Commit(Commit),

    /// a directory listing
    Tree(GitrsTree),
}

impl Object {
    /// Convert an object to be 'file-writable'
    /// Structure:
    ///     | ObjectType (1 byte) | Content (based on ObjectType) |
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![self.to_object_type().to_u8()];
        match self {
            Object::Blob(fc) => {
                // Structure of Blob content:
                // | filename | \0 (to signal end of filename) | file content |
                let fname: String = fc
                    .fname
                    .to_owned()
                    .into_string()
                    .expect("OS String into String");
                let name = fname.as_bytes();
                v.extend(name);
                v.push(b'\0'); // \0 to signale end of filename
                v.extend(fc.content.as_bytes()); // TODO: compress file content
                v
            }
            _ => todo!("Object {:?} -> bytes", self),
        }
    }

    /// Serialize an object to bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        match ObjectType::from_u8(*bytes.first()?)? {
            ObjectType::Blob => {
                let (fname, idx) = {
                    let i = bytes.iter().position(|b| *b == b'\0')?;
                    let s = String::from_utf8(bytes[1..i].to_vec()).unwrap();
                    (s, i + 1)
                };
                let content = String::from_utf8(bytes[idx..].to_vec()).unwrap();
                Some(Self::Blob(FileContent {
                    fname: fname.into(),
                    content,
                }))
            }
            _ => todo!("bytes -> Object"),
        }
    }

    /// Convert an object to bytes and return its size
    pub fn size(&self) -> usize {
        self.to_bytes().len()
    }

    /// Returns the corresponding object type without a value associated with it.
    /// This is usefull for e.g. converting it to a u8 for binary storage.
    pub fn to_object_type(&self) -> ObjectType {
        match self {
            Object::Blob(_) => ObjectType::Blob,
            Object::Commit(_) => ObjectType::Commit,
            Object::Tree(_) => ObjectType::Tree,
        }
    }

    /// Returns pretty printed version of the object's content
    pub fn content(&self) -> String {
        match self {
            Object::Blob(fc) => {
                fc.content.to_string()
            }
            _ => todo!("Pretty print content for {:?}", self)
        }
    }

}

#[test]
fn object_serialization() {
    let obj_blob = Object::Blob(FileContent {
        fname: "hello.c".into(),
        content: "void main(){return 0;}".to_string(),
    });
    let bytes = obj_blob.to_bytes();
    let obj_blob_new = Object::from_bytes(bytes).unwrap();
    assert_eq!(obj_blob, obj_blob_new);
}
