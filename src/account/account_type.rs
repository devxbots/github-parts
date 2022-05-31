use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// GitHub account type
///
/// Accounts can represent either an organization or a user.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum AccountType {
    /// Organization account
    Organization,

    /// User account
    User,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            AccountType::Organization => "Organization",
            AccountType::User => "User",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::AccountType;

    #[test]
    fn trait_display() {
        assert_eq!("Organization", AccountType::Organization.to_string());
        assert_eq!("User", AccountType::User.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AccountType>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AccountType>();
    }
}
