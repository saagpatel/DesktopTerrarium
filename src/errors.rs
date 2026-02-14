use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerrariumError {
    #[error("failed to read state file at {path}: {source}")]
    StateReadFailed { path: String, source: std::io::Error },

    #[error("failed to parse state file: {source}")]
    StateDeserializeFailed { source: serde_json::Error },

    #[error("failed to write state file at {path}: {source}")]
    StateWriteFailed { path: String, source: std::io::Error },

    #[error("state file version {found} is newer than supported version {supported}")]
    UnsupportedStateVersion { found: u32, supported: u32 },

    #[error("failed to get system idle time: {0}")]
    IdleTimeQueryFailed(String),
}
