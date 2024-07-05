use super::BackgroundType;

use serde::{Deserialize, Serialize};

/// This object represents a chat background
/// # Documentation
/// <https://core.telegram.org/bots/api#chatbackground>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatBackground {
    /// Type of the background
    #[serde(rename = "type")]
    pub background_type: BackgroundType,
}
