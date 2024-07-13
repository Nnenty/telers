use serde::{Deserialize, Serialize};

/// This object represents a service message about a user boosting a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostadded>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: i64,
}
