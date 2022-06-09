use secrecy::{ExposeSecret, SecretString};

#[derive(Clone, Debug)]
pub struct PrivateKey(SecretString);

impl PrivateKey {
    pub fn new(private_key: String) -> Self {
        Self(SecretString::new(private_key))
    }

    pub fn get(&self) -> &str {
        self.0.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use super::PrivateKey;

    #[test]
    fn private_key() {
        let private_key = PrivateKey::new("private_key".into());
        assert_eq!("private_key", private_key.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<PrivateKey>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<PrivateKey>();
    }
}
