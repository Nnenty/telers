use super::PhotoSize;

use serde::{Deserialize, Serialize};

/// The paid media is a photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediaphoto>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PaidMediaPhoto {
    /// The photo
    pub photo: Box<[PhotoSize]>,
}
