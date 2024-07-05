use serde::{Deserialize, Serialize};

/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicreopened>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ForumTopicReopened {}
