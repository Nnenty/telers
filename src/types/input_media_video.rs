use super::{InputMediaKind, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a video to be sent.
/// <https://core.telegram.org/bots/api#inputmediavideo>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InputMediaVideo {
    /// Type of the result, must be *video*
    #[serde(rename = "type", default = "video")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using multipart/form-data under <file_attach_name> name. :ref:`More information on Sending Files » <sending-files>`
    pub media: InputMediaKind,
    /// *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass 'attach://<file_attach_name>' if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. :ref:`More information on Sending Files » <sending-files>`
    pub thumb: Option<InputMediaKind>,
    /// *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the video caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Video width
    pub width: Option<i64>,
    /// *Optional*. Video height"
    pub height: Option<i64>,
    /// *Optional*. Video duration in seconds
    pub duration: Option<i64>,
    /// *Optional*. Pass :code:`True` if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}

impl Default for InputMediaVideo {
    fn default() -> Self {
        Self {
            media_type: video(),
            media: InputMediaKind::FileIdOrUrl(String::default()),
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }
}

fn video() -> String {
    "video".to_string()
}
