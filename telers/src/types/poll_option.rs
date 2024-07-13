use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object contains information about one answer option in a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#polloption>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: Box<str>,
    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Number of users that voted for this option
    pub voter_count: i64,
}
