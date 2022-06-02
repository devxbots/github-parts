//! Repository
//!
//! Repositories store source code using the Git version control system. They are the core resource
//! on GitHub, and almost everything else is organized around them.

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::visibility::Visibility;

pub use self::repository_name::RepositoryName;

mod repository_name;

id!(RepositoryId);

/// Repository
///
/// Repositories are uniquely identified by the combination of their `owner` and `name`. They have
/// a `id` that never changes, even if the repository is renamed, and a `visibility` that indicates
/// whether the repository is public or private.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct Repository {
    /// Returns the id of the repository.
    #[getset(get_copy = "pub")]
    id: RepositoryId,

    /// Returns the name of the repository.
    #[getset(get = "pub")]
    name: RepositoryName,

    /// Returns the description of the repository.
    #[getset(get = "pub")]
    description: Option<String>,

    /// Returns the owner of the repository.
    #[getset(get = "pub")]
    owner: Account,

    /// Returns the visibility of the repository.
    #[getset(get_copy = "pub")]
    visibility: Visibility,
}

impl Repository {
    /// Initializes a new repository.
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

    /// Returns the full name of the repository.
    ///
    /// The full name of a repository is the combination of the owner's login and the repository's
    /// name. This combination can be used to uniquely identify a repository on GitHub, and is thus
    /// often used to reference them externally.
    pub fn full_name(&self) -> String {
        let login = match &self.owner {
            Account::Organization(org) => org.login(),
            Account::User(user) => user.login(),
        };

        format!("{}/{}", login, self.name())
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
