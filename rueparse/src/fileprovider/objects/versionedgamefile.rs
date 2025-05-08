use crate::VersionContainer;

use super::GameFile;

#[derive(Debug)]
pub struct VersionedGameFile {
    pub game_file: GameFile,
    pub versions: VersionContainer,
}

impl VersionedGameFile {
    pub fn new(path: &str, size: i64, versions: VersionContainer) -> Self {
        Self {
            game_file: GameFile::new(path.to_string(), size),
            versions,
        }
    }
}
