/// GitHub App id
///
/// GitHub Apps have a unique `id` that is combined with the app's private key to authenticate the
/// app against GitHub's API.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AppId(u64);

impl AppId {
    /// Initializes a new app id.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the app id.
    pub fn get(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::AppId;

    #[test]
    fn app_id() {
        let app_id = AppId::new(1);
        assert_eq!(1, app_id.get());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AppId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AppId>();
    }
}
