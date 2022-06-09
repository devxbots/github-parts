use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::account::{Account, Organization};
use crate::check_run::CheckRun;
use crate::installation::Installation;
use crate::repository::Repository;

pub use self::action::Action;

mod action;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, Getters)]
pub struct CheckRunEvent {
    #[getset(get = "pub")]
    action: Action,

    #[getset(get = "pub")]
    check_run: CheckRun,

    #[getset(get = "pub")]
    repository: Repository,

    #[getset(get = "pub")]
    organization: Option<Organization>,

    #[getset(get = "pub")]
    installation: Option<Installation>,

    #[getset(get = "pub")]
    sender: Account,
}

impl CheckRunEvent {
    pub fn new(
        action: Action,
        check_run: CheckRun,
        repository: Repository,
        organization: Option<Organization>,
        installation: Option<Installation>,
        sender: Account,
    ) -> Self {
        Self {
            action,
            check_run,
            repository,
            organization,
            installation,
            sender,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use super::{Action, CheckRunEvent};

    #[test]
    fn trait_deserialize() {
        let fixture = format!(
            "{}/tests/fixtures/check_run.created.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let body = read(fixture).unwrap();

        let event: CheckRunEvent = serde_json::from_slice(&body).unwrap();

        assert!(matches!(event.action, Action::Created));
        assert_eq!(128620228, event.check_run().id().get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunEvent>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunEvent>();
    }
}
