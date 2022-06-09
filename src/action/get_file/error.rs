use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetFileError {
    #[error("file not found")]
    NotFound,

    #[error("path was a directory, but must be a file")]
    Directory,

    #[error("path was a submodule, but must be a file")]
    Submodule,

    // TODO: Follow symlinks and return the file
    #[error("path was a symlink, but must be a file")]
    Symlink,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::GetFileError;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GetFileError>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GetFileError>();
    }
}
