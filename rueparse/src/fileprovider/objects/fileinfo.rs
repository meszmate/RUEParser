use std::fs;
use std::io;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub is_read_only: bool,
}

impl FileInfo {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let metadata = fs::metadata(file_path)?;

        let size = metadata.len();
        let created = metadata.created().ok();
        let modified = metadata.modified().ok();
        let is_read_only = metadata.permissions().readonly();

        Ok(Self {
            path: file_path.to_string(),
            size,
            created,
            modified,
            is_read_only,
        })
    }
}
