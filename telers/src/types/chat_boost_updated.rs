use super::{Chat, ChatBoostSource, Update, UpdateKind};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::{Deserialize, Serialize};

/// This object represents a boost added to a chat or changed.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostupdated>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Chat,
    /// Information about the chat boost
    pub boost: ChatBoostSource,
}

impl TryFrom<Update> for ChatBoostUpdated {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::ChatBoost(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "ChatBoostUpdated")),
        }
    }
}
