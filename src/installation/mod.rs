//! Installation
//!
//! When a GitHub App is added to a repository, it's called an installation.

use std::fmt::{Display, Formatter};

use getset::CopyGetters;
use serde::{Deserialize, Serialize};

id!(InstallationId);

/// Installation
///
/// When a GitHub App is added to a repository, it's called an installation. Each installation has a
/// unique `id` that can be used by the GitHub App to authenticate and interact with a specific
/// repository.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters,
)]
pub struct Installation {
    /// Returns the installation id.
    #[getset(get_copy = "pub")]
    id: InstallationId,
}

impl Installation {
    /// Initializes a new installation.
    pub fn new(id: InstallationId) -> Self {
        Self { id }
    }
}

impl Display for Installation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{Installation, InstallationId};

    #[test]
    fn trait_display() {
        let installation = Installation::new(InstallationId::new(1));

        assert_eq!("1", installation.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Installation>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Installation>();
    }
}
