//! Internal errors

/// Errors that can occur inside github-parts
///
/// github-parts interacts with external resources, for example GitHub's API, which always has a
/// chance of failing due to a variety of reasons. The different errors that can occur are
/// represented by this struct. Consumers of the crate can decide if they want to rethrow an error
/// or retry an operation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The configuration of the crate is invalid or caused an error
    #[error("{1}")]
    Configuration(#[source] Box<dyn std::error::Error + Send + Sync>, String),

    /// Accessing external resources failed
    #[error(transparent)]
    ExternalResource(#[from] reqwest::Error),

    /// An operation inside the crate failed
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
