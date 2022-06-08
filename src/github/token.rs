//! Authentication tokens for GitHub
//!
//! github-parts interacts with GitHub through its REST API. It authenticates as a GitHub App by
//! default, but can also authenticate as an installation. Both scopes have their own `Token`, and
//! actions can declare which one they need through Rust's type system.

use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;

/// Authentication token for GitHub App
///
/// github-parts interacts with GitHub through its REST API and uses authentication tokens to
/// identify itself. The `AppToken` represents the GitHub App itself, and can be used to interact
/// with GitHub's API on behalf of the app.
///
/// https://docs.github.com/en/developers/apps/authenticating-with-github-apps
#[derive(Clone, Debug)]
pub struct AppToken(SecretString);

/// Authentication token for installations
///
/// github-parts interacts with GitHub through its REST API and uses authentication tokens to
/// identify itself. The `InstallationToken` represents the installation of the GitHub App in a
/// specific account, and can be used to interact with the resources of this particular
/// installation.
///
/// https://docs.github.com/en/developers/apps/authenticating-with-github-apps
#[derive(Clone, Debug)]
pub struct InstallationToken(SecretString);

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

impl AppToken {
    /// Create a new app token
    ///
    /// The app token can be used to request resources on behalf of the GitHub App. It is created by
    /// initializing a JSON Web Token and signing it with the app's private key.
    ///
    /// https://jwt.io/
    pub fn new(app_id: &AppId, private_key: &PrivateKey) -> Result<Self, Error> {
        let now = Utc::now();

        let issued_at = now
            .checked_sub_signed(Duration::seconds(60))
            .context("failed to create timestamp for iat claimn in GitHub App token")?;

        let expires_at = now
            .checked_add_signed(Duration::minutes(10))
            .context("failed to create timestamp for exp claim in GitHub App token")?;

        let claims = Claims {
            iat: issued_at.timestamp(),
            iss: app_id.get().to_string(),
            exp: expires_at.timestamp(),
        };

        let header = Header::new(Algorithm::RS256);
        let key = EncodingKey::from_rsa_pem(private_key.get().as_bytes()).map_err(|error| {
            Error::Configuration(
                Box::new(error),
                "failed to create encoding key for GitHub App token".into(),
            )
        })?;

        let jwt =
            encode(&header, &claims, &key).context("failed to encode JWT for GitHub App token")?;

        Ok(Self(SecretString::new(jwt)))
    }

    /// Get the token
    pub fn get(&self) -> &str {
        self.0.expose_secret()
    }
}

impl InstallationToken {
    /// Request a new installation token
    ///
    /// The installation token can be used to request resources on behalf of the installation. The
    /// token is created by requesting it from GitHub using the app token.
    ///
    /// https://docs.github.com/en/developers/apps/authenticating-with-github-apps
    pub async fn new(
        endpoint: &GitHubHost,
        app_token: &AppToken,
        installation: &InstallationId,
    ) -> Result<InstallationToken, Error> {
        let url = format!(
            "{}/app/installations/{}/access_tokens",
            endpoint.get(),
            installation
        );

        let response = Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", app_token.get()))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "devxbots/github-parts")
            .send()
            .await?;

        let access_token_response = response.json::<AccessTokensResponse>().await?;

        Ok(Self(SecretString::new(access_token_response.token)))
    }

    /// Get the token
    pub fn get(&self) -> &str {
        self.0.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use mockito::mock;

    use crate::error::Error;
    use crate::github::{AppId, GitHubHost, PrivateKey};
    use crate::installation::InstallationId;

    use super::{AppToken, Claims, InstallationToken};

    #[test]
    fn app() -> Result<(), Error> {
        let app_token = AppToken::new(
            &AppId::new(1),
            &PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
        )?;

        let token = decode::<Claims>(
            app_token.get(),
            &DecodingKey::from_rsa_pem(
                include_str!("../../tests/fixtures/public-key.pem").as_bytes(),
            )
            .context("failed to create decoding key from public key")?,
            &Validation::new(Algorithm::RS256),
        );

        assert!(token.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn installation_token() -> Result<(), Error> {
        let _mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();

        let github_host = GitHubHost::new(mockito::server_url());
        let app_token = AppToken::new(
            &AppId::new(1),
            &PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem").into()),
        )?;
        let installation_id = InstallationId::new(1);

        let installation_token =
            InstallationToken::new(&github_host, &app_token, &installation_id).await;

        assert!(installation_token.is_ok());
        Ok(())
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AppToken>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AppToken>();
    }
}
