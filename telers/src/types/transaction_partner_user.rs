use super::{PaidMedia, User};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a transaction with a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionPartnerUser {
    /// Information about the user
    pub user: User,
    /// Bot-specified invoice payload
    pub invoice_payload: Option<Box<str>>,
    /// Information about the paid media bought by the user
    pub paid_media: Option<Box<[PaidMedia]>>,
}
