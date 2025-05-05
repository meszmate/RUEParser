use crate::compression::CompressionMethod;

#[derive(Debug)]
pub struct GameFile {
    path: String,
    is_encrypted: bool,
    compression_method: CompressionMethod,
    size: i64,
}

static UE_PACKAGE_EXTENSIONS: &[&str] = &["uasset", "umap"];
static UE_PACKAGE_PAYLOAD_EXTENSIONS: &[&str] = &["exp", "ubulk", "uptnl"];
static UE_KNOWN_EXTENSIONS: &[&str] = &[
    "uasset",
    "umap",
    "exp",
    "ubulk",
    "uptnl",
    "bin",
    "ini",
    "uplugin",
    "upluginmanifest",
    "locres",
    "locmeta",
];

impl GameFile {
    pub fn new(path: String, size: i64) -> Self {
        Self {
            path,
            size,
            is_encrypted: false,
            compression_method: CompressionMethod::Unknown,
        }
    }
    pub fn extension(&mut self) -> String {
        self.path.rsplit('.').next().unwrap_or("").to_string()
    }
    pub fn is_ue_package(&mut self) -> bool {
        UE_PACKAGE_EXTENSIONS
            .iter()
            .any(|&ext| ext.eq_ignore_ascii_case(&self.extension()))
    }
    pub fn is_ue_package_payload(&mut self) -> bool {
        UE_PACKAGE_PAYLOAD_EXTENSIONS
            .iter()
            .any(|&ext| ext.eq_ignore_ascii_case(&self.extension()))
    }
}
