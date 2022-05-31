use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Account id
///
/// GitHub assigns a unique `id` to each account that cannot be changed. The `id` is used to
/// interact with resources in GitHub's REST API.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct AccountId(u64);

impl AccountId {
    /// Initializes a new account id.
    pub fn new(account_id: u64) -> Self {
        Self(account_id)
    }

    /// Returns the account id.
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::AccountId;

    #[test]
    fn trait_display() {
        let id = AccountId::new(1);

        assert_eq!("1", id.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AccountId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AccountId>();
    }
}
