use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Created,
    Completed,
    Rerequested,
    RequestedAction,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Action::Created => "created",
            Action::Completed => "completed",
            Action::Rerequested => "re-requested",
            Action::RequestedAction => "requested action",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::Action;

    #[test]
    fn trait_display() {
        assert_eq!("created", Action::Created.to_string());
        assert_eq!("completed", Action::Completed.to_string());
        assert_eq!("re-requested", Action::Rerequested.to_string());
        assert_eq!("requested action", Action::RequestedAction.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Action>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Action>();
    }
}
