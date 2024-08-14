use super::User;

use serde::{Deserialize, Serialize};

/// Represents a [`ChatMember`](crate::types::ChatMember) that has no additional privileges or restrictions.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmembermember>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMemberMember {
    /// Information about the user
    pub user: User,
    /// Date when the user's subscription will expire; Unix time
    pub until_date: Option<i64>,
}
