use serde::{Deserialize, Serialize};

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
/// <https://core.telegram.org/bots/api#passportelementerrorunspecified>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be *unspecified*
    #[serde(default = "unspecified")]
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}

impl Default for PassportElementErrorUnspecified {
    fn default() -> Self {
        Self {
            source: unspecified(),
            element_type: String::default(),
            element_hash: String::default(),
            message: String::default(),
        }
    }
}

fn unspecified() -> String {
    "unspecified".to_string()
}
