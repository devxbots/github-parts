//! Webhook events
//!
//! GitHub enables integrations to subscribe to events. Whenever such an event happens on GitHub, a
//! webhook is sent to the integration. Events correspond to certain actions, for example the
//! `issue` event is fired everytime an issue is opened, closed, labeled, etc.
//!
//! Read more: <https://docs.github.com/en/developers/webhooks-and-events/webhooks>

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Webhook event
///
/// GitHub provides a wide variety of events, which enable integrations to react to almost any
/// action that is taken on the platform. Webhooks represent a single event, and their payload is
/// deserialized to the [`Event`] enum.
///
/// Events that are not yet supported by [`github-parts`] are captured in the `Event::Unsupported`
/// variant. It contains [`serde_json::Value`] payload, which makes it possible to still work with
/// the event.
///
/// Read more: <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads>
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum Event {
    /// Unsupported event
    ///
    /// This event is not yet supported by [`github-parts`], but the webhook payload is passed
    /// through as a [`serde_json::Value`] so that consumers can still work with it.
    Unsupported(serde_json::Value),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Event::Unsupported(_) => "unsupported event",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Event;

    #[test]
    fn trait_display() {
        let event = Event::Unsupported(json!({}));

        assert_eq!("unsupported event", event.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Event>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Event>();
    }
}
