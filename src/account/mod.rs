use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{id, name};

pub use self::bot::Bot;
pub use self::organization::Organization;
pub use self::user::User;

mod bot;
mod organization;
mod user;

id!(AccountId);
name!(Login);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Account {
    Bot(Bot),
    Organization(Organization),
    User(User),
}

impl Account {
    pub fn login(&self) -> &Login {
        match self {
            Account::Bot(bot) => bot.login(),
            Account::Organization(org) => org.login(),
            Account::User(user) => user.login(),
        }
    }
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
