use super::MessageEntity;

use serde::Serialize;

/// This object contains information about one answer option in a poll to send.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpolloption>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct InputPollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Currently, only custom emoji entities are allowed
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of `text_parse_mode`
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl<T> From<T> for InputPollOption
where
    T: Into<String>,
{
    fn from(val: T) -> Self {
        Self {
            text: val.into(),
            text_parse_mode: None,
            text_entities: None,
        }
    }
}
