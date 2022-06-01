use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Repository name
///
/// Repositories on GitHub are uniquely identified by the combination of their `owner` and `name`.
/// The repository name can be any string, but it has to be unique for its owner. Repositories can
/// be renamed at any time.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct RepositoryName(String);

impl RepositoryName {
    /// Initializes a new repository name.
    pub fn new(repository_name: impl Into<String>) -> Self {
        Self(repository_name.into())
    }

    /// Returns a string representation of the repository name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use github_parts::repository::RepositoryName;
    ///
    /// let repository_name = RepositoryName::new("repository_name");
    /// assert_eq!("repository_name", repository_name.get());
    /// ```
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for RepositoryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryName;

    #[test]
    fn trait_display() {
        let repository_name = RepositoryName::new("repository_name");

        assert_eq!("repository_name", repository_name.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<RepositoryName>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RepositoryName>();
    }
}
