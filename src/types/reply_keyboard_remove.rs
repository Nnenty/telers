use serde::{Deserialize, Serialize};

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see :class:`aiogram_rs.types.reply_keyboard_markup.ReplyKeyboardMarkup`).
/// <https://core.telegram.org/bots/api#replykeyboardremove>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use *one_time_keyboard* in :class:`aiogram_rs.types.reply_keyboard_markup.ReplyKeyboardMarkup`)
    pub remove_keyboard: bool,
    /// *Optional*. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the *text* of the :class:`aiogram_rs.types.message.Message` object; 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    pub selective: Option<bool>,
}

impl Default for ReplyKeyboardRemove {
    fn default() -> Self {
        Self {
            remove_keyboard: false,
            selective: None,
        }
    }
}
