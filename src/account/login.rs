use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Login or account name
///
/// Every account on GitHub is uniquely identified by its `login`, which is the name of the account.
/// Users can rename accounts, but the account's `id` and `node_id` always stays the same.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Login(String);

impl Login {
    /// Initializes a new login.
    pub fn new(login: impl Into<String>) -> Self {
        Self(login.into())
    }

    /// Returns a string representation of the login.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use github_parts::account::Login;
    ///
    /// let login = Login::new("login");
    /// assert_eq!("login", login.get());
    /// ```
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl Display for Login {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Login;

    #[test]
    fn trait_display() {
        let login = Login::new("login");

        assert_eq!("login", login.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Login>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Login>();
    }
}
