use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub use self::check_run::CheckRunEvent;

mod check_run;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum Event {
    CheckRun(Box<CheckRunEvent>),
    Unsupported(serde_json::Value),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Event::CheckRun(_) => "check run",
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
