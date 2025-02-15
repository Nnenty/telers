use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to stop updating a live location message before `live_period` expires.
/// # Documentation
/// <https://core.telegram.org/bots/api#stopmessagelivelocation>
/// # Returns
/// On success, if the message is not an inline message, the edited [`MessageOrTrue`] is returned,
/// otherwise `true` is returned
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct StopMessageLiveLocation {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message with live location to stop
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id(self, val: impl Into<String>) -> Self {
        Self {
            inline_message_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl StopMessageLiveLocation {
    #[must_use]
    pub fn business_connection_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            business_connection_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id_option(self, val: Option<impl Into<ChatIdKind>>) -> Self {
        Self {
            chat_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn message_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            inline_message_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for StopMessageLiveLocation {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("stopMessageLiveLocation", self, None)
    }
}

impl AsRef<StopMessageLiveLocation> for StopMessageLiveLocation {
    fn as_ref(&self) -> &Self {
        self
    }
}
