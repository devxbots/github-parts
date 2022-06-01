use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Check run conclusion
///
/// When a check run finishes, its result is indicated by the `conclusion`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunConclusion {
    /// The check run finished successfully.
    Success,

    /// The check run failed.
    Failure,

    /// The check run finished without a conclusion.
    Neutral,

    /// The check run was cancelled.
    Cancelled,

    /// The check run timed out.
    TimedOut,

    /// The check run requires an action by the user.
    ActionRequired,

    /// The check run is stale.
    Stale,
}

impl Display for CheckRunConclusion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            CheckRunConclusion::Success => "success",
            CheckRunConclusion::Failure => "failure",
            CheckRunConclusion::Neutral => "neutral",
            CheckRunConclusion::Cancelled => "cancelled",
            CheckRunConclusion::TimedOut => "timed out",
            CheckRunConclusion::ActionRequired => "action required",
            CheckRunConclusion::Stale => "stale",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckRunConclusion;

    #[test]
    fn trait_display() {
        assert_eq!("success", CheckRunConclusion::Success.to_string());
        assert_eq!("failure", CheckRunConclusion::Failure.to_string());
        assert_eq!("neutral", CheckRunConclusion::Neutral.to_string());
        assert_eq!("cancelled", CheckRunConclusion::Cancelled.to_string());
        assert_eq!("timed out", CheckRunConclusion::TimedOut.to_string());
        assert_eq!(
            "action required",
            CheckRunConclusion::ActionRequired.to_string()
        );
        assert_eq!("stale", CheckRunConclusion::Stale.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunConclusion>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunConclusion>();
    }
}
