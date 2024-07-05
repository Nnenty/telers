use serde::{Deserialize, Serialize};

/// The background is a freeform gradient that rotates after every message in the chat
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfillfreeformgradient>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackgroundFillFreeformGradient {
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Box<[u32]>,
}
