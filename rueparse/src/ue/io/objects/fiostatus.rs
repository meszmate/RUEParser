#[derive(Debug)]
pub enum EIoErrorCode {
    Ok,
    Unknown,
    InvalidCode,
    Cancelled,
    FileOpenFailed,
    FileNotOpen,
    ReadError,
    WriteError,
    NotFound,
    CorruptToc,
    UnknownChunkID,
    InvalidParameter,
    SignatureError,
    InvalidEncryptionKey,
}

#[derive(Debug)]
pub struct FIoStatus {
    pub error_code: EIoErrorCode,
    pub error_message: String,
}

impl FIoStatus {
    pub fn new(error_code: EIoErrorCode, error_message: String) -> Self {
        Self {
            error_code,
            error_message,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} ({:?})", self.error_message, self.error_code)
    }
}
