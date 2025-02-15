use super::PhotoSize;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a [`video message`](https://telegram.org/blog/video-messages-and-telescope) (available in Telegram apps as of [`v.4.0`](https://telegram.org/blog/video-messages-and-telescope)).
/// # Documentation
/// <https://core.telegram.org/bots/api#videonote>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Video thumbnail
    pub thumbnail: Option<PhotoSize>,
    /// File size in bytes
    pub file_size: Option<i64>,
}
