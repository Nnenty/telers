use super::Sticker;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// # Documentation
/// <https://core.telegram.org/bots/api#businessintro>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct BusinessIntro {
    /// Title text of the business intro
    pub title: Option<Box<str>>,
    /// Message text of the business intro
    pub message: Option<Box<str>>,
    /// Sticker of the business intro
    pub sticker: Option<Sticker>,
}
