use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// The action performed
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// A new check run was created.
    Created,

    /// The `status` of the check run is `completed`.
    Completed,

    /// Someone requested to re-run your check run from the pull request UI.
    ///
    /// See [About status checks](https://docs.github.com/en/articles/about-status-checks#checks)
    /// for more details about the GitHub UI. When you receive a `rerequested` action, you'll need
    /// to [create a new check run](https://docs.github.com/en/rest/reference/checks#create-a-check-run).
    /// Only the GitHub App that someone requests to re-run the check will receive the `rerequested`
    /// payload.
    Rerequested,

    /// Someone requested an action your app provides to be taken.
    ///
    /// Only the GitHub App someone requests to perform an action will receive the
    /// `requested_action` payload. To learn more about check runs and requested actions, see
    /// [Check runs and requested actions](https://docs.github.com/en/rest/reference/checks#check-runs-and-requested-actions).
    RequestedAction,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Action::Created => "created",
            Action::Completed => "completed",
            Action::Rerequested => "re-requested",
            Action::RequestedAction => "requested action",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::Action;

    #[test]
    fn trait_display() {
        assert_eq!("created", Action::Created.to_string());
        assert_eq!("completed", Action::Completed.to_string());
        assert_eq!("re-requested", Action::Rerequested.to_string());
        assert_eq!("requested action", Action::RequestedAction.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Action>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Action>();
    }
}
