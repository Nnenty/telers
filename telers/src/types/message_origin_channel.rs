use super::Chat;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The message was originally sent to a channel chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginchannel>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginChannel {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Channel chat to which the message was originally sent
    pub chat: Chat,
    /// Unique message identifier inside the chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Signature of the original post author
    pub author_signature: Option<Box<str>>,
}
