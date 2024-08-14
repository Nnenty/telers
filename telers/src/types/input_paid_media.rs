use super::{InputPaidMediaPhoto, InputPaidMediaVideo};

use serde::Serialize;

/// This object describes the paid media to be sent. Currently, it can be one of
/// - [`InputPaidMediaPhoto`]
/// - [`InputPaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmedia>
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia<'a> {
    Photo(InputPaidMediaPhoto<'a>),
    Video(InputPaidMediaVideo<'a>),
}

impl<'a> From<InputPaidMediaPhoto<'a>> for InputPaidMedia<'a> {
    fn from(fill: InputPaidMediaPhoto<'a>) -> Self {
        Self::Photo(fill)
    }
}

impl<'a> From<InputPaidMediaVideo<'a>> for InputPaidMedia<'a> {
    fn from(fill: InputPaidMediaVideo<'a>) -> Self {
        Self::Video(fill)
    }
}
