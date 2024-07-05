use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a service message about a new forum topic created in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicedited>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    pub name: Option<Box<str>>,
    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty `Box<str>` if the icon was removed
    pub icon_custom_emoji_id: Option<Box<str>>,
}
