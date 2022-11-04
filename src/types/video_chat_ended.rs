use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat ended in the chat.
/// <https://core.telegram.org/bots/api#videochatended>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}
