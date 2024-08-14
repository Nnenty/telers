use super::{PaidMediaPhoto, PaidMediaPreview, PaidMediaVideo};

use serde::{Deserialize, Serialize};

/// This object describes paid media. Currently, it can be one of
/// - [`PaidMediaPreview`]
/// - [`PaidMediaPhoto`]
/// - [`PaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmedia>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}

impl From<PaidMediaPreview> for PaidMedia {
    fn from(fill: PaidMediaPreview) -> Self {
        Self::Preview(fill)
    }
}

impl From<PaidMediaPhoto> for PaidMedia {
    fn from(fill: PaidMediaPhoto) -> Self {
        Self::Photo(fill)
    }
}

impl From<PaidMediaVideo> for PaidMedia {
    fn from(fill: PaidMediaVideo) -> Self {
        Self::Video(fill)
    }
}
