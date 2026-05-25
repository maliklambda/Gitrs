use std::{ffi::OsString, fs::File, io::Read, path::Path, time::{Duration, SystemTime, UNIX_EPOCH}};

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
    pub const BYTE_LEN: usize = 20;

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

    /// Serializes the struct into a fixed-size 20-byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_LEN] {
        let mut bytes = [0u8; Self::BYTE_LEN];

        // 1. Convert SystemTime to duration since UNIX_EPOCH
        let duration = self.last_modified
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards or file predates 1970");

        let secs = duration.as_secs();        // 8 bytes
        let nanos = duration.subsec_nanos();  // 4 bytes

        // 2. Write data using Little-Endian format
        bytes[0..8].copy_from_slice(&secs.to_le_bytes());
        bytes[8..12].copy_from_slice(&nanos.to_le_bytes());
        bytes[12..20].copy_from_slice(&self.filesize.to_le_bytes());

        bytes
    }

    /// Deserializes the struct from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < Self::BYTE_LEN {
            return Err("Byte slice is too short; expected at least 20 bytes");
        }

        // 1. Read the bytes back into integers
        // We use try_into().unwrap() because we already verified the slice length above
        let secs = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let nanos = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let filesize = u64::from_le_bytes(bytes[12..20].try_into().unwrap());

        // 2. Reconstruct the SystemTime
        let last_modified = UNIX_EPOCH + Duration::new(secs, nanos);

        Ok(FileMetadata {
            last_modified,
            filesize,
        })
    }
}
