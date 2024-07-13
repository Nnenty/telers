use serde::{Deserialize, Serialize};

/// This object represents a unique message identifier.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageid>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageId {
    /// Unique message identifier
    #[serde(rename = "message_id")]
    pub id: i64,
}

impl From<MessageId> for i64 {
    fn from(val: MessageId) -> Self {
        val.id
    }
}
