use super::{Chat, Update, UpdateKind};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::{Deserialize, Serialize};

/// This object is received when messages are deleted from a connected business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessmessagesdeleted>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    pub chat: Chat,
    /// A JSON-serialized list of identifiers of deleted messages in the chat of the business account
    pub message_ids: Box<[i64]>,
}

impl TryFrom<Update> for BusinessMessagesDeleted {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::DeletedBusinessMessages(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "DeletedBusinessMessages")),
        }
    }
}
