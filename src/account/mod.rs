//! GitHub account
//!
//! Repositories hosted on GitHub belong to an account, which can be either an organization or a
//! user. Accounts have various unique properties such as a `login` and an `id` that are used to
//! identify and interact with them.

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub use self::account_id::AccountId;
pub use self::account_type::AccountType;
pub use self::login::Login;

mod account_id;
mod account_type;
mod login;

/// GitHub account
///
/// An account on GitHub can represent either an organization or a user. Accounts have a unique `id`
/// that is used throughout GitHub's REST API to identify accounts. They also have a `login`, which
/// is a human-readable name that must be unique, but can be changed by the owner.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct Account {
    /// Returns the login of the account.
    #[getset(get = "pub")]
    login: Login,

    /// Returns the id of the account.
    #[getset(get_copy = "pub")]
    id: AccountId,

    /// Returns the type of the account.
    #[getset(get_copy = "pub")]
    #[serde(rename = "type")]
    account_type: AccountType,
}

impl Account {
    /// Initializes a new account.
    pub fn new(login: Login, id: AccountId, account_type: AccountType) -> Self {
        Self {
            login,
            id,
            account_type,
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.login.get())
    }
}

#[cfg(test)]
mod tests {
    use super::{Account, AccountType};
    use crate::account::{AccountId, Login};

    #[test]
    fn trait_deserialize() {
        let json = r#"
        {
            "login": "octocat",
            "id": 1,
            "node_id": "MDQ6VXNlcjE=",
            "avatar_url": "https://github.com/images/error/octocat_happy.gif",
            "gravatar_id": "",
            "url": "https://api.github.com/users/octocat",
            "html_url": "https://github.com/octocat",
            "followers_url": "https://api.github.com/users/octocat/followers",
            "following_url": "https://api.github.com/users/octocat/following{/other_user}",
            "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
            "organizations_url": "https://api.github.com/users/octocat/orgs",
            "repos_url": "https://api.github.com/users/octocat/repos",
            "events_url": "https://api.github.com/users/octocat/events{/privacy}",
            "received_events_url": "https://api.github.com/users/octocat/received_events",
            "type": "User",
            "site_admin": false
        }
        "#;

        let account: Account = serde_json::from_str(json).unwrap();

        assert_eq!(1, account.id().get());
        assert_eq!("octocat", account.login().get());
        assert!(matches!(account.account_type(), AccountType::User));
    }

    #[test]
    fn trait_display() {
        let account = Account::new(
            Login::new("devxbots"),
            AccountId::new(104442885),
            AccountType::Organization,
        );

        assert_eq!("devxbots", account.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Account>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Account>();
    }
}
