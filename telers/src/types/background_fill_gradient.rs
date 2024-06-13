use serde::Deserialize;

/// The background is a gradient fill
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfillgradient>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct BackgroundFillGradient {
    /// Top color of the gradient in the RGB24 format
    pub top_color: u32,
    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: u32,
    /// Clockwise rotation angle of the background fill in degrees; 0-359
    pub rotation_angle: i32,
}
