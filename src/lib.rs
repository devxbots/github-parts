//! Types and actions for GitHub
//!
//! [`github-parts`] is a tailor-made integration with GitHub. It is designed to make it easier to
//! build GitHub Apps, and thus focuses mostly on [webhook events and payloads]. The crate contains
//! _types_ that represent resources on GitHub, and _actions_ that can create and modify them.
//!
//! [webhook events and payloads]: https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads

#![warn(missing_docs)]

macro_rules! id {
    ($name:ident) => {
        /// Id
        ///
        /// Resources on GitHub have a unique `id` that is used to interact with them through
        /// GitHub's REST API.
        #[derive(
            Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize,
        )]
        pub struct $name(u64);

        impl $name {
            /// Initializes a new id.
            pub fn new(id: u64) -> Self {
                Self(id)
            }

            /// Returns the id.
            pub fn get(&self) -> u64 {
                self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

pub mod account;
pub mod check_run;
pub mod event;
pub mod github;
pub mod installation;
pub mod repository;
pub mod visibility;
