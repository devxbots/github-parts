use secrecy::{ExposeSecret, SecretString};

#[derive(Clone, Debug)]
pub struct WebhookSecret(SecretString);

impl WebhookSecret {
    pub fn new(webhook_secret: String) -> Self {
        Self(SecretString::new(webhook_secret))
    }

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
