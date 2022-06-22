use std::fmt::{Display, Formatter};

use derive_new::new;
use getset::CopyGetters;
use serde::{Deserialize, Serialize};

use crate::id;

id!(InstallationId);

#[derive(
    Copy,
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
    new,
)]
pub struct Installation {
    #[getset(get_copy = "pub")]
    id: InstallationId,
}

impl Display for Installation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{Installation, InstallationId};

    #[test]
    fn trait_display() {
        let installation = Installation::new(InstallationId::new(1));

        assert_eq!("1", installation.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Installation>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Installation>();
    }
}
