use std::fmt::Debug;

use anyhow::{anyhow, Context};
use reqwest::header::HeaderValue;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::github::app::AppId;
use crate::github::token::TokenFactory;
use crate::github::{GitHubHost, PrivateKey};
use crate::installation::InstallationId;

#[derive(Clone, Debug)]
pub struct GitHubClient {
    github_host: GitHubHost,
    token_factory: TokenFactory,
    installation_id: InstallationId,
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubClientError {
    #[error("failed to find the request resource")]
    NotFound,

    #[error("{0}")]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl GitHubClient {
    #[tracing::instrument]
    pub fn new(
        github_host: GitHubHost,
        app_id: AppId,
        private_key: PrivateKey,
        installation_id: InstallationId,
    ) -> Self {
        let token_factory = TokenFactory::new(github_host.clone(), app_id, private_key);

        Self {
            github_host,
            token_factory,
            installation_id,
        }
    }

    #[tracing::instrument]
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, GitHubClientError>
    where
        T: DeserializeOwned,
    {
        // We need to explicitly declare the type of the body somewhere to silence a compiler error.
        let body: Option<Value> = None;

        self.send_request(Method::GET, endpoint, body).await
    }

    #[tracing::instrument(skip(body))]
    pub async fn post<T>(
        &self,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, GitHubClientError>
    where
        T: DeserializeOwned,
    {
        self.send_request(Method::POST, endpoint, body).await
    }

    #[tracing::instrument(skip(body))]
    pub async fn patch<T>(
        &self,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, GitHubClientError>
    where
        T: DeserializeOwned,
    {
        self.send_request(Method::PATCH, endpoint, body).await
    }

    #[tracing::instrument(skip(body))]
    async fn send_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, GitHubClientError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.github_host.get(), endpoint);

        let mut client = self.client(method.clone(), &url).await?;

        if let Some(body) = body {
            client = client.json(&body);
        }

        let response = client.send().await?;
        let status = &response.status();

        if !status.is_success() {
            tracing::error!(
                "failed to {} to GitHub: {:?}",
                &method,
                response.text().await?
            );

            return if status == &404 {
                Err(GitHubClientError::NotFound)
            } else {
                Err(GitHubClientError::UnexpectedError(anyhow!(
                    "failed to {} to GitHub",
                    &method
                )))
            };
        }

        let data = response.json::<T>().await?;

        Ok(data)
    }

    #[tracing::instrument]
    pub async fn paginate<T>(
        &self,
        method: Method,
        endpoint: &str,
        key: &str,
    ) -> Result<Vec<T>, GitHubClientError>
    where
        T: DeserializeOwned,
    {
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
        let token = self
            .token_factory
            .installation(self.installation_id)
            .await
            .context("failed to get authentication token from factory")?;

        let client = Client::new()
            .request(method, url)
            .header("Authorization", format!("Bearer {}", token.get()))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "devxbots/github-parts");

        Ok(client)
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
}

#[cfg(test)]
mod tests {
    use mockito::mock;
    use reqwest::header::HeaderValue;
    use reqwest::Method;

    use crate::github::app::AppId;
    use crate::github::{GitHubHost, PrivateKey};
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

        let client = GitHubClient::new(
            GitHubHost::new(mockito::server_url()),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
            InstallationId::new(1),
        );

        let repository: Repository = client.get("/repos/octocat/Hello-World").await.unwrap();

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

        let client = GitHubClient::new(
            GitHubHost::new(mockito::server_url()),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
            InstallationId::new(1),
        );

        let repository: Vec<Repository> = client
            .paginate(Method::GET, "/installation/repositories", "repositories")
            .await
            .unwrap();

        assert_eq!(2, repository.len());
    }

    #[test]
    fn get_next_url_returns_url() {
        let client = GitHubClient::new(
            GitHubHost::new(mockito::server_url()),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
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
        let client = GitHubClient::new(
            GitHubHost::new(mockito::server_url()),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
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
        assert_send::<GitHubClient>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubClient>();
    }
}
