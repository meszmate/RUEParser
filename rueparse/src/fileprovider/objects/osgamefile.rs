use std::path::Path;

use super::{DirectoryInfo, FileInfo, VersionedGameFile};
use crate::{VersionContainer, compression::CompressionMethod};

#[derive(Debug)]
pub struct OsGameFile {
    pub versioned_game_file: VersionedGameFile,

    pub actual_file: FileInfo,
    pub is_encrypted: bool,
    pub compression_method: CompressionMethod,
}

impl OsGameFile {
    pub fn new(
        base_dir: DirectoryInfo,
        info: FileInfo,
        mount_point: String,
        versions: VersionContainer,
    ) -> Self {
        let base_dir_len = base_dir.path.len() + 1;
        let infop = info.path.as_str();
        Self {
            versioned_game_file: VersionedGameFile::new(
                &(mount_point + &infop[base_dir_len..].replace("\\", "/")).as_str(),
                info.size as i64,
                versions,
            ),
            actual_file: info,
            is_encrypted: false,
            compression_method: CompressionMethod::None,
        }
    }
}
