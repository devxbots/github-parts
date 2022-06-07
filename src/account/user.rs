//! GitHub user
//!
//! Users have various unique properties such as a `login` and an `id` that are used to identify and
//! interact with them.

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::account::{AccountId, Login};

/// GitHub user
///
/// Users on GitHub have a unique `id` that is used throughout GitHub's REST API to identify users.
/// They also have a `login`, which is a human-readable name that must be unique, but can be changed
/// by the owner.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct User {
    /// Returns the login of the user.
    #[getset(get = "pub")]
    login: Login,

    /// Returns the id of the user.
    #[getset(get_copy = "pub")]
    id: AccountId,
}

impl User {
    /// Initializes a new user.
    pub fn new(login: Login, id: AccountId) -> Self {
        Self { login, id }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.login.get())
    }
}

#[cfg(test)]
mod tests {
    use super::User;

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

        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(1, user.id().get());
        assert_eq!("octocat", user.login().get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<User>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<User>();
    }
}