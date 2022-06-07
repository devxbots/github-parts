//! GitHub organization
//!
//! Organizations have various unique properties such as a `login` and an `id` that are used to
//! identify and interact with them.

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::account::{AccountId, Login};

/// GitHub organization
///
/// Organizations on GitHub have a unique `id` that is used throughout GitHub's REST API to identify
/// them. They also have a `login`, which is a human-readable name that must be unique, but can be
/// changed by the owner.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct Organization {
    /// Returns the login of the organization.
    #[getset(get = "pub")]
    login: Login,

    /// Returns the id of the organization.
    #[getset(get_copy = "pub")]
    id: AccountId,
}

impl Organization {
    /// Initializes a new organization.
    pub fn new(login: Login, id: AccountId) -> Self {
        Self { login, id }
    }
}

impl Display for Organization {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.login.get())
    }
}

#[cfg(test)]
mod tests {
    use super::Organization;

    #[test]
    fn trait_deserialize() {
        let json = r#"
        {
            "login": "devxbots",
            "id": 104442885,
            "node_id": "O_kgDOBjmsBQ",
            "url": "https://api.github.com/orgs/devxbots",
            "repos_url": "https://api.github.com/orgs/devxbots/repos",
            "events_url": "https://api.github.com/orgs/devxbots/events",
            "hooks_url": "https://api.github.com/orgs/devxbots/hooks",
            "issues_url": "https://api.github.com/orgs/devxbots/issues",
            "members_url": "https://api.github.com/orgs/devxbots/members{/member}",
            "public_members_url": "https://api.github.com/orgs/devxbots/public_members{/member}",
            "avatar_url": "https://avatars.githubusercontent.com/u/104442885?v=4",
            "description": "We're here to make developers happier and more productive by automating the boring parts of programming"
        }
        "#;

        let organization: Organization = serde_json::from_str(json).unwrap();

        assert_eq!(104442885, organization.id().get());
        assert_eq!("devxbots", organization.login().get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Organization>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Organization>();
    }
}