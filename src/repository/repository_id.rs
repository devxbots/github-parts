use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Repository id
///
/// GitHub assigns a unique `id` to each repository that cannot be changed. The `id` is used to
/// interact with resources in GitHub's REST API.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct RepositoryId(u64);

impl RepositoryId {
    /// Initializes a new repository id.
    pub fn new(repository_id: u64) -> Self {
        Self(repository_id)
    }

    /// Returns the repository id.
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Display for RepositoryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryId;

    #[test]
    fn trait_display() {
        let id = RepositoryId::new(1);

        assert_eq!("1", id.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<RepositoryId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RepositoryId>();
    }
}
