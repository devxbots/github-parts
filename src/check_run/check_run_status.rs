use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Check run status
///
/// Check runs have a status, which indicates whether the check run has already started, is
/// currently running, or has finished.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum CheckRunStatus {
    /// The check run has been queued, but not started yet.
    Queued,

    /// The check run is currently running.
    InProgress,

    /// The check run has finished.
    Completed,
}

impl Display for CheckRunStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            CheckRunStatus::Queued => "queued",
            CheckRunStatus::InProgress => "in progress",
            CheckRunStatus::Completed => "completed",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckRunStatus;

    #[test]
    fn trait_display() {
        assert_eq!("queued", CheckRunStatus::Queued.to_string());
        assert_eq!("in progress", CheckRunStatus::InProgress.to_string());
        assert_eq!("completed", CheckRunStatus::Completed.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunStatus>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunStatus>();
    }
}
