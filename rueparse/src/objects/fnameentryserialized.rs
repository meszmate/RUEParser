use std::collections::HashMap;

#[derive(Debug)]
pub struct FNameEntrySerialized {
    pub name: String,
    pubg_name_map: HashMap<String, String>,
}
