use super::{InputFile, MessageEntity};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Represents a photo to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediaphoto>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct InputMediaPhoto<'a> {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under `<file_attach_name>` name. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub media: InputFile<'a>,
    /// Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass `true`, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Pass `true` if the photo needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}

impl<'a> InputMediaPhoto<'a> {
    #[must_use]
    pub fn new(media: impl Into<InputFile<'a>>) -> Self {
        Self {
            media: media.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
        }
    }

    #[must_use]
    pub fn media(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            media: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn show_caption_above_media(self, val: bool) -> Self {
        Self {
            show_caption_above_media: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn has_spoiler(self, val: bool) -> Self {
        Self {
            has_spoiler: Some(val),
            ..self
        }
    }
}

impl<'a> InputMediaPhoto<'a> {
    #[must_use]
    pub fn caption_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            caption: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            caption_entities: val.map(|val| {
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn show_caption_above_media_option(self, val: Option<bool>) -> Self {
        Self {
            show_caption_above_media: val,
            ..self
        }
    }

    #[must_use]
    pub fn has_spoiler_option(self, val: Option<bool>) -> Self {
        Self {
            has_spoiler: val,
            ..self
        }
    }
}
