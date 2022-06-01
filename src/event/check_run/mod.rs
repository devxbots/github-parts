//! Check run event
//!
//! Check run activity has occurred. The type of activity is specified in the `action` property of
//! the payload object.

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::check_run::CheckRun;
use crate::installation::Installation;
use crate::repository::Repository;

pub use self::action::Action;

mod action;

/// Check run event
///
/// Check run activity has occurred. The type of activity is specified in the `action` property of
/// the payload object. For more information, see the [check runs](https://docs.github.com/en/rest/reference/checks#runs)
/// REST API.
///
/// # Note
///
/// The Checks API only looks for pushes in the repository where the check suite or check run were
/// created. Pushes to a branch in a forked repository are not detected and return an empty
/// `pull_requests` array and a `null` value for `head_branch`.
///
/// # Availability
///
/// - Repository webhooks only receive payloads for the `created` and `completed` event types in a
///   repository
/// - Organization webhooks only receive payloads for the `created` and `completed` event types in
///   repositories
/// - GitHub Apps with the `checks:read` permission receive payloads for the `created` and
///   `completed` events that occur in the repository where the app is installed. The app must have
///   the `checks:write` permission to receive the `rerequested` and `requested_action` event types.
///   The `rerequested` and `requested_action` event type payloads are only sent to the GitHub App
///   being requested. GitHub Apps with the `checks:write` are automatically subscribed to this
///   webhook event.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, Getters)]
pub struct CheckRunEvent {
    /// The [`Action`] performed.
    #[getset(get = "pub")]
    action: Action,

    /// The [`CheckRun`].
    #[getset(get = "pub")]
    check_run: CheckRun,

    /// The [`Repository`] where the event occurred.
    #[getset(get = "pub")]
    repository: Repository,

    /// Webhook payloads contain the `organization` object when the webhook is configured for an
    /// organization or the event occurs from activity in a repository owned by an organization.
    #[getset(get = "pub")]
    organization: Option<Account>,

    /// The GitHub App installation. Webhook payloads contain the `installation` property when the
    /// event is configured for and sent to a GitHub App.
    #[getset(get = "pub")]
    installation: Option<Installation>,

    /// The user that triggered the event.
    #[getset(get = "pub")]
    sender: Account,
}

impl CheckRunEvent {
    /// Initializes a new check run event.
    pub fn new(
        action: Action,
        check_run: CheckRun,
        repository: Repository,
        organization: Option<Account>,
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
