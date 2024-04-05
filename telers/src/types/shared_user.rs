use super::PhotoSize;

use serde::Deserialize;

/// This object contains information about a user that was shared with the bot using a [`KeyboardButtonRequestUsers`](crate::types::KeyboardButtonRequestUsers) button.
/// # Documentation
/// <https://core.telegram.org/bots/api#shareduser>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct SharedUser {
    /// Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    #[serde(rename = "user_id")]
    pub id: i64,
    /// First name of the user, if the name was requested by the bot
    pub first_name: Option<Box<str>>,
    /// Last name of the user, if the name was requested by the bot
    pub last_name: Option<Box<str>>,
    /// Username of the user, if the username was requested by the bot
    pub username: Option<Box<str>>,
    /// Available sizes of the chat photo, if the photo was requested by the bot
    pub photo: Option<Box<[PhotoSize]>>,
}
