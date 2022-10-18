use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat scheduled in the chat.
/// <https://core.telegram.org/bots/api#videochatscheduled>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}

impl Default for VideoChatScheduled {
    fn default() -> Self {
        Self {
            start_date: 0,
        }
    }
}
