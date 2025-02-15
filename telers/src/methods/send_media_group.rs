use super::base::{prepare_input_media_group, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InputMedia, Message, ReplyParameters},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendmediagroup>
/// # Returns
/// On success, an array of [`Message`]s that were sent is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct SendMediaGroup<'a> {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMedia<'a>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
}

impl<'a> SendMediaGroup<'a> {
    #[must_use]
    pub fn new<T, I>(chat_id: impl Into<ChatIdKind>, media: I) -> Self
    where
        T: Into<InputMedia<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            media: media.into_iter().map(Into::into).collect(),
            disable_notification: None,
            protect_content: None,
            message_effect_id: None,
            reply_parameters: None,
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
    pub fn media_single(self, val: impl Into<InputMedia<'a>>) -> Self {
        Self {
            media: self.media.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn media<T, I>(self, val: I) -> Self
    where
        T: Into<InputMedia<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            media: self
                .media
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification(self, val: bool) -> Self {
        Self {
            disable_notification: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn protect_content(self, val: bool) -> Self {
        Self {
            protect_content: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn message_effect_id(self, val: impl Into<String>) -> Self {
        Self {
            message_effect_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn reply_parameters(self, val: ReplyParameters) -> Self {
        Self {
            reply_parameters: Some(val),
            ..self
        }
    }
}

impl<'a> SendMediaGroup<'a> {
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

    #[must_use]
    pub fn disable_notification_option(self, val: Option<bool>) -> Self {
        Self {
            disable_notification: val,
            ..self
        }
    }

    #[must_use]
    pub fn protect_content_option(self, val: Option<bool>) -> Self {
        Self {
            protect_content: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_effect_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            message_effect_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn reply_parameters_option(self, val: Option<ReplyParameters>) -> Self {
        Self {
            reply_parameters: val,
            ..self
        }
    }
}

impl<'a> TelegramMethod for SendMediaGroup<'a> {
    type Method = Self;
    type Return = Vec<Message>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_media_group(&mut files, &self.media);

        Request::new("sendMediaGroup", self, Some(files.into()))
    }
}

impl<'a> AsRef<SendMediaGroup<'a>> for SendMediaGroup<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
