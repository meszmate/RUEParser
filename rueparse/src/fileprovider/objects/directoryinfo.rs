use std::fs;
use std::time::SystemTime;

#[derive(Debug)]
pub struct DirectoryInfo {
    pub path: String,
    pub creation_time: Option<SystemTime>,
    pub last_write_time: Option<SystemTime>,
    pub is_empty: bool,
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

impl DirectoryInfo {
    pub fn new(directory_path: &str) -> std::io::Result<Self> {
        let metadata = fs::metadata(directory_path)?;

        let creation_time = metadata.created().ok();
        let last_write_time = metadata.modified().ok();
        let mut files = Vec::new();
        let mut directories = Vec::new();

        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                let entry = entry?;
                let entry_name = entry.file_name().into_string().unwrap_or_default();
                let entry_metadata = entry.metadata()?;

                if entry_metadata.is_file() {
                    files.push(entry_name);
                } else if entry_metadata.is_dir() {
                    directories.push(entry_name);
                }
            }
        }

        let is_empty = files.is_empty() && directories.is_empty();

        Ok(DirectoryInfo {
            path: directory_path.to_string(),
            creation_time,
            last_write_time,
            is_empty,
            files,
            directories,
        })
    }
    pub fn get_file() {}
}
