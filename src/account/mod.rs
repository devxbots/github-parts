//! GitHub account
//!
//! Repositories hosted on GitHub belong to an account, which can be either an organization or a
//! user. Accounts have various unique properties such as a `login` and an `id` that are used to
//! identify and interact with them.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub use self::login::Login;
pub use self::organization::Organization;
pub use self::user::User;

mod login;
mod organization;
mod user;

id!(AccountId);

/// GitHub account
///
/// An account on GitHub can represent either an organization or a user. Accounts have a unique `id`
/// that is used throughout GitHub's REST API to identify accounts. They also have a `login`, which
/// is a human-readable name that must be unique, but can be changed by the owner.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Account {
    /// An organization
    Organization(Organization),

    /// A user
    User(User),
}

#[cfg(test)]
mod tests {
    use super::Account;

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
        assert!(matches!(account, Account::User(_)));
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
