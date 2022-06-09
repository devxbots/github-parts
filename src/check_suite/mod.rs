use std::fmt::{Display, Formatter};

use getset::CopyGetters;
use serde::{Deserialize, Serialize};

use crate::check_run::{CheckRunConclusion, CheckRunStatus};
use crate::id;

id!(CheckSuiteId);

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters,
)]
pub struct CheckSuite {
    #[getset(get_copy = "pub")]
    id: CheckSuiteId,

    #[getset(get_copy = "pub")]
    status: CheckRunStatus,

    #[getset(get_copy = "pub")]
    conclusion: Option<CheckRunConclusion>,

    #[getset(get_copy = "pub")]
    latest_check_runs_count: Option<u64>,
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
