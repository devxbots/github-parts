use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::name;

pub use self::private_key::PrivateKey;
pub use self::webhook_secret::WebhookSecret;

mod private_key;
mod webhook_secret;

name!(GitHubHost);

pub mod app;
pub mod client;
pub mod token;
