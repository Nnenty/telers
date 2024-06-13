use super::{
    BackgroundTypeChatTheme, BackgroundTypeFill, BackgroundTypePattern, BackgroundTypeWallpaper,
};

use serde::Deserialize;

/// This object describes the type of a background. Currently, it can be one of
/// - [`BackgroundTypeFill`]
/// - [`BackgroundTypeWallpaper`]
/// - [`BackgroundTypePattern`]
/// - [`BackgroundTypeChatTheme`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtype>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

impl From<BackgroundTypeFill> for BackgroundType {
    fn from(fill: BackgroundTypeFill) -> Self {
        Self::Fill(fill)
    }
}

impl From<BackgroundTypeWallpaper> for BackgroundType {
    fn from(wallpaper: BackgroundTypeWallpaper) -> Self {
        Self::Wallpaper(wallpaper)
    }
}

impl From<BackgroundTypePattern> for BackgroundType {
    fn from(pattern: BackgroundTypePattern) -> Self {
        Self::Pattern(pattern)
    }
}

impl From<BackgroundTypeChatTheme> for BackgroundType {
    fn from(chat_theme: BackgroundTypeChatTheme) -> Self {
        Self::ChatTheme(chat_theme)
    }
}
