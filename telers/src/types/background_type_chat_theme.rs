use serde::{Deserialize, Serialize};

/// The background is taken directly from a built-in chat theme.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypechattheme>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the chat theme, which is usually an emoji
    #[serde(rename = "theme_name")]
    pub name: String,
}
