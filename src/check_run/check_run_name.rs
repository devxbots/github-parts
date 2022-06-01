use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Check run name
///
/// Check runs have a `name` that describes them and their purpose. The name is set by the app or
/// integration when the check run is created.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct CheckRunName(String);

impl CheckRunName {
    /// Initializes a new check run name.
    pub fn new(check_run_name: impl Into<String>) -> Self {
        Self(check_run_name.into())
    }

    /// Returns a string representation of the check run name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use github_parts::check_run::CheckRunName;
    ///
    /// let check_run_name = CheckRunName::new("check_run_name");
    /// assert_eq!("check_run_name", check_run_name.get());
    /// ```
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for CheckRunName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckRunName;

    #[test]
    fn trait_display() {
        let check_run_name = CheckRunName::new("check_run_name");

        assert_eq!("check_run_name", check_run_name.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunName>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunName>();
    }
}
