use std::marker::PhantomData;

use anyhow::Context;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct App;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Installation;

#[derive(Clone, Debug)]
pub struct Token<Scope> {
    scope: PhantomData<Scope>,
    token: SecretString,
    expires_at: DateTime<Utc>,
}

impl<Scope> Token<Scope> {
    pub fn get(&self) -> &str {
        self.token.expose_secret()
    }
}

#[derive(Clone, Debug)]
pub struct TokenFactory {
    github_host: GitHubHost,
    app_id: AppId,
    private_key: PrivateKey,
    app_token: Option<Token<App>>,
    installation_token: Option<Token<Installation>>,
}

impl TokenFactory {
    pub fn new(github_host: GitHubHost, app_id: AppId, private_key: PrivateKey) -> Self {
        Self {
            github_host,
            app_id,
            private_key,
            app_token: None,
            installation_token: None,
        }
    }

    pub fn app(&mut self) -> Result<Token<App>, Error> {
        let now = Utc::now();

        if let Some(token) = &self.app_token {
            if token.expires_at > now {
                return Ok(token.clone());
            }
        }

        let jwt = self.generate_jwt()?;
        let token = Token {
            scope: PhantomData,
            token: SecretString::new(jwt),
            expires_at: now,
        };

        self.app_token = Some(token.clone());

        Ok(token)
    }

    pub async fn installation(
        &mut self,
        installation_id: InstallationId,
    ) -> Result<Token<Installation>, Error> {
        let now = Utc::now();

        if let Some(token) = &self.installation_token {
            if token.expires_at > now {
                return Ok(token.clone());
            }
        }

        let url = format!(
            "{}/app/installations/{}/access_tokens",
            self.github_host.get(),
            installation_id
        );

        let app_token = self.app()?;

        let response = Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", app_token.get()))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "devxbots/github-parts")
            .send()
            .await?;

        let access_token_response = response.json::<AccessTokensResponse>().await?;
        let token = Token {
            scope: PhantomData,
            token: SecretString::new(access_token_response.token),
            expires_at: now,
        };

        self.installation_token = Some(token.clone());

        Ok(token)
    }

    fn generate_jwt(&self) -> Result<String, Error> {
        let now = Utc::now();

        let issued_at = now
            .checked_sub_signed(Duration::seconds(60))
            .context("failed to create timestamp for iat claimn in GitHub App token")?;

        let expires_at = now
            .checked_add_signed(Duration::minutes(10))
            .context("failed to create timestamp for exp claim in GitHub App token")?;

        let claims = Claims {
            iat: issued_at.timestamp(),
            iss: self.app_id.get().to_string(),
            exp: expires_at.timestamp(),
        };

        let header = Header::new(Algorithm::RS256);
        let key =
            EncodingKey::from_rsa_pem(self.private_key.get().as_bytes()).map_err(|error| {
                Error::Configuration(
                    Box::new(error),
                    "failed to create encoding key for GitHub App token".into(),
                )
            })?;

        Ok(encode(&header, &claims, &key).context("failed to encode JWT for GitHub App token")?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: i64,
    iss: String,
    exp: i64,
}

#[derive(Deserialize, Serialize)]
struct AccessTokensResponse {
    token: String,
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use std::ops::{Add, Sub};

    use chrono::{Duration, Utc};
    use mockito::mock;
    use secrecy::SecretString;

    use crate::github::{AppId, GitHubHost, PrivateKey};
    use crate::installation::InstallationId;

    use super::{App, Installation, Token, TokenFactory};

    fn factory(
        app_token: Option<Token<App>>,
        installation_token: Option<Token<Installation>>,
    ) -> TokenFactory {
        TokenFactory {
            github_host: GitHubHost::new(mockito::server_url()),
            app_id: AppId::new(1),
            private_key: PrivateKey::new(
                include_str!("../../tests/fixtures/private-key.pem").into(),
            ),
            app_token,
            installation_token,
        }
    }

    #[test]
    fn app_caches_token_while_it_is_not_expired() {
        let token = Token {
            scope: PhantomData,
            token: SecretString::new("app".into()),
            expires_at: Utc::now().add(Duration::minutes(10)),
        };
        let mut factory = factory(Some(token.clone()), None);

        let new_token = factory.app().unwrap();

        assert_eq!(new_token.get(), token.get());
    }

    #[test]
    fn app_generates_new_when_token_expired() {
        let token = Token {
            scope: PhantomData,
            token: SecretString::new("app".into()),
            expires_at: Utc::now().sub(Duration::minutes(10)),
        };
        let mut factory = factory(Some(token.clone()), None);

        let new_token = factory.app().unwrap();

        assert_ne!(new_token.get(), token.get());
    }

    #[tokio::test]
    async fn installation_caches_token_while_it_is_not_expired() {
        let token = Token {
            scope: PhantomData,
            token: SecretString::new("installation".into()),
            expires_at: Utc::now().add(Duration::minutes(10)),
        };
        let mut factory = factory(None, Some(token.clone()));

        let new_token = factory.installation(InstallationId::new(1)).await.unwrap();

        assert_eq!(new_token.get(), token.get());
    }

    #[tokio::test]
    async fn installation_requests_new_when_token_expired() {
        let _mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();

        let app_token = Token {
            scope: PhantomData,
            token: SecretString::new("app".into()),
            expires_at: Utc::now().sub(Duration::minutes(10)),
        };
        let installation_token = Token {
            scope: PhantomData,
            token: SecretString::new("installation".into()),
            expires_at: Utc::now().add(Duration::minutes(10)),
        };
        let mut factory = factory(Some(app_token.clone()), Some(installation_token));

        let new_token = factory.installation(InstallationId::new(1)).await.unwrap();

        assert_ne!(new_token.get(), app_token.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Token<App>>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Token<App>>();
    }
}
