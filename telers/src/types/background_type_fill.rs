use super::BackgroundFill;

use serde::{Deserialize, Serialize};

/// The background is automatically filled based on the selected colors
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypefill>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackgroundTypeFill {
    /// The background fill
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: u8,
}
