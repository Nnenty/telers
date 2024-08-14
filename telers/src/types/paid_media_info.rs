use super::PaidMedia;

use serde::{Deserialize, Serialize};

/// Describes the paid media added to a message.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediainfo>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: i64,
    /// Information about the paid media
    pub paid_media: Box<[PaidMedia]>,
}
