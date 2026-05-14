use std::{ffi::OsString, fs::File, io::Read};

#[derive(Debug, PartialEq)]
pub struct FileContent {
    /// Metadata
    pub fname: OsString,

    /// File content as a string
    pub content: String,
}

impl FileContent {
    pub fn from_file(mut f: File) -> Result<Self, std::io::Error> {
        let fname = "";
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(Self {
            fname: fname.into(),
            content,
        })
    }
}
