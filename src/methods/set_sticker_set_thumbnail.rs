use super::base::{prepare_file, Request, TelegramMethod};

use crate::{client::Bot, types::InputFile};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Video thumbnails can be set only for video sticker sets only.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickersetthumbnail>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetStickerSetThumbnail<'a> {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A *PNG* image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a *TGS* animation with the thumbnail up to 32 kilobytes in size; see <https://core.telegram.org/stickers#animated-sticker-requirements> for animated sticker technical requirements, or a *WEBM* video with the thumbnail up to 32 kilobytes in size; see <https://core.telegram.org/stickers#video-sticker-requirements> for video sticker technical requirements. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [`More info on Sending Files`](https://core.telegram.org/bots/api#sending-files). Animated sticker set thumbnails can't be uploaded via HTTP URL.
    pub thumbnail: Option<InputFile<'a>>,
}

impl<'a> SetStickerSetThumbnail<'a> {
    #[must_use]
    pub fn new(name: impl Into<String>, user_id: i64) -> Self {
        Self {
            name: name.into(),
            user_id,
            thumbnail: None,
        }
    }

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn thumb(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            thumbnail: Some(val.into()),
            ..self
        }
    }
}

impl<'a> SetStickerSetThumbnail<'a> {
    #[must_use]
    pub fn thumbnail_option(self, val: Option<impl Into<InputFile<'a>>>) -> Self {
        Self {
            thumbnail: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetStickerSetThumbnail<'_> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        if let Some(thumb) = &self.thumbnail {
            prepare_file(&mut files, thumb);
        }

        Request::new("setStickerSetThumbnail", self, Some(files))
    }
}
