use secrecy::{ExposeSecret, SecretString};

/// Webhook secret
///
/// GitHub adds a cryptographic signature based on a shared secret to its webhooks. The signature
/// can be used to verify that the webhook was sent by GitHub and not a malicious party.
#[derive(Clone, Debug)]
pub struct WebhookSecret(SecretString);

impl WebhookSecret {
    /// Initializes a new webhook secret.
    pub fn new(webhook_secret: String) -> Self {
        Self(SecretString::new(webhook_secret))
    }

    /// Returns the webhook secret.
    pub fn get(&self) -> &str {
        self.0.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use super::WebhookSecret;

    #[test]
    fn webhook_secret() {
        let webhook_secret = WebhookSecret::new("webhook_secret".into());
        assert_eq!("webhook_secret", webhook_secret.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<WebhookSecret>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<WebhookSecret>();
    }
}
