use super::Location;

use serde::{Deserialize, Serialize};

/// Represents a location to which a chat is connected.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatlocation>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: Box<str>,
}
