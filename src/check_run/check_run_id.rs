use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Check run id
///
/// Check runs are uniquely identified by their `id`, which can be used to interact with a check run
/// in GitHub's REST API.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct CheckRunId(u64);

impl CheckRunId {
    /// Initializes a new check run id.
    pub fn new(check_run_id: u64) -> Self {
        Self(check_run_id)
    }

    /// Returns the check run id.
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Display for CheckRunId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckRunId;

    #[test]
    fn trait_display() {
        let id = CheckRunId::new(1);

        assert_eq!("1", id.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunId>();
    }
}
