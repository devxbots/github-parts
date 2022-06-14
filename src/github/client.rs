use std::fmt::Debug;
use std::marker::PhantomData;

use anyhow::{anyhow, Context};
use reqwest::header::HeaderValue;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::github::token::{AppToken, InstallationToken};
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;

#[derive(Clone, Debug)]
pub struct GitHubClient<'a, T>
where
    T: Debug,
{
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
    T: Debug + DeserializeOwned,
{
    #[tracing::instrument]
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

    #[tracing::instrument]
    pub async fn get(&self, endpoint: &str) -> Result<T, GitHubClientError> {
        let url = format!("{}{}", self.github_host.get(), endpoint);

        let data = self
            .client(Method::GET, &url)
            .await?
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(data)
    }

    #[tracing::instrument(skip(body))]
    pub async fn post(
        &self,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, GitHubClientError> {
        self.request_with_body(Method::POST, endpoint, body).await
    }

    #[tracing::instrument(skip(body))]
    pub async fn patch(
        &self,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, GitHubClientError> {
        self.request_with_body(Method::PATCH, endpoint, body).await
    }

    #[tracing::instrument]
    pub async fn paginate(
        &self,
        method: Method,
        endpoint: &str,
        key: &str,
    ) -> Result<Vec<T>, GitHubClientError> {
        let url = format!("{}{}", self.github_host.get(), endpoint);

        let mut collection = Vec::new();
        let mut next_url = Some(url);

        while next_url.is_some() {
            let response = self
                .client(method.clone(), &next_url.unwrap())
                .await?
                .send()
                .await?;

            next_url = self.get_next_url(response.headers().get("link"))?;
            let body = &response.json::<Value>().await?;

            let payload = body
                .get(key)
                .context("failed to find pagination key in HTTP response")?;

            // TODO: Avoid cloning the payload
            let mut entities: Vec<T> = serde_json::from_value(payload.clone())
                .context("failed to deserialize paginated entities")?;

            collection.append(&mut entities);
        }

        Ok(collection)
    }

    #[tracing::instrument]
    async fn client(&self, method: Method, url: &str) -> Result<RequestBuilder, GitHubClientError> {
        let token = self.token().await?;

        let client = Client::new()
            .request(method, url)
            .header("Authorization", format!("Bearer {}", token.get()))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "devxbots/github-parts");

        Ok(client)
    }

    #[tracing::instrument]
    async fn token(&self) -> Result<InstallationToken, GitHubClientError> {
        let app_token = AppToken::new(&self.app_id, self.private_key)
            .context("failed to create GitHub App token")?;

        let installation_token =
            InstallationToken::new(self.github_host, &app_token, &self.installation_id)
                .await
                .context("failed to create GitHub installation token")?;

        Ok(installation_token)
    }

    #[tracing::instrument]
    fn get_next_url(
        &self,
        header: Option<&HeaderValue>,
    ) -> Result<Option<String>, GitHubClientError> {
        let header = match header {
            Some(header) => header,
            None => return Ok(None),
        };

        let relations: Vec<&str> = header
            .to_str()
            .context("failed to parse HTTP request header")?
            .split(',')
            .collect();

        let next_rel = match relations.iter().find(|link| link.contains(r#"rel="next"#)) {
            Some(link) => link,
            None => return Ok(None),
        };

        let link_start_position = 1 + next_rel
            .find('<')
            .context("failed to extract next url from link header")?;
        let link_end_position = next_rel
            .find('>')
            .context("failed to extract next url from link header")?;

        let link = String::from(&next_rel[link_start_position..link_end_position]);

        Ok(Some(link))
    }

    #[tracing::instrument(skip(body))]
    async fn request_with_body(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, GitHubClientError> {
        let url = format!("{}{}", self.github_host.get(), endpoint);

        let mut client = self.client(method.clone(), &url).await?;

        if let Some(body) = body {
            client = client.json(&body);
        }

        let response = client.send().await?;

        if !response.status().is_success() {
            tracing::error!(
                "failed to {} to GitHub: {:?}",
                &method,
                response.text().await?
            );
            return Err(GitHubClientError::UnexpectedError(anyhow!(
                "failed to {} to GitHub",
                &method
            )));
        }

        let data = response.json::<T>().await?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use reqwest::header::HeaderValue;
    use reqwest::Method;

    use crate::github::{AppId, GitHubHost, PrivateKey};
    use crate::installation::InstallationId;
    use crate::repository::Repository;

    use super::GitHubClient;

    #[tokio::test]
    async fn get_entity() {
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

        let repository = client.get("/repos/octocat/Hello-World").await.unwrap();

        assert_eq!(1296269, repository.id().get());
    }

    #[tokio::test]
    async fn paginate_returns_all_entities() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _first_page_mock = mock("GET", "/installation/repositories")
            .with_status(200)
            .with_header(
                "link",
                &format!(
                    "<{}/installation/repositories?page=2>; rel=\"next\"",
                    mockito::server_url()
                ),
            )
            .with_body(
                r#"
                {
                    "total_count": 2,
                    "repositories": [
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
                    ]
                }
            "#,
            )
            .create();
        let _second_page_mock = mock("GET", "/installation/repositories?page=2")
            .with_status(200)
            .with_body(
                r#"
                {
                    "total_count": 2,
                    "repositories": [
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
                    ]
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

        let repository = client
            .paginate(Method::GET, "/installation/repositories", "repositories")
            .await
            .unwrap();

        assert_eq!(2, repository.len());
    }

    #[test]
    fn get_next_url_returns_url() {
        let github_host = GitHubHost::new(mockito::server_url());
        let private_key =
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into());
        let client: GitHubClient<Repository> = GitHubClient::new(
            &github_host,
            AppId::new(1),
            &private_key,
            InstallationId::new(1),
        );

        let header = HeaderValue::from_str(r#"<https://api.github.com/search/code?q=addClass+user%3Amozilla&page=13>; rel="prev", <https://api.github.com/search/code?q=addClass+user%3Amozilla&page=15>; rel="next", <https://api.github.com/search/code?q=addClass+user%3Amozilla&page=34>; rel="last", <https://api.github.com/search/code?q=addClass+user%3Amozilla&page=1>; rel="first""#).unwrap();

        let next_url = client.get_next_url(Some(&header)).unwrap().unwrap();

        assert_eq!(
            "https://api.github.com/search/code?q=addClass+user%3Amozilla&page=15",
            next_url
        );
    }

    #[test]
    fn get_next_url_returns_none() {
        let github_host = GitHubHost::new(mockito::server_url());
        let private_key =
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into());
        let client: GitHubClient<Repository> = GitHubClient::new(
            &github_host,
            AppId::new(1),
            &private_key,
            InstallationId::new(1),
        );

        let header = HeaderValue::from_str(
            r#"<https://api.github.com/search/code?q=addClass+user%3Amozilla&page=13>; rel="prev""#,
        )
        .unwrap();

        let next_url = client.get_next_url(Some(&header)).unwrap();

        assert!(next_url.is_none());
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
