//! Entities to interact with GitHub

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{id, name};

pub use self::private_key::PrivateKey;
pub use self::webhook_secret::WebhookSecret;

mod private_key;
mod webhook_secret;

id!(AppId);
name!(GitHubHost);

pub mod token;
