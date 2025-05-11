use std::collections::HashMap;

use crate::{VersionContainer, models::FGuid};

pub struct CustomConfigIni {
    pub encryption_key_guid: Option<FGuid>,
}

#[derive(Debug)]
pub struct AbstractFileProvider {
    pub versions: VersionContainer,
    pub virtual_paths: HashMap<String, String>,
}
