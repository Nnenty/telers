use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the voice message.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedvoice>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    /// Type of the result, must be *voice*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Recording title
    pub title: String,
    /// *Optional*. Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the voice message caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the voice recording
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedVoice {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, voice_file_id: T, title: T) -> Self {
        Self {
            id: id.into(),
            voice_file_id: voice_file_id.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn voice_file_id<T: Into<String>>(mut self, val: T) -> Self {
        self.voice_file_id = val.into();
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn caption<T: Into<String>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    #[must_use]
    pub fn parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption_entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}

impl Default for InlineQueryResultCachedVoice {
    fn default() -> Self {
        Self {
            result_type: "voice".to_string(),
            id: String::default(),
            voice_file_id: String::default(),
            title: String::default(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
