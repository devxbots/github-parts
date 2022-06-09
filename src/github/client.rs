use std::marker::PhantomData;

use anyhow::Context;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

use crate::github::token::{AppToken, InstallationToken};
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;

#[derive(Copy, Clone, Debug)]
pub struct GitHubClient<'a, T> {
    return_type: PhantomData<T>,
    github_host: &'a GitHubHost,
    app_id: AppId,
    private_key: &'a PrivateKey,
    installation_id: InstallationId,
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubClientError {
    #[error("{0}")]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl<'a, T> GitHubClient<'a, T>
where
    T: DeserializeOwned,
{
    pub fn new(
        github_host: &'a GitHubHost,
        app_id: AppId,
        private_key: &'a PrivateKey,
        installation_id: InstallationId,
    ) -> Self {
        Self {
            return_type: PhantomData::default(),
            github_host,
            app_id,
            private_key,
            installation_id,
        }
    }

    pub async fn request(&self, method: Method, url: &str) -> Result<T, GitHubClientError> {
        let token = self.token().await?;

        let data = Client::new()
            .request(method, url)
            .header("Authorization", format!("Bearer {}", token.get()))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "devxbots/github-parts")
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(data)
    }

    async fn token(&self) -> Result<InstallationToken, GitHubClientError> {
        let app_token = AppToken::new(&self.app_id, self.private_key)
            .context("failed to create GitHub App token")?;

        let installation_token =
            InstallationToken::new(self.github_host, &app_token, &self.installation_id)
                .await
                .context("failed to create GitHub installation token")?;

        Ok(installation_token)
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use reqwest::Method;

    use crate::github::{AppId, GitHubHost, PrivateKey};
    use crate::installation::InstallationId;
    use crate::repository::Repository;

    use super::GitHubClient;

    #[tokio::test]
    async fn get_installation_repositories() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/octocat/Hello-World")
            .with_status(200)
            .with_body(
                r#"
                {
                    "id": 1296269,
                    "name": "Hello-World",
                    "description": "This your first repo!",
                    "owner": {
                        "login": "octocat",
                        "id": 1,
                        "type": "User"
                    },
                    "visibility": "public"
                }
            "#,
            )
            .create();

        let github_host = GitHubHost::new(mockito::server_url());
        let private_key =
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into());
        let client: GitHubClient<Repository> = GitHubClient::new(
            &github_host,
            AppId::new(1),
            &private_key,
            InstallationId::new(1),
        );

        let url = format!("{}/repos/octocat/Hello-World", mockito::server_url());
        let repository = client.request(Method::GET, &url).await.unwrap();

        assert_eq!(1296269, repository.id().get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubClient<usize>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubClient<usize>>();
    }
}
