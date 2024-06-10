use super::{BackgroundFillFreeformGradient, BackgroundFillGradient, BackgroundFillSolid};

use serde::Deserialize;

/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// - [`BackgroundFillSolid`]
/// - [`BackgroundFillGradient`]
/// - [`BackgroundFillFreeformGradient`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfill>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

impl From<BackgroundFillSolid> for BackgroundFill {
    fn from(fill: BackgroundFillSolid) -> Self {
        Self::Solid(fill)
    }
}

impl From<BackgroundFillGradient> for BackgroundFill {
    fn from(fill: BackgroundFillGradient) -> Self {
        Self::Gradient(fill)
    }
}

impl From<BackgroundFillFreeformGradient> for BackgroundFill {
    fn from(fill: BackgroundFillFreeformGradient) -> Self {
        Self::FreeformGradient(fill)
    }
}
