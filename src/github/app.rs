use std::fmt::{Display, Formatter};

use derive_new::new;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::{id, name};

id!(AppId);
name!(AppName);

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
pub struct App {
    #[getset(get_copy = "pub")]
    id: AppId,

    #[getset(get = "pub")]
    name: AppName,

    #[getset(get = "pub")]
    owner: Account,
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<App>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<App>();
    }
}
