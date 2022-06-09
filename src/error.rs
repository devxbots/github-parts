#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{1}")]
    Configuration(#[source] Box<dyn std::error::Error + Send + Sync>, String),

    #[error(transparent)]
    ExternalResource(#[from] reqwest::Error),

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
