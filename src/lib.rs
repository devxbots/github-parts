//! Types and actions for GitHub
//!
//! [`github-parts`] is a tailor-made integration with GitHub. It is designed to make it easier to
//! build GitHub Apps, and thus focuses mostly on [webhook events and payloads]. The crate contains
//! _types_ that represent resources on GitHub, and _actions_ that can create and modify them.
//!
//! [webhook events and payloads]: https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads

#![warn(missing_docs)]

pub mod event;
