use crate::github::app::AppId;
use crate::github::client::GitHubClient;
use crate::github::{GitHubHost, PrivateKey};
use crate::installation::InstallationId;

pub fn github_client() -> GitHubClient {
    GitHubClient::new(
        GitHubHost::new(mockito::server_url()),
        AppId::new(1),
        PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
        InstallationId::new(1),
    )
}
