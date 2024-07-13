use super::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a [`ChatMember`](crate::types::ChatMember) that owns the chat and has all administrator privileges.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberowner>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMemberOwner {
    /// Information about the user
    pub user: User,
    /// `true`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Custom title for this user
    pub custom_title: Option<Box<str>>,
}
