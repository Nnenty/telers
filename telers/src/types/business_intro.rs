use super::Sticker;

use serde::Deserialize;

/// # Documentation
/// <https://core.telegram.org/bots/api#businessintro>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct BusinessIntro {
    /// Title text of the business intro
    pub title: Option<Box<str>>,
    /// Message text of the business intro
    pub message: Option<Box<str>>,
    /// Sticker of the business intro
    pub sticker: Option<Sticker>,
}
