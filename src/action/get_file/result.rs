use std::path::PathBuf;

use derive_new::new;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Deserialize,
    Serialize,
    CopyGetters,
    Getters,
    new,
)]
pub struct GetFileResult {
    #[getset(get_copy = "pub")]
    pub(super) size: u64,

    #[getset(get = "pub")]
    pub(super) name: String,

    #[getset(get = "pub")]
    pub(super) path: PathBuf,

    #[getset(get = "pub")]
    pub(super) content: Vec<u8>,

    #[getset(get = "pub")]
    pub(super) sha: String,

    #[getset(get = "pub")]
    pub(super) html_url: Url,
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
