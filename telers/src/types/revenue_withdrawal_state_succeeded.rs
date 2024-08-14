use serde::{Deserialize, Serialize};

/// The withdrawal succeeded.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatesucceeded>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Date the withdrawal was completed in Unix time
    pub date: i64,
    /// An HTTPS URL that can be used to see transaction details
    pub url: Box<str>,
}
