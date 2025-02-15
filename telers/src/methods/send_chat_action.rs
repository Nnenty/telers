use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchataction>
/// # Notes
/// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
/// # Example
/// The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [`SendChatAction`](crate::methods::SendChatAction) with `action = upload_photo`. The user will see a “sending photo” status for the bot.
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SendChatAction {
    /// Unique identifier of the business connection on behalf of which the action will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread; supergroups only
    pub message_thread_id: Option<i64>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: `typing` for [`text messages`](crate::methods::SendMessage), `upload_photo` for [`photos`](crate::methods::SendPhoto), `record_video` or `upload_video` for [`videos`](crate::methods::SendVideo), `record_voice` or `upload_voice` for [`voice notes`](crate::methods::SendVoice), `upload_document` for [`general files`](crate::methods::SendDocument), `choose_sticker` for [`stickers`](crate::methods::SendSticker), `find_location` for [`location data`](crate::methods::SendLocation), `record_video_note` or `upload_video_note` for [`video notes`](crate::methods::SendVideoNote).
    pub action: String,
}

impl SendChatAction {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, action: impl Into<String>) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            action: action.into(),
        }
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
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn action(self, val: impl Into<String>) -> Self {
        Self {
            action: val.into(),
            ..self
        }
    }
}

impl SendChatAction {
    #[must_use]
    pub fn business_connection_id_option(self, val: Option<String>) -> Self {
        Self {
            business_connection_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }
}

impl TelegramMethod for SendChatAction {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendChatAction", self, None)
    }
}

impl AsRef<SendChatAction> for SendChatAction {
    fn as_ref(&self) -> &Self {
        self
    }
}
