use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat ended in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatended>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}
