use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Installation id
///
/// When a GitHub App is added to a repository, it's called an installation. Each installation has a
/// unique `id` that can be used by the GitHub App to authenticate and interact with a specific
/// repository.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct InstallationId(u64);

impl InstallationId {
    /// Initializes a new installation id.
    pub fn new(installation_id: u64) -> Self {
        Self(installation_id)
    }

    /// Returns the installation id.
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Display for InstallationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::InstallationId;

    #[test]
    fn trait_display() {
        let id = InstallationId::new(1);

        assert_eq!("1", id.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<InstallationId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<InstallationId>();
    }
}
