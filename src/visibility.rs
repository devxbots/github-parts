//! Visibility
//!
//! Many resources on GitHub can either be public or private, which is called their visibility.
//! Private resources are only accessible to the owner, team members, or collaborators. Public
//! resources can be seen by anyone.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Visibility
///
/// Many resources on GitHub can either be public or private, which is called their visibility.
/// Private resources are only accessible to the owner, team members, or collaborators. Public
/// resources can be seen by anyone.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Visibility {
    /// Only visible to the owner, team members, and collaborators
    Private,

    /// Visible to everyone
    Public,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Visibility::Private => "private",
            Visibility::Public => "public",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::Visibility;

    #[test]
    fn trait_display() {
        assert_eq!("private", Visibility::Private.to_string());
        assert_eq!("public", Visibility::Public.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Visibility>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Visibility>();
    }
}
