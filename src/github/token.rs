use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::github::{AppId, GitHubHost, PrivateKey};
use crate::installation::InstallationId;

#[derive(Clone, Debug)]
pub struct AppToken(SecretString);

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

    pub fn get(&self) -> &str {
        self.0.expose_secret()
    }
}

impl InstallationToken {
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
