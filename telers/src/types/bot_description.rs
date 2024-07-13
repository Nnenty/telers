use serde::{Deserialize, Serialize};

/// This object represents the bot's description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botdescription>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotDescription {
    /// The bot's description
    pub description: Box<str>,
}
