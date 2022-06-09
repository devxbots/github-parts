//! Check suite
//!
//! When code is pushed to GitHub, a new check suite is started. Apps and integrations can then
//! start check runs inside this suite, and GitHub will show them together in the UI.

use std::fmt::{Display, Formatter};

use getset::CopyGetters;
use serde::{Deserialize, Serialize};

use crate::check_run::{CheckRunConclusion, CheckRunStatus};
use crate::id;

id!(CheckSuiteId);

/// Check suite
///
/// When code is pushed to GitHub, a new check suite is started. Apps and integrations can then
/// start check runs inside this suite, and GitHub will show them together in the UI.
///
/// The `status` and `conclusion` of a check suite are derived from its check runs.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters,
)]
pub struct CheckSuite {
    /// Returns the check suite id.
    #[getset(get_copy = "pub")]
    id: CheckSuiteId,

    /// Returns the status of the check suite.
    #[getset(get_copy = "pub")]
    status: CheckRunStatus,

    /// Returns the conclusion of the check suite.
    #[getset(get_copy = "pub")]
    conclusion: CheckRunConclusion,

    /// Returns the latest number of check runs that are part of the suite.
    #[getset(get_copy = "pub")]
    latest_check_runs_count: u64,
}

#[cfg(test)]
mod tests {
    use super::CheckSuite;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckSuite>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckSuite>();
    }
}
