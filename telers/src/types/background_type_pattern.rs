use super::{BackgroundFill, Document};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The background is a PNG or TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypepattern>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackgroundTypePattern {
    /// Document with the wallpaper
    pub document: Document,
    /// The background fill that is combined with the pattern
    pub fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: u8,
    /// `true`, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    pub is_inverted: Option<bool>,
    /// `true`, if the background moves slightly when the device is tilted
    pub is_moving: Option<bool>,
}
