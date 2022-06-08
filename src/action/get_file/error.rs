use thiserror::Error;

/// Errors that can occur when getting a file from GitHub
#[derive(Debug, Error)]
pub enum GetFileError {
    /// The file was not found
    #[error("file not found")]
    NotFound,

    /// The user tried to get a directory
    #[error("path was a directory, but must be a file")]
    Directory,

    /// The user tried to get a git submodule
    #[error("path was a submodule, but must be a file")]
    Submodule,

    /// The user tried to get a symlink
    // TODO: Follow symlinks and return the file
    #[error("path was a symlink, but must be a file")]
    Symlink,

    /// An unexpected error occurred while running the action
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
