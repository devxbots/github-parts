//! Internal errors

use thiserror::Error as ThisError;

/// Errors that can occur inside github-parts
///
/// github-parts interacts with external resources, for example GitHub's API, which always has a
/// chance of failing due to a variety of reasons. The different errors that can occur are
/// represented by this struct. Consumers of the crate can decide if they want to rethrow an error
/// or retry an operation.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Authenticating with GitHub failed
    #[error("authentication failed with the following error: {0}")]
    Authentication(#[from] jsonwebtoken::errors::Error),

    /// An operation inside the crate failed
    #[error("{0}")]
    Internal(String),

    /// Accessing external resources failed
    #[error("an outgoing request failed with the following error: {0}")]
    Request(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
