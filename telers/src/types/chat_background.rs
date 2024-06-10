use super::BackgroundType;

use serde::Deserialize;

/// This object represents a chat background
/// # Documentation
/// <https://core.telegram.org/bots/api#chatbackground>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatBackground {
    /// Type of the background
    #[serde(rename = "type")]
    pub background_type: BackgroundType,
}
