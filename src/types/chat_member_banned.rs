use super::User;

use crate::enums::ChatMemberStatus;

use serde::Deserialize;

/// Represents a [`ChatMember`](crate::types::ChatMember) that was banned in the chat and can't return to the chat or view chat messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberbanned>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always 'kicked'
    #[serde(default = "kicked")]
    pub status: Box<str>,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    pub until_date: i64,
}

fn kicked() -> Box<str> {
    ChatMemberStatus::Kicked.into()
}
