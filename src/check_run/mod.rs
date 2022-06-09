//! Check run
//!
//! When code is pushed to GitHub, apps and integrations can start check runs to perform arbitrary
//! tasks, e.g. run tests or perform static analysis.

use std::fmt::{Display, Formatter};

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::check_suite::CheckSuite;
use crate::{id, name};

pub use self::check_run_conclusion::CheckRunConclusion;
pub use self::check_run_status::CheckRunStatus;

mod check_run_conclusion;
mod check_run_status;

id!(CheckRunId);
name!(CheckRunName);

/// Check run
///
/// When code is pushed to GitHub, apps and integrations can start check runs to perform arbitrary
/// tasks, e.g. run tests or perform static analysis. These check runs end with a conclusion that
/// informs the user about the success or failure of the task.
///
/// The `conclusion` of a check run is only available when the check run has been completed.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct CheckRun {
    /// Returns the unique id of the check run.
    #[getset(get_copy = "pub")]
    id: CheckRunId,

    /// Returns the name of the check run.
    #[getset(get = "pub")]
    name: CheckRunName,

    /// Returns the check suite that the check run is a part of.
    #[getset(get = "pub")]
    check_suite: CheckSuite,

    /// Returns the status of the check run.
    #[getset(get_copy = "pub")]
    status: CheckRunStatus,

    /// Returns the conclusion of the check run.
    ///
    /// The conclusion is only set when the status of the check run is `completed`.
    #[getset(get_copy = "pub")]
    conclusion: Option<CheckRunConclusion>,
}

#[cfg(test)]
mod tests {
    use super::CheckRun;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRun>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRun>();
    }
}
