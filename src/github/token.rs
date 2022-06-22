use std::marker::PhantomData;
use std::ops::Sub;
use std::sync::Arc;

use anyhow::Context;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use parking_lot::Mutex;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::github::app::AppId;
use crate::github::{GitHubHost, PrivateKey};
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
    app_token: Arc<Mutex<Token<App>>>,
    installation_token: Arc<Mutex<Token<Installation>>>,
}

impl TokenFactory {
    #[tracing::instrument]
    pub fn new(github_host: GitHubHost, app_id: AppId, private_key: PrivateKey) -> Self {
        let expiration = Utc::now().sub(Duration::days(1));

        let expired_app_token = Token {
            scope: PhantomData,
            token: SecretString::new("app_token".into()),
            expires_at: expiration,
        };
        let expired_installation_token = Token {
            scope: PhantomData,
            token: SecretString::new("installation_token".into()),
            expires_at: expiration,
        };

        Self {
            github_host,
            app_id,
            private_key,
            app_token: Arc::new(Mutex::new(expired_app_token)),
            installation_token: Arc::new(Mutex::new(expired_installation_token)),
        }
    }

    #[tracing::instrument]
    pub fn app(&self) -> Result<Token<App>, Error> {
        let now = Utc::now();

        {
            let app_token = self.app_token.lock();
            if app_token.expires_at > now {
                return Ok(app_token.clone());
            }
        }

        let jwt = self.generate_jwt()?;
        let token = Token {
            scope: PhantomData,
            token: SecretString::new(jwt),
            expires_at: now,
        };

        {
            let mut app_token = self.app_token.lock();
            *app_token = token.clone();
        }

        Ok(token)
    }

    #[tracing::instrument]
    pub async fn installation(
        &self,
        installation_id: InstallationId,
    ) -> Result<Token<Installation>, Error> {
        let now = Utc::now();

        {
            let installation_token = self.installation_token.lock();
            if installation_token.expires_at > now {
                return Ok(installation_token.clone());
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

        {
            let mut installation_token = self.installation_token.lock();
            *installation_token = token.clone();
        }

        Ok(token)
    }

    #[tracing::instrument]
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
    use std::sync::Arc;

    use chrono::{Duration, Utc};
    use mockito::mock;
    use parking_lot::Mutex;
    use secrecy::SecretString;

    use crate::github::app::AppId;
    use crate::github::{GitHubHost, PrivateKey};
    use crate::installation::InstallationId;

    use super::{App, Installation, Token, TokenFactory};

    fn factory(
        app_token: Option<Token<App>>,
        installation_token: Option<Token<Installation>>,
    ) -> TokenFactory {
        let expiration = Utc::now().sub(Duration::days(1));

        let app_token = match app_token {
            Some(token) => token,
            None => Token {
                scope: PhantomData,
                token: SecretString::new("app_token".into()),
                expires_at: expiration,
            },
        };
        let installation_token = match installation_token {
            Some(token) => token,
            None => Token {
                scope: PhantomData,
                token: SecretString::new("installation_token".into()),
                expires_at: expiration,
            },
        };

        TokenFactory {
            github_host: GitHubHost::new(mockito::server_url()),
            app_id: AppId::new(1),
            private_key: PrivateKey::new(
                include_str!("../../tests/fixtures/private-key.pem").into(),
            ),
            app_token: Arc::new(Mutex::new(app_token)),
            installation_token: Arc::new(Mutex::new(installation_token)),
        }
    }

    #[test]
    fn app_caches_token_while_it_is_not_expired() {
        let token = Token {
            scope: PhantomData,
            token: SecretString::new("app".into()),
            expires_at: Utc::now().add(Duration::minutes(10)),
        };
        let factory = factory(Some(token.clone()), None);

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
        let factory = factory(Some(token.clone()), None);

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
        let factory = factory(None, Some(token.clone()));

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
        let factory = factory(Some(app_token.clone()), Some(installation_token));

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
