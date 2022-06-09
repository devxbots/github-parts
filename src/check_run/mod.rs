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

#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, CopyGetters, Getters,
)]
pub struct CheckRun {
    #[getset(get_copy = "pub")]
    id: CheckRunId,

    #[getset(get = "pub")]
    name: CheckRunName,

    #[getset(get = "pub")]
    check_suite: CheckSuite,

    #[getset(get_copy = "pub")]
    status: CheckRunStatus,

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
