use std::path::PathBuf;

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

/// Get file result
///
/// The `get_file` action returns a file object with some metadata and the file's contents as a
/// binary blob.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Default,
    Deserialize,
    Serialize,
    CopyGetters,
    Getters,
)]
pub struct GetFileResult {
    /// Returns the size of the file.
    #[getset(get_copy = "pub")]
    pub(super) size: u64,

    /// Returns the name of the file.
    #[getset(get = "pub")]
    pub(super) name: String,

    /// Returns the path of the file inside the repository.
    #[getset(get = "pub")]
    pub(super) path: PathBuf,

    /// Returns the file content.
    #[getset(get = "pub")]
    pub(super) content: Vec<u8>,

    /// Returns the SHA hash of the file.
    #[getset(get = "pub")]
    pub(super) sha: String,
}

#[cfg(test)]
mod tests {
    use super::GetFileResult;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GetFileResult>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GetFileResult>();
    }
}
