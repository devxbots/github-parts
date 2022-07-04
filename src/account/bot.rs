use std::fmt::{Display, Formatter};

use derive_new::new;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::account::{AccountId, Login};

#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Deserialize,
    Serialize,
    CopyGetters,
    Getters,
    new,
)]
pub struct Bot {
    #[getset(get = "pub")]
    login: Login,

    #[getset(get_copy = "pub")]
    id: AccountId,
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.login.get())
    }
}

#[cfg(test)]
mod tests {
    use super::Bot;

    #[test]
    fn trait_deserialize() {
        let json = r#"
        {
            "login": "dependabot[bot]",
            "id": 49699333,
            "node_id": "MDM6Qm90NDk2OTkzMzM=",
            "avatar_url": "https://avatars.githubusercontent.com/in/29110?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/dependabot%5Bbot%5D",
            "html_url": "https://github.com/apps/dependabot",
            "followers_url": "https://api.github.com/users/dependabot%5Bbot%5D/followers",
            "following_url": "https://api.github.com/users/dependabot%5Bbot%5D/following{/other_user}",
            "gists_url": "https://api.github.com/users/dependabot%5Bbot%5D/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/dependabot%5Bbot%5D/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/dependabot%5Bbot%5D/subscriptions",
            "organizations_url": "https://api.github.com/users/dependabot%5Bbot%5D/orgs",
            "repos_url": "https://api.github.com/users/dependabot%5Bbot%5D/repos",
            "events_url": "https://api.github.com/users/dependabot%5Bbot%5D/events{/privacy}",
            "received_events_url": "https://api.github.com/users/dependabot%5Bbot%5D/received_events",
            "type": "Bot",
            "site_admin": false
        }
        "#;

        let bot: Bot = serde_json::from_str(json).unwrap();

        assert_eq!(49699333, bot.id().get());
        assert_eq!("dependabot[bot]", bot.login().get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Bot>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Bot>();
    }
}
