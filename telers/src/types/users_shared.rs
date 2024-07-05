use super::SharedUser;

use serde::{Deserialize, Serialize};

/// This object contains information about the users whose identifiers were shared with the bot using a  [`KeyboardButtonRequestUsers`](crate::types::KeyboardButtonRequestUsers) button.
/// # Documentation
/// <https://core.telegram.org/bots/api#usersshared>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Information about users shared with the bot.
    pub users: Box<[SharedUser]>,
}
