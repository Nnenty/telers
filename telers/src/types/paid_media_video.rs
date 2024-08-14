use super::Video;

use serde::{Deserialize, Serialize};

/// The paid media is a video.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediavideo>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PaidMediaVideo {
    /// The video
    pub video: Video,
}
