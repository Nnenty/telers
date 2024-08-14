use serde::{Deserialize, Serialize};

/// The paid media isn't available before the payment.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediapreview>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender
    pub width: Option<i64>,
    /// Media height as defined by the sender
    pub height: Option<i64>,
    /// Duration of the media in seconds as defined by the sender
    pub duration: Option<i64>,
}
