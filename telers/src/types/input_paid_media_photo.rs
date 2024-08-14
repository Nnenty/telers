use super::InputFile;

use serde::Serialize;

/// The paid media to send is a photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmediaphoto>
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct InputPaidMediaPhoto<'a> {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under `<file_attach_name>` name. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub media: InputFile<'a>,
}

impl<'a> InputPaidMediaPhoto<'a> {
    #[must_use]
    pub fn new(media: impl Into<InputFile<'a>>) -> Self {
        Self {
            media: media.into(),
        }
    }

    #[must_use]
    pub fn media(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            media: val.into(),
            ..self
        }
    }
}
