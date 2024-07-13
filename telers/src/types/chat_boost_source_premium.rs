use super::User;

use serde::{Deserialize, Serialize};

/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcepremium>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatBoostSourcePremium {
    /// User that boosted the chat
    pub user: User,
}
