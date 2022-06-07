/// GitHub host
///
/// GitHub can be hosted on-prem using the GitHub Enterprise server. Clients can interact with these
/// servers by setting a custom base URL for GitHub.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GitHubHost(String);

impl GitHubHost {
    /// Initializes a new GitHub host.
    pub fn new(github_host: String) -> Self {
        Self(github_host)
    }

    /// Returns the URL of the GitHub host.
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl From<&str> for GitHubHost {
    fn from(github_host: &str) -> Self {
        GitHubHost::new(github_host.into())
    }
}

#[cfg(test)]
mod tests {
    use super::GitHubHost;

    #[test]
    fn github_host() {
        let github_host = GitHubHost::new("github_host".into());
        assert_eq!("github_host", github_host.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubHost>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubHost>();
    }
}
