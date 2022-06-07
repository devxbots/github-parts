use base64::DecodeError;
use thiserror::Error;

use crate::error::Error;

/// Errors that can occur when getting a file from GitHub
#[derive(Debug, Error)]
pub enum GetFileError {
    /// An invalid argument was passed to the action
    #[error("argument was invalid. {0}")]
    Argument(String),

    /// Authenticating with GitHub failed
    #[error("authentication failed. {0}")]
    Authentication(#[from] Error),

    /// Decoding the file's content failed
    #[error("decoding content failed. {0}")]
    Decoding(#[from] DecodeError),

    /// The user tried to get a directory
    #[error("path was a directory, but must be a file")]
    Directory,

    /// The file encoding is not supported by the crate
    #[error("encoding {0} is not supported")]
    Encoding(String),

    /// The outgoing request to GitHub failed
    #[error("querying the content failed. {0}")]
    Request(#[from] reqwest::Error),

    /// Deserializing the payload in the API response failed
    #[error("response could not be deserialized. {0}")]
    Response(#[from] serde_json::Error),

    /// The user tried to get a git submodule
    #[error("path was a submodule, but must be a file")]
    Submodule,

    /// The user tried to get a symlink
    // TODO: Follow symlinks and return the file
    #[error("path was a symlink, but must be a file")]
    Symlink,
}

#[cfg(test)]
mod tests {
    use super::GetFileError;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GetFileError>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GetFileError>();
    }
}
