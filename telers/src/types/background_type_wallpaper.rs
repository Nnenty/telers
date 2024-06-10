use super::Document;

use serde::Deserialize;

/// The background is a wallpaper in the JPEG format
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypewallpaper>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct BackgroundTypeWallpaper {
    /// Document with the wallpaper
    pub document: Document,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: u8,
    /// `true`, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    pub is_blurred: Option<bool>,
    /// `true`, if the background moves slightly when the device is tilted
    pub is_moving: Option<bool>,
}
