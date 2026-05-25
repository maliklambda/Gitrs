use std::{ffi::OsString, fs::File, io::Read, path::Path, time::SystemTime};

#[derive(Debug, PartialEq)]
pub struct FileContent {
    /// Metadata
    pub fname: OsString,

    /// File content as a string
    pub content: String,

    /// Metadata
    pub metadata: FileMetadata,
}

impl FileContent {
    pub fn from_file(fpath: &Path, mut f: File) -> Result<Self, std::io::Error> {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let metadata = FileMetadata::from_file(f);
        Ok(Self {
            fname: fpath.into(),
            content,
            metadata,
        })
    }

    pub fn new(fname: OsString, content: String, metadata: FileMetadata) -> Self {
        Self { fname, content, metadata }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileMetadata {
    last_modified: SystemTime,
    filesize: u64,
}

impl FileMetadata {
    pub const BYTE_LEN: usize = 16;

    pub fn default() -> Self {
        Self {
            last_modified: SystemTime::now(),
            filesize: 1234,
        }
    }

    pub fn from_file(f: File) -> Self {
        let fmd = f.metadata().unwrap();
        Self::new(fmd.modified().unwrap(), fmd.len())
    }

    pub fn new(last_modified: SystemTime, filesize: u64) -> Self {
        Self {
            last_modified,
            filesize,
        }
    }

    /// | last_modified (8b) | filesize (8b) |
    pub fn to_bytes(&self) -> &[u8] {
        &[0_u8; Self::BYTE_LEN]
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self::default())
    }
}
