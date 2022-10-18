use super::{Location, User};

use serde::{Deserialize, Serialize};

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
/// <https://core.telegram.org/bots/api#inlinequery>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// *Optional*. Type of the chat from which the inline query was sent. Can be either 'sender' for a private chat with the inline query sender, 'private', 'group', 'supergroup', or 'channel'. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    pub chat_type: Option<String>,
    /// *Optional*. Sender location, only for bots that request user location
    pub location: Option<Location>,
}

impl Default for InlineQuery {
    fn default() -> Self {
        Self {
            id: String::default(),
            from: User::default(),
            query: String::default(),
            offset: String::default(),
            chat_type: None,
            location: None,
        }
    }
}
