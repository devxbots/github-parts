use std::fmt::{Display, Formatter};

use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::name;

name!(CheckRunOutputTitle);
name!(CheckRunOutputSummary);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize, Getters)]
pub struct CheckRunOutput {
    /// The title of the check run.
    #[getset(get = "pub")]
    title: CheckRunOutputTitle,

    /// The summary of the check run. This parameter supports Markdown.
    #[getset(get = "pub")]
    summary: CheckRunOutputSummary,

    /// The details of the check run. This parameter supports Markdown.
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::CheckRunOutput;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunOutput>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunOutput>();
    }
}
