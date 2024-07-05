use super::PhotoSize;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object contains information about a chat that was shared with the bot using a [`KeyboardButtonRequestChat`](crate::types::KeyboardButtonRequestChat) button.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatshared>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: i64,
    /// Title of the chat, if the title was requested by the bot.
    pub title: Option<Box<str>>,
    /// Username of the chat, if the username was requested by the bot and available.
    pub username: Option<Box<str>>,
    /// Available sizes of the chat photo, if the photo was requested by the bot
    pub photo: Option<Box<[PhotoSize]>>,
}
