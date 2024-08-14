use super::InputFile;

use serde::Serialize;

/// The paid media to send is a video.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmediavideo>
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct InputPaidMediaVideo<'a> {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under `<file_attach_name>` name. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub media: InputFile<'a>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using `multipart/form-data` under `<file_attach_name>`. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumbnail: Option<InputFile<'a>>,
    /// Video width
    pub width: Option<i64>,
    /// Video height"
    pub height: Option<i64>,
    /// Video duration in seconds
    pub duration: Option<i64>,
    /// Pass `true` if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}

impl<'a> InputPaidMediaVideo<'a> {
    #[must_use]
    pub fn new(media: impl Into<InputFile<'a>>) -> Self {
        Self {
            media: media.into(),
            thumbnail: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
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
    pub fn thumbnail(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            thumbnail: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn width(self, val: i64) -> Self {
        Self {
            width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn height(self, val: i64) -> Self {
        Self {
            height: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn duration(self, val: i64) -> Self {
        Self {
            duration: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn supports_streaming(self, val: bool) -> Self {
        Self {
            supports_streaming: Some(val),
            ..self
        }
    }
}

impl<'a> InputPaidMediaVideo<'a> {
    #[must_use]
    pub fn thumbnail_option(self, val: Option<impl Into<InputFile<'a>>>) -> Self {
        Self {
            thumbnail: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn width_option(self, val: Option<i64>) -> Self {
        Self { width: val, ..self }
    }

    #[must_use]
    pub fn height_option(self, val: Option<i64>) -> Self {
        Self {
            height: val,
            ..self
        }
    }

    #[must_use]
    pub fn duration_option(self, val: Option<i64>) -> Self {
        Self {
            duration: val,
            ..self
        }
    }

    #[must_use]
    pub fn supports_streaming_option(self, val: Option<bool>) -> Self {
        Self {
            supports_streaming: val,
            ..self
        }
    }
}
