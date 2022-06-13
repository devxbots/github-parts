use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::visibility::Visibility;
use crate::{id, name};

id!(RepositoryId);
name!(RepositoryName);

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct Repository {
    #[getset(get_copy = "pub")]
    id: RepositoryId,

    #[getset(get = "pub")]
    name: RepositoryName,

    #[getset(get = "pub")]
    description: Option<String>,

    #[getset(get = "pub")]
    owner: Account,

    #[getset(get_copy = "pub")]
    visibility: Visibility,
}

impl Repository {
    pub fn new(
        id: RepositoryId,
        name: RepositoryName,
        description: Option<String>,
        owner: Account,
        visibility: Visibility,
    ) -> Self {
        Self {
            id,
            name,
            description,
            owner,
            visibility,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner.login(), self.name())
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

#[cfg(test)]
mod tests {
    use crate::account::{Account, AccountId, Login, Organization};
    use crate::repository::{RepositoryId, RepositoryName};
    use crate::visibility::Visibility;

    use super::Repository;

    fn repository() -> Repository {
        Repository::new(
            RepositoryId::new(496534847),
            RepositoryName::new("github-parts"),
            Some("🔩 Types and actions to interact with GitHub".into()),
            Account::Organization(Organization::new(
                Login::new("devxbots"),
                AccountId::new(104442885),
            )),
            Visibility::Public,
        )
    }

    #[test]
    fn full_name() {
        assert_eq!("devxbots/github-parts", repository().full_name());
    }

    #[test]
    fn trait_display() {
        assert_eq!("devxbots/github-parts", repository().to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Repository>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Repository>();
    }
}
